#![allow(unused_braces)]

use std::sync::Arc;

use async_graphql::Object;
use chrono::NaiveDate;
use crate::types::Period;
use crate::types::PackedAbsenceState;


#[Object]
impl PackedAbsenceState {
    async fn full(&self) -> bool {
        self.fully
    }
    async fn fully(&self) -> bool {
        self.fully
    }
    async fn fully_absent(&self) -> bool {
        self.fully
    }

    async fn periods(&self) -> &[Arc<Period>] {
        &self.periods
    }

    async fn date(&self) -> NaiveDate {
        self.date
    }

    async fn comments(&self) -> Option<&str> {
        self.comments.as_deref()
    }
}
