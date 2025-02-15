
use std::fmt::Display;

use async_graphql::Object;

use crate::types::TimeRange;

impl Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TimeRange<{}-{}>", self.start.time(), self.end.time())
    }
}

#[Object]
impl TimeRange {
    async fn start(&self) -> f64 {
        self.start.seconds()
    }
    async fn end(&self) -> f64 {
        self.end.seconds()
    }
}

