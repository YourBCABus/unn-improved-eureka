use uuid::Uuid;

use crate::utils::list_to_value;
use crate::database::prelude::*;

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
        Delete => "delete_teacher"
}

make_static_enum_error! {
    /// This struct contains all the possible error types that can occur when executing the delete_teacher mutation.
    /// 2 are client errors (C), and 2 are server errors (S).
    pub DeleteTeacherError;
        /// C - This id was not parseble in the correct format (UUID).
        IdFormatError(TeacherId)
            => "The teacher ID was incorrectly formatted",
                "bad_id_format" ==> |id| {
                    "id": id.id_str(),
                };
        /// C - There was no teacher associated with this ID.
        IdDoesNotExist(Uuid)
            => "There is no teacher associated with this ID",
                "id_does_not_exist" ==> |id| {
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
}

/// Executes the mutation delete_teacher. Takes Context and an ID.
/// Returns nothing.
/// TODO: Make this require auth.
pub async fn delete_teacher(
    db_client: &mut Client,
    id: TeacherId,
) -> Result<(), DeleteTeacherError> {
    let dtq = modifying::delete_teacher_query(db_client).await;

    let dtq = handle_prepared!(
        dtq;
        DeleteTeacherError::PreparedQueryError
    )?;
    
    let (id, _) = id.try_into_uuid().map_err(DeleteTeacherError::IdFormatError)?;

    delete_teacher_by_id(&id, db_client, dtq).await
}

/// Attempts to permanantly delete a teacher from the DB.
/// 
/// May fail with an `IdDoesNotExist` error in the case of it not being a valid existent ID.
/// 
/// Optimally, `dtq` should be a reference to a memoized query.
/// ```ignore
/// let db_client = /* get your database client here */;
/// let dtq_query = get_memoized_dtq_query(&db_client).await.unwrap();
///
/// let id = uuid!("00000000-0000-0000-0000-000000000000");
/// 
/// match add_period_to_db(&id, &db_client, dtq_query).await {
///     Ok(()) => println!("Teacher deleted successfully."),
///     Err(err) => eprintln!("Deleting the teacher failed: {:?}", err),
/// }
/// ```
async fn delete_teacher_by_id(id: &Uuid, db_client: &Client, dtq: &Statement) -> Result<(), DeleteTeacherError> {
    let rows_modified = db_client
        .execute(dtq, &[&id])
        .await
        .map_err(|_| DeleteTeacherError::ExecError(DbExecError::Delete))?;
    if rows_modified == 1 {
        Ok(())
    } else {
        Err(DeleteTeacherError::IdDoesNotExist(*id))
    }
}
