mod data;
mod r#trait;
pub mod middleware;

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use std::time::Duration;
use tokio::sync::mpsc::{ channel, error::SendError, Receiver, Sender };
use tokio::sync::oneshot::{ Sender as OneshotSender, channel as oneshot_channel };

use self::data::ResponseTimeMap;
use self::r#trait::Metrics;

#[derive(Debug)]
pub struct ResponseTimeMetrics {
    rtm: ResponseTimeMap,
    reciever: Receiver<Duration>,
    command_reciever: Receiver<(MetricsCommand, OneshotSender<bool>)>,
    sender: Sender<Duration>,
    command_sender: Sender<(MetricsCommand, OneshotSender<bool>)>,
}

impl Metrics for ResponseTimeMetrics {
    fn mean(&self) -> f64 { self.rtm.mean() }
    fn mean_without_outliers(&self, mads: f64) -> f64 { self.rtm.mean_without_outliers(mads) }
    fn percentile(&self, p: f64) -> f64 { self.rtm.percentile(p) }
    fn median(&self) -> f64 { self.rtm.median() }
    fn mode(&self) -> f64 { self.rtm.mode() }
    fn min(&self) -> f64 { self.rtm.min() }
    fn max(&self) -> f64 { self.rtm.max() }
    fn mad(&self) -> f64 { self.rtm.mad() }
    fn std_dev(&self) -> f64 { self.rtm.std_dev() }
}


impl ResponseTimeMetrics {
    pub fn sender(&self) -> MetricProducer {
        MetricProducer(self.sender.clone(), self.command_sender.clone())
    }

    pub fn spawn(mut self) {
        tokio::spawn(async move {
            self.start().await;
        });
    }

    pub async fn start(&mut self) {
        let Self { rtm, reciever, command_reciever, .. } = self;

        let working = AtomicBool::new(true);

        while working.load(std::sync::atomic::Ordering::Acquire) {
            tokio::select! {
                duration = reciever.recv() => {
                    if let Some(duration) = duration {
                        Self::record_duration(rtm, duration)
                    } else {
                        working.store(false, std::sync::atomic::Ordering::Release);
                    }
                },
                command = command_reciever.recv() => {
                    if let Some((cmd, acknowledgement)) = command {
                        Self::handle_command(rtm, cmd, acknowledgement);
                    } else {
                        working.store(false, std::sync::atomic::Ordering::Release);
                    }
                },
            }
        }
    }
    fn record_duration(rtm: &mut ResponseTimeMap, duration: Duration) {
        let nanos = duration
            .clamp(
                Duration::from_secs(0),
                Duration::from_secs(5 * 60),
            )
            .as_nanos() as u64;
        rtm.record_nanos(nanos);
    }
    fn handle_command(rtm: &mut ResponseTimeMap, cmd: MetricsCommand, ack: OneshotSender<bool>) {
        match cmd {
            MetricsCommand::SaveTo(path) => {
                let output = rtm.output();
                tokio::spawn(async move {
                    let res = tokio::fs::write(
                        path,
                        output,
                    ).await;
                    let _ = ack.send(res.is_ok());
                });
            }
            MetricsCommand::Clear => {
                rtm.clear();
                let _ = ack.send(true);
            }
        }
    }
}



impl Default for ResponseTimeMetrics {
    fn default() -> Self {
        let (sender, reciever) = channel(64);
        let (command_sender, command_reciever) = channel(4);

        Self {
            rtm: ResponseTimeMap::new(),
            reciever,
            sender,
            command_reciever,
            command_sender,
        }
    }
}


#[derive(Debug, Clone)]
pub struct MetricProducer(Sender<Duration>, Sender<(MetricsCommand, OneshotSender<bool>)>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetricsCommand {
    SaveTo(PathBuf),
    Clear,
}

impl MetricProducer {
    pub fn record(&self, duration: Duration) {
        let sender = self.0.clone();

        tokio::spawn(async move {
            sender.send(duration).await
        });
    }
    async fn send(&self, command: MetricsCommand, timeout: Duration) -> Result<(), SendError<MetricsCommand>> {
        let (sender, reciever) = oneshot_channel();

        self.1
            .send((command.clone(), sender))
            .await
            .map_err(|SendError((command, _))| SendError(command))?;

        tokio::select! {
            res = reciever => {
                if let Ok(success) = res {
                    if success {
                        Ok(())
                    } else {
                        Err(SendError(command))
                    }
                } else {
                    Err(SendError(command))
                }
            },
            _ = tokio::time::sleep(timeout) => Err(SendError(command)),
        }
    }

    pub async fn save_to(&self, path: PathBuf, timeout: Option<Duration>) -> Result<(), SendError<MetricsCommand>> {
        self.send(MetricsCommand::SaveTo(path), timeout.unwrap_or(Duration::from_secs(10))).await
    }

    pub async fn read(&self, timeout: Option<Duration>) -> Result<String, SendError<MetricsCommand>> {
        let file_name = format!("metrics-{}.metrics", chrono::Utc::now().to_rfc3339());
        let path = PathBuf::from(file_name);
        self.save_to(path.clone(), timeout).await?;

        let output = tokio::fs::read_to_string(path.clone())
            .await
            .map_err(|_| SendError(MetricsCommand::SaveTo(path.clone())))?;

        tokio::fs::remove_file(path.clone()).await
            .map_err(|_| SendError(MetricsCommand::SaveTo(path)))?;

        Ok(output)
    }

    pub async fn clear(&self, timeout: Option<Duration>) -> Result<(), SendError<MetricsCommand>> {
        self.send(MetricsCommand::Clear, timeout.unwrap_or(Duration::from_secs(10))).await
    }
}
