#![allow(unused_braces)]

use async_graphql::Object;

use std::sync::Arc;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::types::Period;
use crate::types::PackedAbsenceState;
use crate::types::TeacherAbsenceStateList;


#[Object]
impl PackedAbsenceState {
    async fn teacher_id(&self) -> Uuid {
        self.teacher_id
    }

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

#[Object]
impl TeacherAbsenceStateList {
    async fn id(&self) -> Uuid {
        self.0
    }

    async fn absences(&self) -> &[PackedAbsenceState] {
        &self.1
    }
}

