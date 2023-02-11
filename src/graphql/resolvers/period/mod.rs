
use std::fmt::Display;

use super::teacher::TeacherMetadata;
use super::time_range::TimeRange;
use juniper::{graphql_object, IntoFieldError, FieldResult};

use crate::graphql_types::{
    scalars::period::*,
    *,
};
use crate::utils::structs::PeriodRow;

pub mod teachers_absent;

/// This struct represents a Period with no teacher information associated with the absence_state.
/// 
/// This is mainly used to be an graphql-compatible type for [TeacherRow],
/// and usually should be created with [From] or [Into].
/// 
/// [juniper] will apply resolvers to it to get the required fields.
#[derive(Debug, Clone)]
pub struct PeriodMetadata {
    /// The id of the period. This is essentially a wrapper for a UUID, but is (de)serializable for juniper.
    pub id: PeriodId,
    /// The name of the period. A String wrapper.
    pub name: PeriodName,

    pub default_range: TimeRange,
    pub temp_range: Option<TimeRange>,
}

impl From<PeriodRow> for PeriodMetadata {
    fn from(row: PeriodRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            default_range: TimeRange::new(row.start_default, row.end_default),
            temp_range: if let (Some(start), Some(end)) = (row.start_curr, row.end_curr) {
                Some(TimeRange::new(start, end))
            } else { None }
        }
    }
}

type TryFromError = <PeriodRow as TryFrom<tokio_postgres::Row>>::Error;
impl PeriodMetadata {
    pub fn try_from_row<T>(row: tokio_postgres::Row, err_map: impl FnOnce(TryFromError) -> T) -> Result<Self, T> {
        PeriodRow::try_from(row).map(Into::into).map_err(err_map)
    }
}

impl Display for PeriodMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PeriodMetadata<{}> ({})", self.name.name_str(), self.id.id_str())
    }
}


#[graphql_object(
    context = Context,
    name = "Period",
    description = "This type represents the a specific period of the day.",
)]
impl PeriodMetadata {
    async fn teachers_absent(
        &self,
        ctx: &Context,
    ) -> FieldResult<Vec<TeacherMetadata>> {
        let db_context = ctx.get_db_mut().await;

        teachers_absent::absent_periods(&self.id.uuid(), &db_context.client)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    fn default_time_range(&self) -> &TimeRange {
        &self.default_range
    }

    fn time_range(&self) -> &TimeRange {
        self.temp_range.as_ref().unwrap_or(&self.default_range)
    }

    fn id(&self) -> &PeriodId {
        &self.id
    }
    fn name(&self) -> &PeriodName {
        &self.name
    }
}