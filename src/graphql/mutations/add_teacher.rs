use tokio_postgres::Statement;
use uuid::Uuid;

use crate::preludes::{
    database::*,
    graphql::*,
    macros::*,
};

use crate::utils::list_to_value;


make_unit_enum_error! {
    /// Database execution errors
    pub DbExecError
        Duplicate => "check_for_duplicates"
        Modify => "modify_teacher_table"
        Get => "retrieve_teacher_data"
}

make_static_enum_error! {
    /// This struct contains all the possible error types that can occur when executing the all_teachers query.
    /// 1 is a client error (C), and 3 are server errors (S).
    pub AddTeacherError;
        /// C - Something else went wrong with the database.
        DuplicateError(Uuid)
            => "There is already a teacher with this name",
                "name_entry_found" ==> |id| {
                    "id": id.to_string(),
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
        /// S - 
        Other()
            => "Unknown server error",
                "unknown" ==> | | {};
}


/// Executes the mutation delete_teacher. Takes Context and an ID.
/// Returns nothing.
/// TODO: Make this require auth.
pub async fn add_teacher(
    ctx: &Context,
    name: &str,
) -> Result<Teacher, AddTeacherError> {
    use AddTeacherError::*;

    let ctbn = read::teacher_id_by_name_query(&ctx.db_context.client).await;
    let at = modifying::add_teacher_query(&ctx.db_context.client).await;
    let gtbn = read::get_teacher_by_name_query(&ctx.db_context.client).await;

    if let (Some(ctbn), Some(at), Some(gtbn)) = (ctbn, at, gtbn) {
        if let Some(err) = check_teacher_duplicate(ctx, name, ctbn).await {
            Err(err)
        } else if let Some(modify_error) = add_teacher_to_db(ctx, name, at).await {
            Err(modify_error)
        } else { 
            get_teacher(ctx, name, gtbn).await
        }
    } else {
        let failed_queries = [wvn!(ctbn), wvn!(at), wvn!(gtbn)]
            .into_iter()
            .flat_map(
                |(option, name)| option
                    .is_none()
                    .then_some(name)
            )
            .collect();
        Err(PreparedQueryError(failed_queries))
    }
}

/// Does what it says. Check whether the teacher.
/// May fail with a DuplicateError error in the case of the name already being registered.
/// 
/// Optimally, `ctbn` should be a reference to a memoized query obtained by 
/// ```
/// use crate::database::prepared::read;
/// 
/// let result = read::check_teacher_by_name_query(&ctx.db_context.client).await;
/// let ctbn = match result {
///     Ok(query) => query,
///     Err(e) => todo!("handle error: {}", e),
/// };
/// ```
async fn check_teacher_duplicate(ctx: &Context, name: &str, ctbn: &Statement) -> Option<AddTeacherError> {
    use AddTeacherError::*;
    use self::DbExecError::*;


    let query_result = ctx.db_context.client.query_opt(ctbn, &[&name]).await;
    if let Ok(returned_value) = query_result {
        returned_value
            .map(|dup_teacher| DuplicateError(dup_teacher.get("TeacherId")))
    } else {
        Some(ExecError(Duplicate))
    }
}

/// Does what it says. Attempts to add a teacher to the DB.
/// May fail with a DuplicateError error in the case of the name already being registered.
/// 
/// Optimally, `ctbn` should be a reference to a memoized query obtained by 
/// ```
/// use crate::database::prepared::read;
/// 
/// let result = read::teacher_id_by_name_query(&ctx.db_context.client).await;
/// let ctbn = match result {
///     Ok(query) => query,
///     Err(e) => todo!("handle error: {}", e),
/// };
/// ```
async fn add_teacher_to_db(ctx: &Context, name: &str, at: &Statement) -> Option<AddTeacherError> {
    use AddTeacherError::*;
    use self::DbExecError::*;


    let query_result = ctx.db_context.client.execute(at, &[&name]).await;
    if let Ok(rows) = query_result {
        if rows != 1 {

            Some(Other())
        } else {
            None
        }
    } else {
        Some(ExecError(Modify))
    }
}

async fn get_teacher(ctx: &Context, name: &str, gtbn: &Statement) -> Result<Teacher, AddTeacherError> {
    use AddTeacherError::*;
    use self::DbExecError::*;

    let query_result = ctx.db_context.client.query_opt(gtbn, &[&name]).await;
    if let Ok(Some(teacher)) = query_result {
        teacher
            .try_into()
            .map_err(|_| Other())
    } else {
        Err(ExecError(Get))
    }
}