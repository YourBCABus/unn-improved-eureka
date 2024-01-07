use async_graphql::Object;
use async_graphql::{ Error as GraphQlError, Result as GraphQlResult, Context };

use crate::types::{Teacher, PronounSet, TeacherName, Period};

use super::get_db;

use uuid::Uuid;

#[Object]
impl Teacher {
    // async fn absence_state(&self) -> AbsenceStateMetadata {
    //     AbsenceStateMetadata::from_id_and_meta(self.id.clone(), self.absence_state_meta)
    // }

    async fn id(&self) -> Uuid {
        self.get_id()
    }

    async fn pronouns(&self) -> &PronounSet {
        &self.get_pronouns()
    }

    async fn name(&self) -> &TeacherName {
        self.get_name()
    }

    async fn absence(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQlResult<Vec<Period>> {
        // trace!("{} - Expanding {}'s {} current absence data", fmt_req_id(req_id(ctx)), self.get_name(), SmallId(Some("t"), req_id(ctx)));

        let mut db_conn = get_db!(ctx);

        let ids = PeriodList::get_by_teacher(self.get_id(), &mut db_conn)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Failed to get periods ids this teacher is absent from database {e}"))
            })?;

        ids.get_periods(&mut db_conn)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Failed to get periods this teacher is absent from database {e}"))
            })
    }

    async fn fully_absent(&self) -> bool {
        self.get_fully_absent()
    }
}



#[derive(Debug, Clone)]
pub struct PeriodList(Vec<Uuid>);


use crate::database::Ctx;

impl PeriodList {
    pub async fn get_by_teacher(period_id: Uuid, db: &mut Ctx) -> sqlx::Result<Self> {
        use crate::database::prepared::absences::get_all_absences_for_teacher;

        let absences = get_all_absences_for_teacher(db, period_id).await?;

        Ok(PeriodList(absences.into_iter().map(|a| a.period).collect()))
    }

    pub async fn get_periods(self, db: &mut Ctx) -> sqlx::Result<Vec<Period>> {
        use std::collections::HashMap;
        use crate::database::prepared::period::get_all_periods;

        let mut period_map: HashMap<_, _> = get_all_periods(db).await?
            .into_iter()
            .map(|period| (period.id, period))
            .collect();

        let periods = self.0
            .into_iter()
            .filter_map(|id| period_map.remove(&id))
            .collect();
        
        Ok(periods)
    }
}

