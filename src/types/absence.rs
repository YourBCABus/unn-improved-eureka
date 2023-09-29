use std::sync::Arc;

use chrono::NaiveDate;
use uuid::Uuid;

use super::Period;

#[derive(Debug, Clone, Copy)]
pub struct Absence {
    pub teacher: Uuid,
    pub period: Uuid,
}

pub struct PackedAbsenceState {
    pub (crate) date: NaiveDate,
    pub (crate) fully: bool,
    pub (crate) periods: Vec<Arc<Period>>,
    pub (crate) comments: Option<String>,
}
