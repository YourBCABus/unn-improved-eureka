use tokio_postgres::{Row, Statement};

use crate::preludes::{
    database::*,
    graphql::*,
    macros::*,
    utils::list_to_value,
};


make_unit_enum_error! {
    /// Database execution errors
    pub DbExecError
        AllTeachers => "all_teachers"
}

make_static_enum_error! {
    /// This struct contains all the possible error types that can occur when executing the all_teachers query.
    /// All 4 of the types are server errors (S).
    pub AllTeachersError;
        /// S - A prepared query (&Statement) failed to load due to some error. Contains the names of the queries.
        PreparedQueryError(Vec<&'static str>)
            => "1 or more prepared queries failed.",
                "prepared_fail" ==> |failed_list| {
                    "failed": list_to_value(failed_list),
                };
        /// S - Something went wrong while running a specific SQL query.
        ExecError(DbExecError)
            => "Database error",
                "db_failed" ==> |error_type| {
                    "part_failed": error_type.error_str(),
                };
        /// S - Something else went wrong with the database.
        OtherDbError(String)
            => "Unknown database error",
                "db_failed" ==> |reason| {
                    "reason": reason,
                };
        /// S - Catch-all for other things.
        Other(String)
            => "Unknown server error",
                "server_failed" ==> |reason| {
                    "reason": reason,
                };
}

/// Executes the query get_teacher. Takes no parameters.
pub async fn all_teachers(
    ctx: &Context,
) -> Result<Vec<Teacher>, AllTeachersError> {
    let at = read::all_teachers(&ctx.db_context.client).await;

    let at = handle_prepared!(
        at;
        AllTeachersError::PreparedQueryError
    )?;

    let teacher_rows = get_all_teachers(ctx, at).await?;

    teacher_rows.into_iter().map(
        |row| row
            .try_into()
            .map_err(AllTeachersError::OtherDbError)
    ).collect()
}

async fn get_all_teachers(ctx: &Context, at: &Statement) -> Result<Vec<Row>, AllTeachersError> {
    ctx.db_context.client
        .query(at, &[])
        .await
        .map_err(|_| AllTeachersError::ExecError(DbExecError::AllTeachers))
}