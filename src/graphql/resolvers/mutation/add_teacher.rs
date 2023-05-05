use std::borrow::Cow;

use uuid::Uuid;

use crate::graphql::resolvers::teacher::TeacherMetadata;
use crate::graphql_types::inputs::PronounSetInput;
use crate::utils::list_to_value;
use crate::database::prelude::*;

use crate::utils::structs::TeacherRow;
use crate::{
    preludes::graphql::*,
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
    prefix: "addTeacher";
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
        Other(Cow<'static, str>)
            => "Unknown server error",
                "unknown" ==> |error_type| {
                    "err": &*error_type,
                };
}


/// Executes the mutation add_teacher. Takes Context and an ID.
/// Returns TeacherMetadata, which should be sufficient, given that the teacher will be fully present anyway.
/// TODO: Make this require auth.
/// 
/// ```ignore
/// let name = "Mr. Smith";
/// let db_client = /* get your database client here */;
/// 
/// match add_teacher(&db_client, name).await {
///     Err(AddTeacherError::DuplicateError(id)) => println!("Duplicate found with id: {}", id),
///     Err(err) => eprintln!("Some other error was encountered: {:?}", err),
///     Ok(teacher) => println!("Teacher added successfully: {:?}", teacher),
/// }
/// ```
pub async fn add_teacher(
    db_client: &mut Client,
    first_name: &str,
    last_name: &str,
    honorific: &str,
    pronouns: &PronounSetInput,
) -> Result<TeacherMetadata, AddTeacherError> {

    let (at, gtbn) = tokio::join!(
        modifying::add_teacher_query(db_client),
        read::get_teacher_by_name_query(db_client),
    );

    let (at, gtbn) = handle_prepared!(
        at, gtbn;
        AddTeacherError::PreparedQueryError
    )?;

    if let Some(err) = check_teacher_duplicate(
        db_client,
        first_name,
        last_name,
        gtbn,
    ).await {
        Err(err)
    } else if let Some(modify_error) = add_teacher_to_db(
        db_client,
        first_name, last_name,
        honorific, &pronouns.format_sql(),
        at,
    ).await {
        Err(modify_error)
    } else { 
        get_teacher(db_client, first_name, last_name, gtbn).await.map(Into::into)
    }
}

/// Does what it says. Checks whether the teacher exists by looking up its name.
/// May fail with a DuplicateError error in the case of the name already being registered.
/// 
/// Optimally, `ctbn` should be a reference to a memoized query found in the crate::database::prepared.
/// ```ignore
/// let db_client = /* get your database client here */;
/// let gtbn_query = get_memoized_gtbn_query(&db_client).await.unwrap();
///
/// let first = "Ryan Smith";
/// let last  = "Smith";
/// 
/// match check_teacher_duplicate(&db_client, first, last, gtbn_query).await {
///     Some(AddTeacherError::DuplicateError(id)) => println!("Duplicate found with id: {}", id),
///     Some(err) => eprintln!("Duplicate check failed with error: {:?}", err),
///     None => println!("No duplicate was found"),
/// }
/// ```
async fn check_teacher_duplicate(db_client: &Client, first: &str, last: &str, gtbn: &Statement) -> Option<AddTeacherError> {
    use AddTeacherError::*;
    use self::DbExecError::*;

    let query_result = db_client.query_opt(gtbn, &[&first, &last]).await;
    if let Ok(returned_value) = query_result {
        returned_value
            .map(|dup_teacher| DuplicateError(dup_teacher.get("TeacherId")))
    } else {
        Some(ExecError(Duplicate))
    }
}

/// Attempts to add a teacher to the DB.
/// 
/// Optimally, `at` should be a reference to a memoized query.
/// ```ignore
/// let db_client = /* get your database client here */;
/// let at_query = get_memoized_at_query(&db_client).await.unwrap();
///
/// let name = "Mr. Smith";
/// 
/// match add_teacher_to_db(&db_client, name, at_query).await {
///     Some(err) => eprintln!("Adding the teacher failed: {:?}", err),
///     None => println!("Teacher added successfully"),
/// }
/// ```
async fn add_teacher_to_db(db_client: &Client, first: &str, last: &str, honorific: &str, pronouns: &str, at: &Statement) -> Option<AddTeacherError> {
    use AddTeacherError::*;
    use self::DbExecError::*;


    let query_result = db_client.execute(at, &[&first, &last, &honorific, &pronouns]).await;
    if let Ok(1) = query_result {
        None
    } else {
        Some(ExecError(Modify))
    }
}

/// Gets the full **metadata** for a teacher from the db.
/// 
/// Optimally, `gtbn` should be a reference to a memoized query.
/// ```ignore
/// let db_client = /* get your database client here */;
/// let gtbn_query = get_memoized_gtbn_query(&db_client).await.unwrap();
///
/// let name = "Mr. Smith";
/// 
/// match get_teacher(&db_client, name, gtbn_query).await {
///     Ok(teacher_row) => eprintln!("Teacher data: {:?}", teacher_row),
///     Err(err) => eprintln!("Failed to retrieve teacher: {:?}", err),
/// }
/// ```
async fn get_teacher(db_client: &Client, first: &str, last: &str, gtbn: &Statement) -> Result<TeacherRow, AddTeacherError> {
    use AddTeacherError::*;
    use self::DbExecError::*;

    let query_result = db_client.query_opt(gtbn, &[&first, &last]).await;
    if let Ok(Some(row)) = query_result {
        row
            .try_into()
            .map_err(Other)
    } else {
        Err(ExecError(Get))
    }
}