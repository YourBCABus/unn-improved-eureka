use std::borrow::Cow;

use uuid::Uuid;

use crate::graphql::resolvers::period::PeriodMetadata;
use crate::utils::list_to_value;
use crate::database::prelude::*;

use crate::utils::structs::PeriodRow;
use crate::{
    preludes::graphql::*,
    graphql::resolvers::absence_state::AbsenceStateMetadata,
};

use crate::macros::{
    handle_prepared,
    make_unit_enum_error,
    make_static_enum_error,
};

make_unit_enum_error! {
    /// Database execution errors
    pub DbExecError
        Get => "retrieve_teacher_periods"
}

make_static_enum_error! {
    /// This struct contains all the possible error types that can occur when executing the all_teachers query.
    /// 1 is a client error (C), and 3 are server errors (S).
    pub AbsentPeriodsError;
        /// S - A prepared query (&Statement) failed to load due to some error. Contains the names of the queries.
        PreparedQueryError(Vec<&'static str>)
            => "1 or more prepared queries failed.",
                "id_does_not_exist" ==> |failed_list| {
                    "failed": list_to_value(failed_list),
                };
        /// S - Something went wrong while running a specific SQL query.
        ExecError(DbExecError)
            => "Database error",
                "db_failed" ==> |error_type| {
                    "part_failed": error_type.error_str(),
                };
        /// S - 
        Other(Cow<'static, str>)
            => "Unknown server error",
                "unknown" ==> |error_type| {
                    "err": &*error_type,
                };
}


    





pub async fn absent_periods(absence_metadata: &AbsenceStateMetadata, db_client: &Client) -> Result<Option<Vec<PeriodMetadata>>, AbsentPeriodsError> {
    if let TeacherPresence::FullPresent = absence_metadata.absence_state_meta {
        Ok(None)
    } else {

        let gtp = read::get_periods_from_teacher_query(db_client).await;

        let gtp = handle_prepared!(
            gtp;
            AbsentPeriodsError::PreparedQueryError
        )?;

        let periods = get_teacher_periods(&absence_metadata.teacher_id.uuid(), db_client, gtp).await?;

        Ok(Some(periods))
    }
}

async fn get_teacher_periods(teacher_id: &Uuid, transaction: &Client, gtp: &Statement) -> Result<Vec<PeriodMetadata>, AbsentPeriodsError> {
    let periods = transaction
        .query(gtp, &[&teacher_id])
        .await
        .map_err(|_| AbsentPeriodsError::ExecError(DbExecError::Get))?;
    
    periods.into_iter()
        .map(|period_row| PeriodRow
            ::try_from(period_row)
            .map(Into::into)
            .map_err(AbsentPeriodsError::Other)
        )
        .collect()
}

