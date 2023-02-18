use std::borrow::Cow;

use uuid::Uuid;

use crate::graphql::resolvers::teacher::TeacherMetadata;
use crate::graphql_types::inputs::PronounSetInput;
use crate::utils::list_to_value;
use crate::database::prelude::*;

use crate::{
    preludes::graphql::*,
    graphql_types::scalars::{
        teacher::*,
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
        Get => "get_teacher_data"
        Modify => "modify_teacher"
}

make_static_enum_error! {
    prefix: "updateTeacher";
    /// This struct contains all the possible error types that can occur when executing the update_teacher mutation.
    /// 2 are client errors (C), and 3 are server errors (S).
    pub UpdateTeacherError;
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
        Other(Cow<'static, str>)
            => "Unknown Error",
                "server_err" ==> |error_type| {
                    "err": &*error_type,
                };
}

/// Executes the mutation update_teacher. Takes Context, an ID, and the new TeacherInput struct.
/// Returns the changed teacher.
/// TODO: Make this require auth.
pub async fn update_teacher(
    db_client: &mut Client,
    id: TeacherId,
    new_name: Option<&str>,
    new_honorific: Option<&str>,
    new_pronouns: Option<&PronounSetInput>,
) -> Result<TeacherMetadata, UpdateTeacherError> {
    let (gtbi, utmq) = tokio::join!(
        read::get_teacher_by_id_query(db_client),
        modifying::update_teacher_metadata_query(db_client),
    );

    let (gtbi, utmq) = handle_prepared!(
        gtbi, utmq;
        UpdateTeacherError::PreparedQueryError
    )?;
    
    let (id, _) = id.try_into_uuid().map_err(UpdateTeacherError::IdFormatError)?;

    let old_teacher_state = get_teacher_by_id(&id, db_client, gtbi).await?;
    
    let name = new_name.unwrap_or_else(|| old_teacher_state.name.name_str());
    let honorific = new_honorific.unwrap_or(&old_teacher_state.honorific);
    let pronouns = if let Some(pronouns) = new_pronouns {
        pronouns.format_sql()
    } else {
        old_teacher_state.pronouns.format_sql()
    };

    update_teacher_query(
        &id,
        name,
        honorific,
        &pronouns,
        db_client,
        utmq
    ).await?;

    get_teacher_by_id(&id, db_client, gtbi).await
}

/// Gets the full **metadata** for a teacher from the db.
/// 
/// Optimally, `gtbn` should be a reference to a memoized query.
/// ```ignore
/// let db_client = /* get your database client here */;
/// let gtbn_query = get_memoized_gtbn_query(&db_client).await.unwrap();
///
/// let id = uuid!("00000000-0000-0000-0000-000000000000");
/// 
/// match get_teacher_by_id(&id, &db_client, gtbn_query).await {
///     Ok(teacher_row) => eprintln!("Teacher data: {:?}", teacher_row),
///     Err(err) => eprintln!("Failed to retrieve teacher: {:?}", err),
/// }
/// ```
async fn get_teacher_by_id(id: &Uuid, db_client: &Client, gtbi: &Statement) -> Result<TeacherMetadata, UpdateTeacherError> {
    let query_result = db_client.query_opt(gtbi, &[&id]).await;

    if let Ok(teacher) = query_result {
        if let Some(teacher) = teacher {
            teacher
                .try_into()
                .map(From::<TeacherRow>::from)
                .map_err(UpdateTeacherError::Other)
        } else {
            Err(UpdateTeacherError::IdDoesNotExist(*id))
        }            
    } else {
        Err(UpdateTeacherError::ExecError(DbExecError::Get))
    }
}

/// Gets the full **metadata** for a teacher from the db.
/// 
/// Optimally, `utq` should be a reference to a memoized query.
/// ```ignore
/// let db_client = /* get your database client here */;
/// let utq_query = get_memoized_utq_query(&db_client).await.unwrap();
///
/// let id = uuid!("00000000-0000-0000-0000-000000000000");
/// let absence_state = ("Mr. Smith", true, false); // Mr Smith is absent for part of the day.
/// 
/// match update_teacher_query(&id, absence_state, &db_client, utq_query).await {
///     Ok(teacher_row) => eprintln!("Teacher data: {:?}", teacher_row),
///     Err(err) => eprintln!("Failed to retrieve teacher: {:?}", err),
/// }
/// ```
async fn update_teacher_query(
    id: &Uuid,
    
    name: &str,
    honorific: &str,
    pronouns: &str,
    
    db_client: &Client,
    utmq: &Statement,
) -> Result<(), UpdateTeacherError> {
    let rows_modified = db_client
        .execute(utmq, &[&id, &name, &honorific, &pronouns])
        .await
        .map_err(|_| UpdateTeacherError::ExecError(DbExecError::Modify))?;

    if rows_modified == 1 {
        Ok(())
    } else {
        Err(UpdateTeacherError::IdDoesNotExist(*id))
    }
}
