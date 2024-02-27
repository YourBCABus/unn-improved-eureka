mod data;
mod r#trait;
pub mod middleware;

use std::sync::atomic::AtomicBool;

use std::time::Duration;
use tokio::sync::mpsc::{ channel, error::SendError, Receiver, Sender };
use tokio::sync::oneshot::{ Sender as OneshotSender, channel as oneshot_channel };

use self::data::ResponseTimeMap;
use self::r#trait::Metrics;
pub use self::r#trait::SparseMetricsView;

#[derive(Debug)]
pub struct SingleResponseMetricsCommand {
    pub command: MetricsCommand,
    pub responder: OneshotSender<Box<SparseMetricsView>>,
}

#[derive(Debug)]
pub struct ResponseTimeMetrics {
    rtm: ResponseTimeMap,
    reciever: Receiver<Duration>,
    command_reciever: Receiver<SingleResponseMetricsCommand>,
    sender: Sender<Duration>,
    command_sender: Sender<SingleResponseMetricsCommand>,
}

impl Metrics for ResponseTimeMetrics {
    fn mean(&self) -> f64 { self.rtm.mean() }
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

    pub fn spawn(self) {
        tokio::spawn(self.start());
    }

    pub async fn start(self) {
        let Self { mut rtm, mut reciever, mut command_reciever, .. } = self;

        let working = AtomicBool::new(true);

        while working.load(std::sync::atomic::Ordering::Acquire) {
            tokio::select! {
                duration = reciever.recv() => {
                    if let Some(duration) = duration {
                        Self::record_duration(&mut rtm, duration)
                    } else {
                        working.store(false, std::sync::atomic::Ordering::Release);
                    }
                },
                command = command_reciever.recv() => {
                    if let Some(command) = command {
                        Self::handle_command(&mut rtm, command.command, command.responder);
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
    fn handle_command(
        rtm: &mut ResponseTimeMap,
        cmd: MetricsCommand,
        responder: OneshotSender<Box<SparseMetricsView>>,
    ) {
        match cmd {
            MetricsCommand::Read => {
                let output = Box::new(SparseMetricsView::from_metrics(rtm));
                
                if let Err(e) = responder.send(output) {
                    crate::logging::error!("Failed to send response to metrics command: {:?}", e);
                    crate::logging::info!("Timeout will likely trigger...");
                }
            }
            MetricsCommand::Clear => {
                rtm.clear();
                if let Err(e) = responder.send(Box::new(SparseMetricsView::zero())) {
                    crate::logging::error!("Failed to send response to metrics command: {:?}", e);
                    crate::logging::info!("Timeout will likely trigger...");
                }
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
pub struct MetricProducer(Sender<Duration>, Sender<SingleResponseMetricsCommand>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricsCommand {
    Read,
    Clear,
}

impl MetricProducer {
    pub fn record(&self, duration: Duration) {
        let sender = self.0.clone();

        tokio::spawn(async move {
            sender.send(duration).await
        });
    }
    async fn send(&self, command: MetricsCommand, timeout: Duration) -> Result<SparseMetricsView, SendError<MetricsCommand>> {
        let (sender, reciever) = oneshot_channel();

        let single_response_command = SingleResponseMetricsCommand {
            command,
            responder: sender,
        };

        self.1
            .send(single_response_command)
            .await
            .map_err(|SendError(single_response_command)| SendError(single_response_command.command))?;

        tokio::select! {
            res = reciever => {
                match res {
                    Ok(view) => {
                        crate::logging::debug!("Successfully recieved metrics view: {view:?}");
                        Ok(*view)
                    },
                    Err(e) => {
                        crate::logging::debug!("Failed to recieve metrics view - recieve failed: {e:?}");
                        Err(SendError(command))
                    },
                }
            },
            _ = tokio::time::sleep(timeout) => {
                crate::logging::debug!("Failed to recieve metrics view - {:.4} second timeout expired", timeout.as_secs_f64());
                Err(SendError(command))
            },
        }
    }

    pub async fn read(&self, timeout: Option<Duration>) -> Result<SparseMetricsView, SendError<MetricsCommand>> {
        self.send(MetricsCommand::Read, timeout.unwrap_or(Duration::from_secs(10))).await
    }

    pub async fn clear(&self, timeout: Option<Duration>) -> Result<(), SendError<MetricsCommand>> {
        self.send(MetricsCommand::Clear, timeout.unwrap_or(Duration::from_secs(10))).await?;
        Ok(())
    }
}
