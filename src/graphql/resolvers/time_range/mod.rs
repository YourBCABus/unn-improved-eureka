
use std::fmt::Display;

use juniper::graphql_object;

use crate::graphql_types::{Context, inputs::TimeRangeInput};
use chrono::{NaiveTime, Timelike};



#[derive(Debug, Clone, Copy)]
pub struct TimeRange {
    start: NaiveTime,
    end: NaiveTime,
}

impl TimeRange {
    pub fn new(start: NaiveTime, end: NaiveTime) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> NaiveTime { self.start }
    pub fn end(&self) -> NaiveTime { self.end }
}

impl TryFrom<TimeRangeInput> for TimeRange {
    type Error = (f64, f64);
    fn try_from(value: TimeRangeInput) -> Result<Self, Self::Error> {
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



#[graphql_object(
    context = Context,
    name = "TimeRange",
)]
impl TimeRange {
    fn start(&self) -> f64 {
        self.start.num_seconds_from_midnight() as f64
    }
    fn end(&self) -> f64 {
        self.end.num_seconds_from_midnight() as f64
    }
}

