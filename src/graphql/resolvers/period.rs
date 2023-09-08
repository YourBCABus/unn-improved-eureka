
use async_graphql::{
    Object,
    Context,
    Error as GraphQlError,
    Result as GraphQlResult,
};
use crate::{types::{Period, Teacher}, state::AppState, database::Ctx};
use uuid::Uuid;


use super::TimeRange;

#[Object]
impl Period {

    async fn id(&self) -> Uuid { self.id }
    async fn name(&self) -> &str { &self.name }

    async fn default_time_range(&self) -> TimeRange {
        (self.start, self.end).into()
    }
    async fn time_range(&self) -> TimeRange {
        (self.temp_start.unwrap_or(self.start), self.temp_end.unwrap_or(self.end)).into()
    }


    async fn teachers_absent(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQlResult<Vec<Teacher>> {

        let ctx_accessor = ctx;
        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        let ids = TeacherList::get_by_period(self.id, &mut db_conn)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Failed to get absent teacher ids from database {e}"))
            })?;

        ids.get_teachers(&mut db_conn)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Failed to get absent teachers from database {e}"))
            })
    }

}
// {
//     async fn teachers_absent(
//         &self,
//         ctx: &Context,
//     ) -> FieldResult<Vec<TeacherMetadata>> {
//         let db_context = ctx.get_db_mut().await;

//         teachers_absent::absent_periods(&self.id.uuid(), &db_context.client)
//             .await
//             .map_err(IntoFieldError::into_field_error)
//     }

//     async fn default_time_range(&self) -> &TimeRange {
//         &self.default_range
//     }

//     async fn time_range(&self) -> &TimeRange {
//         self.temp_range.as_ref().unwrap_or(&self.default_range)
//     }

//     async fn id(&self) -> &PeriodId {
//         &self.id
//     }
//     async fn name(&self) -> &PeriodName {
//         &self.name
//     }
// }




#[derive(Debug, Clone)]
pub struct TeacherList(Vec<Uuid>);

impl TeacherList {
    pub async fn get_by_period(period_id: Uuid, db: &mut Ctx) -> sqlx::Result<Self> {
        use crate::database::prepared::absences::get_all_absences_for_period;

        let absences = get_all_absences_for_period(db, period_id).await?;

        Ok(Self(absences.into_iter().map(|a| a.teacher).collect()))
    }

    pub async fn get_teachers(self, db: &mut Ctx) -> sqlx::Result<Vec<Teacher>> {
        use std::collections::HashMap;
        use crate::database::prepared::teacher::get_all_teachers;

        let mut teacher_map: HashMap<_, _> = get_all_teachers(db).await?
            .into_iter()
            .map(|teacher| (teacher.get_id(), teacher))
            .collect();

        let teachers = self.0
            .into_iter()
            .flat_map(|id| teacher_map.remove(&id))
            .collect();
        
        Ok(teachers)
    }
}


