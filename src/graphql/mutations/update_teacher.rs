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
        Get => "get_teacher_data"
        Modify => "modify_teacher"
}

make_static_enum_error! {
    /// This struct contains all the possible error types that can occur when executing the update_teacher mutation.
    /// 2 are client errors (C), and 3 are server errors (S).
    pub UpdateTeacherError;
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
                "query_load_failed" ==> |failed_list| {
                    "failed": list_to_value(failed_list),
                };
        /// S - Something went wrong while running a specific SQL query.
        ExecError(DbExecError)
            => "Database error",
                "db_failed" ==> |error_type| {
                    "part_failed": error_type.error_str(),
                };
        /// S - Something went wrong.
        Other(String)
            => "Unknown Error",
                "server_err" ==> |error_type| {
                    "err": error_type,
                };
}

/// Executes the mutation update_teacher. Takes Context, an ID, and the new TeacherInput struct.
/// Returns the changed teacher.
/// TODO: Make this require auth.
pub async fn update_teacher(
    ctx: &Context,
    id: TeacherId,
    name: &str,
) -> Result<Teacher, UpdateTeacherError> {
    let (gtbi, utq) = tokio::join!(
        read::get_teacher_by_id_query(&ctx.db_context.client),
        modifying::update_teacher_query(&ctx.db_context.client),
    );

    let (gtbi, utq) = handle_prepared!(
        gtbi, utq;
        UpdateTeacherError::PreparedQueryError
    )?;
    
    let (id, _) = id.try_into_uuid().map_err(UpdateTeacherError::IdFormatError)?;

    let old_teacher_state = get_teacher_by_id(&id, ctx, gtbi).await?;
    
    let new_teacher_name = Some(name).or(Some(old_teacher_state.name.name_str())).unwrap();
    update_teacher_query(&id, (new_teacher_name, false, false), ctx, utq).await?;

    get_teacher_by_id(&id, ctx, gtbi).await
}

async fn get_teacher_by_id(id: &Uuid, ctx: &Context, gtbi: &Statement) -> Result<Teacher, UpdateTeacherError> {
    let query_result = ctx.db_context.client.query_opt(gtbi, &[&id]).await;

    if let Ok(teacher) = query_result {
        if let Some(teacher) = teacher {
            teacher
                .try_into()
                .map_err(|e| UpdateTeacherError::Other(e))
        } else {
            Err(UpdateTeacherError::IdDoesNotExist(id.to_string()))
        }            
    } else {
        Err(UpdateTeacherError::ExecError(DbExecError::Get))
    }
}


async fn update_teacher_query(
    id: &Uuid,
    (name, is_absent, fully_absent): (&str, bool, bool),
    ctx: &Context,
    utq: &Statement,
) -> Result<(), UpdateTeacherError> {
    let rows_modified = ctx.db_context.client
        .execute(utq, &[&id, &name, &is_absent, &fully_absent])
        .await
        .map_err(|_| UpdateTeacherError::ExecError(DbExecError::Modify))?;

    if rows_modified == 1 {
        Ok(())
    } else {
        Err(UpdateTeacherError::IdDoesNotExist(id.to_string()))
    }
}


// /// Does what it says. Attempts to permanantly delete a teacher from the DB.
// /// May fail with an `IdDoesNotExist` error in the case of it not being a valid existent ID
// /// or an ExecError if it fails while deleting it.
// /// 
// /// Optimally, `dtq` should be a reference to a memoized query obtained by 
// /// ```
// /// use crate::database::prepared::modifying;
// /// 
// /// let result = modifying::delete_teacher_query(&ctx.db_context.client).await;
// /// let memoized = match result {
// ///     Ok(query) => query,
// ///     Err(e) => todo!("handle error: {}", e),
// /// };
// /// ```
// async fn delete_teacher_by_id(id: &Uuid, ctx: &Context, dtq: &Statement) -> Result<(), DeleteTeacherError> {
//     let rows_modified = ctx.db_context.client
//         .execute(dtq, &[&id])
//         .await
//         .map_err(|_| DeleteTeacherError::ExecError(DbExecError::Delete))?;
//     if rows_modified == 1 {
//         Ok(())
//     } else {
//         Err(DeleteTeacherError::IdDoesNotExist(id.to_string()))
//     }
// }
