use super::data::{ResponseTime, ResponseTimeMap};

pub trait Metrics {
    fn mean(&self) -> f64;
    
    fn mean_without_outliers(&self, mads: f64) -> f64;

    fn median(&self) -> f64;
    fn mode(&self) -> f64;
    fn min(&self) -> f64;
    fn max(&self) -> f64;
    fn percentile(&self, p: f64) -> f64;

    fn mad(&self) -> f64;
    fn std_dev(&self) -> f64;

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
    fn mean_without_outliers(&self, mads: f64) -> f64 {
        let mad = self.mad();
        let median = self.median();


        let nonoutliers = self.iter()
            .filter(|(k, v)|{
                let diff = k.as_nanos() as f64 - median;
                diff.abs() < mad * mads
            });
        
        let (sum, count) = nonoutliers
            .fold((0.0, 0.0), |(total, count), (k, v)| {
                (
                    total + k.as_nanos() as f64 * v as f64,
                    count + v as f64,
                )
            });
        
        sum / count
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
