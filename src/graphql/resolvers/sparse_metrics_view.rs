#![allow(unused_braces)]

use crate::metrics::SparseMetricsView;

const NS_PER_MS: f64 = 1_000_000.0;

#[async_graphql::Object]
impl SparseMetricsView {
    /// Mean e2e response time (in ms)
    async fn mean(&self) -> f64 { self.mean / NS_PER_MS }

    /// Median e2e response time (in ms)
    async fn median(&self) -> f64 { self.median / NS_PER_MS }

    /// Mode e2e response time (in ms)
    async fn mode(&self) -> f64 { self.mode / NS_PER_MS }

    /// Minimum e2e response time (in ms)
    async fn min(&self) -> f64 { self.min / NS_PER_MS }

    /// Maximum e2e response time (in ms)
    async fn max(&self) -> f64 { self.max / NS_PER_MS }
    
    /// Standard Deviation of e2e response times (in ms)
    async fn std_dev(&self) -> f64 { self.std_dev / NS_PER_MS }

    /// Mean Absolute Deviation of e2e response times (in ms)
    async fn mad(&self) -> f64 { self.mad / NS_PER_MS }

    /// Percentile marks of e2e response times
    async fn percentiles(&self) -> Percentiles { Percentiles(self.percentiles) }
}



#[derive(Debug, Clone, Copy)]
pub struct Percentiles(pub [(u8, f64); 101]);

macro_rules! percentiles {
    ($($i:literal),+ $(,)?) => {
        paste::paste! {
            #[async_graphql::Object]
            impl Percentiles {
                /// A list of all percentiles of e2e response times (in ms)
                async fn arr(&self) -> Vec<f64> {
                    self.0.iter().map(|(_, v)| *v / NS_PER_MS).collect()
                }
                $(
                    #[doc = "Percentile " $i " of e2e response times (in ms)"]
                    async fn [<p $i>](&self) -> f64 {
                        self.0[$i].1 / NS_PER_MS
                    }
                )+
            }
        }
    };
}

percentiles!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
    60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
    70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
    90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
    100,
);
