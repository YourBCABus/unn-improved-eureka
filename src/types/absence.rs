use std::sync::Arc;

use chrono::NaiveDate;
use uuid::Uuid;

use super::Period;

#[derive(Debug, Clone, Copy)]
pub struct Absence {
    pub teacher: Uuid,
    pub period: Uuid,
}

#[derive(Debug, Clone)]
pub struct PackedAbsenceState {
    pub (crate) teacher_id: Uuid,
    pub (crate) date: NaiveDate,
    pub (crate) fully: bool,
    pub (crate) periods: Vec<Arc<Period>>,
    pub (crate) comments: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TeacherAbsenceStateList(pub Uuid, pub Vec<PackedAbsenceState>);
