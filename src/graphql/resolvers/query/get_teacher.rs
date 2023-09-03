//! This *__private__* module contains solely things things required to
//! execute and resolve the `getTeacher` GraphQL mutation.
//! 
//! Main function is [get_teacher].

use sqlx::Postgres;
use sqlx::pool::PoolConnection;
// use tokio_postgres::{Row, Statement};
use uuid::Uuid;



// use crate::graphql::resolvers::teacher::TeacherMetadata;
use crate::types::Teacher;
// use crate::utils::list_to_value;


// use crate::macros::{
//     handle_prepared,
//     make_unit_enum_error,
//     make_static_enum_error,
// };


// make_unit_enum_error! {
//     /// Database execution errors
//     pub DbExecError
//         ById => "teacher_by_id"
//         ByName => "teacher_by_name"
// }

// make_static_enum_error! {
//     <S>
//     prefix: "getTeacher";
//     /// This struct contains all the possible error types that can occur when executing the get_teacher query.
//     /// 4 of the types are client errors (C), 4 are server errors (S).
//     pub GetTeacherError;
//         /// C - Signifies the passing of an incorrectly-formatted teacher ID (should be a UUID). 
//         IdFormatError(TeacherId)
//             => "The teacher ID was incorrectly formatted",
//                 "bad_id_format" ==> |bad_id| {
//                     "id": bad_id.id_str(),
//                 };
//         /// C - The teacher ID format was correct, but it doesn't refer to any existing teacher.
//         IdDoesNotExist(Uuid)
//             => "There is no teacher associated with this ID",
//                 "id_does_not_exist" ==> |id| {
//                     "id": id.to_string(),
//                 };
//         /// C - The teacher name doesn't refer to any existing teacher.
//         NameDoesNotExist(String)
//             => "There is no teacher associated with this name",
//                 "name_does_not_exist" ==> |name|  {
//                     "name": name,
//                 };
//         /// C - One of the preconditions of GraphQL Contract was violated.
//         ContractViolated(&'static str, JuniperValue<S>)
//             => "Contract violation",
//                 "contract_violated" ==> |part_violated, violation_description|  {
//                     "part_violated": part_violated,
//                     "violation": violation_description,
//                 };

//         /// S - A prepared query (&Statement) failed to load due to some error. Contains the names of the queries.
//         PreparedQueryError(Vec<&'static str>)
//             => "1 or more prepared queries failed.",
//                 "prepared_fail" ==> |failed_list| {
//                     "failed": list_to_value(failed_list),
//                 };
//         /// S - Something went wrong while running a specific SQL query.
//         ExecError(DbExecError)
//             => "Database error",
//                 "db_failed" ==> |error_type| {
//                     "part_failed": error_type.error_str(),
//                 };
//         /// S - Something else went wrong with the database.
//         OtherDbError(Cow<'static, str>)
//             => "Unknown database error",
//                 "db_failed" ==> |reason| {
//                     "reason": &*reason,
//                 };
//         // /// S - Catch-all for other things.
//         // Other(String)
//         //     => "Unknown server error",
//         //         "server_failed" ==> |reason| {
//         //             "reason": reason,
//         //         };
// }

/// Executes the query get_teacher. Takes Context, an optional name, and an optional ID.
/// At least one of the name or ID must be provided, otherwise a contract violation error is returned.
pub async fn get_teacher(
    db_client: &mut PoolConnection<Postgres>,
    id: Uuid,
) -> Result<Teacher, sqlx::Error> {

    crate::database::prepared::teacher::get_teacher(db_client, id).await
}

// /// Does what it says. Gets the teacher row by ID, returning an ExecError if it fails.
// /// 
// /// Optimally, `gtbi` should be a reference to a memoized query obtained by 
// /// ```
// /// use crate::database::prepared::read;
// /// 
// /// let result = read::get_teacher_by_id_query(&ctx.db_context.client).await;
// /// let memoized = match result {
// ///     Ok(query) => query,
// ///     Err(e) => todo!("handle error: {}", e),
// /// };
// /// ```
// async fn get_teacher_row_by_id<S: ScalarValue>(uuid: Uuid, client: &Client, gtbi: &Statement) -> Result<Option<Row>, GetTeacherError<S>> {
//     client
//         .query_opt(gtbi, &[&uuid])
//         .await
//         .map_err(|_| GetTeacherError::ExecError(DbExecError::ById))
// }

// /// Does what it says. Gets the teacher row by Name, returning an ExecError if it fails.
// /// 
// /// Optimally, `gtbn` should be a reference to a memoized query obtained by 
// /// ```
// /// use crate::database::prepared::read;
// /// 
// /// let result = read::get_teacher_by_name_query(&ctx.db_context.client).await;
// /// let memoized = match result {
// ///     Ok(query) => query,
// ///     Err(e) => todo!("handle error: {}", e),
// /// };
// /// ```
// async fn get_teacher_row_by_name<S: ScalarValue>(first: &str, last: &str, client: &Client, gtbn: &Statement) -> Result<Option<Row>, GetTeacherError<S>> {
//     client
//         .query_opt(gtbn, &[&first, &last])
//         .await
//         .map_err(|_| GetTeacherError::ExecError(DbExecError::ByName))
// }
