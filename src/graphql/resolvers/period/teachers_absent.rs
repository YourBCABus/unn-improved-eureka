use std::borrow::Cow;

use uuid::Uuid;

use crate::graphql::resolvers::teacher::TeacherMetadata;
use crate::utils::list_to_value;
use crate::database::prelude::*;

use crate::preludes::graphql::*;

use crate::macros::{
    handle_prepared,
    make_unit_enum_error,
    make_static_enum_error,
};

make_unit_enum_error! {
    /// Database execution errors
    pub DbExecError
        Get => "retrieve_period_teacher"
}

make_static_enum_error! {
    /// This struct contains all the possible error types that can occur when executing the all_teachers query.
    /// 1 is a client error (C), and 3 are server errors (S).
    pub TeachersAbsentError;
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


pub async fn absent_periods(period_id: &Uuid, db_client: &Client) -> Result<Vec<TeacherMetadata>, TeachersAbsentError> {
    let gpt = read::get_teachers_from_period_query(db_client).await;

    let gpt = handle_prepared!(
        gpt;
        TeachersAbsentError::PreparedQueryError
    )?;

    let teachers = get_teachers_for_periods(period_id, db_client, gpt).await?;

    Ok(teachers)
}

async fn get_teachers_for_periods(period_id: &Uuid, transaction: &Client, gpt: &Statement) -> Result<Vec<TeacherMetadata>, TeachersAbsentError> {
    let teachers = transaction
        .query(gpt, &[&period_id])
        .await
        .map_err(|_| TeachersAbsentError::ExecError(DbExecError::Get))?;
    
    teachers.into_iter()
        .map(|teacher_row| TeacherRow
            ::try_from(teacher_row)
            .map(Into::into)
            .map_err(TeachersAbsentError::Other)
        )
        .collect()
}

