use uuid::Uuid;

use crate::utils::list_to_value;
use crate::database::prelude::*;

use crate::utils::structs::TeacherRow;
use crate::{
    preludes::graphql::*,
    graphql_types::{
        teachers::*,
    },
};

use crate::macros::{
    handle_prepared,
    make_unit_enum_error,
    make_static_enum_error,
};

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
    db_client: &mut Client,
    name: &str,
) -> Result<TeacherMetadata, AddTeacherError> {

    let (ctbn, at, gtbn) = tokio::join!(
        read::teacher_id_by_name_query(db_client),
        modifying::add_teacher_query(db_client),
        read::get_teacher_by_name_query(db_client),
    );

    let (ctbn, at, gtbn) = handle_prepared!(
        ctbn, at, gtbn;
        AddTeacherError::PreparedQueryError
    )?;

    if let Some(err) = check_teacher_duplicate(db_client, name, ctbn).await {
        Err(err)
    } else if let Some(modify_error) = add_teacher_to_db(db_client, name, at).await {
        Err(modify_error)
    } else { 
        get_teacher(db_client, name, gtbn).await.map(Into::into)
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
async fn check_teacher_duplicate(db_client: &Client, name: &str, ctbn: &Statement) -> Option<AddTeacherError> {
    use AddTeacherError::*;
    use self::DbExecError::*;


    let query_result = db_client.query_opt(ctbn, &[&name]).await;
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
async fn add_teacher_to_db(db_client: &Client, name: &str, at: &Statement) -> Option<AddTeacherError> {
    use AddTeacherError::*;
    use self::DbExecError::*;


    let query_result = db_client.execute(at, &[&name]).await;
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

async fn get_teacher(db_client: &Client, name: &str, gtbn: &Statement) -> Result<TeacherRow, AddTeacherError> {
    use AddTeacherError::*;
    use self::DbExecError::*;

    let query_result = db_client.query_opt(gtbn, &[&name]).await;
    if let Ok(Some(row)) = query_result {
        row
            .try_into()
            .map_err(|_| Other())
    } else {
        Err(ExecError(Get))
    }
}