
use async_graphql::{
    Object,
    Context,
    Error as GraphQlError,
    Result as GraphQlResult,
};
use uuid::Uuid;


use crate::graphql::req_id;
use crate::types::{Period, Teacher};
use crate::database::Ctx;
use crate::logging::*;

use super::TimeRange;
use super::get_db;

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
        let mut db_conn = get_db!(ctx);
        let req_id = req_id(ctx);

        let ids = TeacherList::get_by_period(req_id, self.id, &mut db_conn)
            .await
            .map_err(|e| {
                let e = e.to_string();
                error!("{} - Failed to get absent teacher ids from database {e}", fmt_req_id(req_id));
                GraphQlError::new(format!("Failed to get absent teacher ids from database {e}"))
            })?;

        ids.get_teachers(req_id, &mut db_conn)
            .await
            .map_err(|e| {
                let e = e.to_string();
                error!("{} - Failed to get absent teacher data from database {e}", fmt_req_id(req_id));
                GraphQlError::new(format!("Failed to get absent teachers from database {e}"))
            })
    }
}


#[derive(Debug, Clone)]
pub struct TeacherList(Vec<Uuid>);

impl TeacherList {
    pub async fn get_by_period(req_id: Uuid, period_id: Uuid, db: &mut Ctx) -> sqlx::Result<Self> {
        use crate::database::prepared::absences::get_all_absences_for_period;

        trace!("{} - Getting absent teachers for period <{}>", fmt_req_id(req_id), period_id);
        let absences = get_all_absences_for_period(db, period_id).await?;

        Ok(Self(absences.into_iter().map(|a| a.teacher).collect()))
    }

    pub async fn get_teachers(self, req_id: Uuid, db: &mut Ctx) -> sqlx::Result<Vec<Teacher>> {
        use std::collections::HashMap;
        use crate::database::prepared::teacher::get_all_teachers;

        trace!("{} - Getting the {} teachers inside of this struct", fmt_req_id(req_id), self.0.len());
        let mut teacher_map: HashMap<_, _> = get_all_teachers(db).await?
            .into_iter()
            .map(|teacher| (teacher.get_id(), teacher))
            .collect();

        let teachers = self.0
            .into_iter()
            .filter_map(|id| teacher_map.remove(&id))
            .collect();
        
        Ok(teachers)
    }
}


