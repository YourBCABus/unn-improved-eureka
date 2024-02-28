#![allow(unused_braces)]

use async_graphql::{Lookahead, Result as GraphQlResult};
use crate::metrics::{Buckets, SparseMetricsView};

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

    /// Buckets (min, max, and step) of response time
    #[graphql(complexity = "((end - start) / step) as usize")]
    async fn buckets(&self, start: f64, end: f64, step: f64) -> GraphQlResult<&Buckets> {
        if start != self.buckets.range.start || end != self.buckets.range.end || step != self.buckets.step {
            Err(format!(
                "Invalid bucket parameters — expected: ({}, {}, {}), got: ({}, {}, {})",
                self.buckets.range.start, self.buckets.range.end, self.buckets.step,
                start, end, step,
            ).into())
        } else {
            Ok(&self.buckets)
        }
    }
}

pub fn find_buckets_params_from_lookahead(lookahead: Lookahead<'_>) -> Option<(f64, std::ops::Range<f64>)> {
    use std::collections::HashMap;
    use async_graphql_value::Value as GraphQlValue;

    let buckets_params = lookahead
        .selection_fields()
        .into_iter()
        .find(|f| f.name() == "getMetrics")?
        .selection_set()
        .find(|f| f.name() == "buckets")?
        .arguments()
        .ok()?;

    let buckets_hashmap: HashMap<_, _> = buckets_params.into_iter().collect();
    let (start, end, step) = (
        buckets_hashmap.get("start").map(|arg| arg.clone().into_value())?,
        buckets_hashmap.get("end").map(|arg| arg.clone().into_value())?,
        buckets_hashmap.get("step").map(|arg| arg.clone().into_value())?,
    );

    let (start, end, step) = (
        match start { GraphQlValue::Number(n) => n.as_f64()?, _ => return None },
        match end { GraphQlValue::Number(n) => n.as_f64()?, _ => return None },
        match step { GraphQlValue::Number(n) => n.as_f64()?, _ => return None },
    );
    Some((step, start..end))
}

const MAX_BUCKET_COUNT: f64 = 100.0;



pub fn buckets_valid(range: std::ops::Range<f64>, step: f64) -> GraphQlResult<()> {
    fn remainder_diff(range: f64, step: f64) -> f64 {
        (range.rem_euclid(step) + step / 2.0).rem_euclid(step) - step / 2.0
    }

    if range.start >= range.end {
        Err(async_graphql::Error::new("start >= end"))
    } else if step <= 0.0 {
        Err(async_graphql::Error::new("step <= 0"))
    } else if remainder_diff(range.end - range.start, step).abs() >= std::f64::EPSILON {
        Err(async_graphql::Error::new("the range is not divisible by step"))
    } else if ((range.end - range.start) / step).round() > MAX_BUCKET_COUNT {
        Err(async_graphql::Error::new("too many buckets"))
    } else {
        Ok(())
    }
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



#[async_graphql::Object]
impl Buckets {
    async fn values(&self) -> &[f64] { &self.values }

    async fn with_max_of(&self, max: f64) -> Buckets {
        self.rescale_with_max_of(max)
    }

    async fn as_entries(&self) -> Vec<BucketEntry> {
        self.entries_iter()
            .map(|(value, range)| BucketEntry { value, min: range.start, max: range.end })
            .collect()
    }

    async fn graph(&self, max_width: usize) -> String {
        let rescaled = self.rescale_with_max_of(max_width as f64);

        let mut graph = String::new();
        for entry in rescaled.values_slice() {
            let width = *entry as usize;
            for _ in 0..width {
                graph.push('#');
            }
            graph.push('\n');
        }

        graph
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct BucketEntry {
    value: f64,
    min: f64,
    max: f64,
}
