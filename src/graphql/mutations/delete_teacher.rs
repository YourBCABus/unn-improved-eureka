use tokio_postgres::Statement;
use uuid::Uuid;

use crate::preludes::{
    database::*,
    graphql::*,
    macros::*,
    utils::list_to_value,
};

make_unit_enum_error! {
    /// Database execution errors
    pub DbExecError
        Delete => "delete_teacher"
}

make_static_enum_error! {
    /// This struct contains all the possible error types that can occur when executing the all_teachers query.
    /// 2 are client errors (C), and 2 are server errors (S).
    pub DeleteTeacherError;
        /// C - This id was not parseble in the correct format (UUID).
        IdFormatError(String)
            => "The teacher ID was incorrectly formatted",
                "bad_id_format" ==> |id| {
                    "id": id,
                };
        /// C - There was no teacher associated with this ID.
        IdDoesNotExist(String)
            => "There is no teacher associated with this ID",
                "id_does_not_exist" ==> |id| {
                    "id": id,
                };
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
}

/// Executes the mutation delete_teacher. Takes Context and an ID.
/// Returns nothing.
/// TODO: Make this require auth.
pub async fn delete_teacher(
    ctx: &Context,
    id: TeacherId,
) -> Result<(), DeleteTeacherError> {
    let dtq = modifying::delete_teacher_query(&ctx.db_context.client).await;

    let dtq = handle_prepared!(
        dtq;
        DeleteTeacherError::PreparedQueryError
    )?;
    
    let (id, _) = id.try_into_uuid().map_err(DeleteTeacherError::IdFormatError)?;

    delete_teacher_by_id(&id, ctx, dtq).await
}

/// Does what it says. Attempts to permanantly delete a teacher from the DB.
/// May fail with an `IdDoesNotExist` error in the case of it not being a valid existent ID
/// or an ExecError if it fails while deleting it.
/// 
/// Optimally, `dtq` should be a reference to a memoized query obtained by 
/// ```
/// use crate::database::prepared::modifying;
/// 
/// let result = modifying::delete_teacher_query(&ctx.db_context.client).await;
/// let memoized = match result {
///     Ok(query) => query,
///     Err(e) => todo!("handle error: {}", e),
/// };
/// ```
async fn delete_teacher_by_id(id: &Uuid, ctx: &Context, dtq: &Statement) -> Result<(), DeleteTeacherError> {
    let rows_modified = ctx.db_context.client
        .execute(dtq, &[&id])
        .await
        .map_err(|_| DeleteTeacherError::ExecError(DbExecError::Delete))?;
    if rows_modified == 1 {
        Ok(())
    } else {
        Err(DeleteTeacherError::IdDoesNotExist(id.to_string()))
    }
}
