
use std::fmt::Display;

use async_graphql::Object;

use chrono::{NaiveTime, Timelike};

use crate::graphql::structs::TimeRangeInput;



#[derive(Debug, Clone, Copy)]
pub struct TimeRange {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl TimeRange {
    pub fn new(start: NaiveTime, end: NaiveTime) -> Self {
        Self { start, end }
    }
}

impl TryFrom<TimeRangeInput> for TimeRange {
    type Error = (f64, f64);
    fn try_from(value: TimeRangeInput) -> Result<Self, Self::Error> {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let (start, end) = (
            value.start.rem_euclid(24.0 * 60.0 * 60.0).floor() as u32,
            value.end.rem_euclid(24.0 * 60.0 * 60.0).floor() as u32,
        );
        let (start, end) = (
            NaiveTime::from_num_seconds_from_midnight_opt(start, 0),
            NaiveTime::from_num_seconds_from_midnight_opt(end, 0),
        );

        if let (Some(start), Some(end)) = (start, end) {
            Ok(Self { start, end })
        } else {
            Err((value.start, value.end))
        }
    }
}

impl Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TimeRange<{}-{}>", self.start, self.end)
    }
}

impl From<(f64, f64)> for TimeRange {
    fn from(value: (f64, f64)) -> Self {
        let ns_per_s = 1_000_000_000.0;
        let start = NaiveTime::from_num_seconds_from_midnight_opt(
            value.0 as u32,
            ((value.0 % 1.0) * ns_per_s) as u32,
        ).unwrap_or_default();
        let end = NaiveTime::from_num_seconds_from_midnight_opt(
            value.1 as u32,
            ((value.1 % 1.0) * ns_per_s) as u32,
        ).unwrap_or_default();

        Self { start, end }
    }
}



// #[allow(clippy::)]
#[Object]
impl TimeRange {
    async fn start(&self) -> f64 {
        let seconds = f64::from(self.start.num_seconds_from_midnight());
        let nanoseconds = f64::from(self.start.nanosecond()) / 1_000_000_000.0;
        seconds + nanoseconds
    }
    async fn end(&self) -> f64 {
        let seconds = f64::from(self.end.num_seconds_from_midnight());
        let nanoseconds = f64::from(self.end.nanosecond()) / 1_000_000_000.0;
        seconds + nanoseconds
    }
}

