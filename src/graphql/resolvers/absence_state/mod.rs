//! This module contains solely the [QueryRoot] struct.
//! It only exists for organizational purposes.

use std::fmt::Display;

use juniper::graphql_object;

use crate::graphql_types::{
    scalars::teacher::*,
    juniper_types::IntoFieldError,
    *,
};
use crate::database::prelude::TeacherPresence;

use super::period::PeriodMetadata;

mod absent_periods;


/// This struct represents a Teacher with no period information associated with the absence_state.
/// 
/// This is mainly used to be an graphql-compatible type for [TeacherRow],
/// and usually should be created with [From] or [Into].
/// 
/// [juniper] will apply resolvers to it to get the required fields.
#[derive(Debug, Clone)]
pub struct AbsenceStateMetadata {
    /// The id of the teacher. This is essentially a wrapper for a UUID, but is (de)serializable for juniper.
    teacher_id: TeacherId,
    /// The stripped absence state of the teacher.
    absence_state_meta: TeacherPresence,
}

impl AbsenceStateMetadata {
    pub fn from_id_and_meta(id: TeacherId, meta: TeacherPresence) -> Self {
        Self { teacher_id: id, absence_state_meta: meta }
    }
}

impl Display for AbsenceStateMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AbsenceStateMetadata<{}> (teach_id: {})", self.absence_state_meta.to_sql_type(), self.teacher_id.id_str())
    }
}



#[graphql_object(
    context = Context,
    name = "AbsenceState",
    description = "This type represents the parts of a day in which a teacher is absent.",
)]
impl AbsenceStateMetadata {
    async fn absent_periods(
        &self,
        ctx: &Context,
    ) -> juniper::FieldResult<Option<Vec<PeriodMetadata>>> {
        let db_context = ctx.get_db_mut().await;

        absent_periods::absent_periods(self, &db_context.client)
            .await
            .map_err(IntoFieldError::into_field_error)

    }

    fn is_absent(&self) -> bool {
        matches!(self.absence_state_meta, TeacherPresence::PartAbsent | TeacherPresence::FullAbsent)
    }
    fn is_fully_absent(&self) -> bool {
        matches!(self.absence_state_meta, TeacherPresence::FullAbsent)
    }
}