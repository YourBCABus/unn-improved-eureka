use super::data::{ResponseTime, ResponseTimeMap};

pub trait Metrics {
    fn mean(&self) -> f64;

    fn median(&self) -> f64;
    fn mode(&self) -> f64;
    fn min(&self) -> f64;
    fn max(&self) -> f64;
    fn percentile(&self, p: f64) -> f64;

    fn mad(&self) -> f64;
    fn std_dev(&self) -> f64;

    fn buckets(&self, range_ms: std::ops::Range<f64>, step: f64) -> Buckets;

    fn output(&self) -> String {
        use std::fmt::Write;

        let mut output = String::new();
        let _ = writeln!(&mut output, "Mean:    {:.4}ms", self.mean() / 1_000_000.0);
        let _ = writeln!(&mut output, "Median:  {:.4}ms", self.median() / 1_000_000.0);
        let _ = writeln!(&mut output, "Mode:    {:.4}ms", self.mode() / 1_000_000.0);
        let _ = writeln!(&mut output, "Min:     {:.4}ms", self.min() / 1_000_000.0);
        let _ = writeln!(&mut output, "Max:     {:.4}ms", self.max() / 1_000_000.0);
        let _ = writeln!(&mut output, "MAD:     {:.4}ms", self.mad() / 1_000_000.0);
        let _ = writeln!(&mut output, "Std Dev: {:.4}ms", self.std_dev() / 1_000_000.0);

        let _ = writeln!(&mut output);
        let _ = writeln!(&mut output, "Percentiles:");
        for p in (0..=100).map(|p| p as f64 / 100.0) {
            let _ = writeln!(&mut output, "  {:.0}%: {:.4}ms", p * 100.0, self.percentile(p) / 1_000_000.0);
        }

        output
    }
}

impl Metrics for ResponseTimeMap {
    fn mean(&self) -> f64 {
        let sum: f64 = self.iter().map(|(k, v)| k.as_nanos() as f64 * v as f64).sum();
        let count: f64 = self.recorded() as f64;
        sum / count
    }

    fn buckets(&self, range_ms: std::ops::Range<f64>, step: f64) -> Buckets {
        let mut values = vec![0.0; ((range_ms.end - range_ms.start) / step).ceil() as usize];

        for (k, v) in self.iter() {
            let ms = k.as_nanos() as f64 / 1_000_000.0;
            if ms < range_ms.start || ms >= range_ms.end {
                continue;
            }
            let index = ((ms - range_ms.start) / step) as usize;

            if index < values.len() {
                values[index] += v as f64;
            }
        }

        Buckets {
            values,
            range: range_ms,
            step,
        }
    }

    fn percentile(&self, p: f64) -> f64 {
        let p = p.clamp(0.0, 1.0);

        let mut sorted: Vec<_> = self.iter().collect();
        sorted.sort_by(|(a, _), (b, _)| a.cmp(b));

        let index = if p == 1.0 {
            self.recorded().saturating_sub(1)
        } else {
            (p * (self.recorded()) as f64) as u64
        };

        let mut left = index;
        for (time, count) in sorted {
            if count > left {
                return time.as_nanos() as f64;
            }
            left -= count;
        }

        f64::NAN
    }

    fn median(&self) -> f64 {
        self.percentile(0.5)
    }
    fn mode(&self) -> f64 {
        let mut max = (ResponseTime::from_nanos(0), 0);
        for (k, v) in self.iter() {
            if v > max.1 {
                max = (k, v);
            }
        }
        max.0.as_nanos() as f64
    }
    fn min(&self) -> f64 {
        self
            .iter()
            .map(|(k, _)| k.as_nanos() as f64)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(f64::INFINITY)
    }
    fn max(&self) -> f64 {
        self
            .iter()
            .map(|(k, _)| k.as_nanos() as f64)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }
    fn mad(&self) -> f64 {
        let median = self.median();
        let mad: f64 = self.iter().map(|(k, v)| (v as f64) * (k.as_nanos() as f64 - median).abs()).sum();
        mad / self.recorded() as f64
    }
    fn std_dev(&self) -> f64 {
        let mean = self.mean();
        let sum: f64 = self.iter().map(|(k, v)| (v as f64) * (k.as_nanos() as f64 - mean).powi(2)).sum();
        (sum / self.recorded() as f64).sqrt()
    }
}


#[derive(Debug, Clone)]
pub struct Buckets {
    pub values: Vec<f64>,
    pub range: std::ops::Range<f64>,
    pub step: f64,
}

impl Buckets {
    pub fn values_slice(&self) -> &[f64] { &self.values }

    pub fn rescale_with_max_of(&self, max: f64) -> Self {
        let values_max = self.values.iter().copied().max_by(|a, b| a.total_cmp(b)).unwrap_or(1.0);
        let multiplier = max / values_max;
        let new_values = self.values.iter().map(|&v| v * multiplier).collect();

        Buckets { values: new_values, range: self.range.clone(), step: self.step }
    }

    pub fn entries_iter(&self) -> impl Iterator<Item = (f64, std::ops::Range<f64>)> + '_ {
        self.values
            .iter().copied()
            .enumerate()
            .map(|(ids, val)| {
                let start = self.range.start + (self.step * ids as f64);
                let end = start + self.step;
                (val, start..end)
            })
    }
}

#[derive(Debug, Clone)]
pub struct SparseMetricsView {
    pub mean: f64,
    pub median: f64,
    pub mode: f64,
    pub min: f64,
    pub max: f64,
    pub mad: f64,
    pub std_dev: f64,

    pub percentiles: [(u8, f64); 101],
    pub buckets: Buckets,
}

impl SparseMetricsView {
    pub fn zero() -> Self {
        Self {
            mean: 0.0,
            median: 0.0,
            mode: 0.0,
            min: 0.0,
            max: 0.0,
            mad: 0.0,
            std_dev: 0.0,
            percentiles: std::array::from_fn(|i| (i as u8, 0.0)),
            buckets: Buckets {
                values: vec![0.0; 1],
                range: 0.0..1.0,
                step: 1.0,
            },
        }
    }

    pub fn from_metrics(metrics_object: &impl Metrics, range: std::ops::Range<f64>, step: f64) -> Self {
        Self {
            mean: metrics_object.mean(),
            median: metrics_object.median(),
            mode: metrics_object.mode(),
            min: metrics_object.min(),
            max: metrics_object.max(),
            mad: metrics_object.mad(),
            std_dev: metrics_object.std_dev(),
            percentiles: std::array::from_fn(
                |percentile| (
                    percentile as u8,
                    metrics_object.percentile(percentile as f64 / 100.0),
                )
            ),
            buckets: metrics_object.buckets(range, step),
        }
    }
}
