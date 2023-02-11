//! This *__private__* module contains solely things things required to
//! execute and resolve the `allPeriods` GraphQL mutation.
//! 
//! Main function is [all_periods].


use std::borrow::Cow;

use crate::utils::list_to_value;
use crate::database::prelude::*;

use crate::{
    preludes::graphql::*,
    graphql_types::{
        periods::*,
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
        AllPeriods => "all_periods"
}

make_static_enum_error! {
    /// This struct contains all the possible error types that can occur when executing the all_periods query.
    /// All 3 of the types are server errors (S).
    pub AllPeriodsError;
        /// S - A prepared query (&Statement) failed to load due to some error. Contains the names of the queries.
        PreparedQuery(Vec<&'static str>)
            => "1 or more prepared queries failed.",
                "prepared_fail" ==> |failed_list| {
                    "failed": list_to_value(failed_list),
                };
        /// S - Something went wrong while running a specific SQL query.
        Exec(DbExecError)
            => "Database error",
                "db_failed" ==> |error_type| {
                    "part_failed": error_type.error_str(),
                };
        /// S - Something else went wrong with the database.
        OtherDb(Cow<'static, str>)
            => "Unknown database error",
                "db_failed" ==> |reason| {
                    "reason": &*reason,
                };
        // /// S - Catch-all for other things.
        // Other(String)
        //     => "Unknown server error",
        //         "server_failed" ==> |reason| {
        //             "reason": reason,
        //         };
}

/// Executes the query all_teachers. Takes no parameters.
pub async fn all_periods(
    db_client: &mut Client,
) -> Result<Vec<Period>, AllPeriodsError> {
    let at = read::all_periods_query(db_client).await;

    let at = handle_prepared!(
        at;
        AllPeriodsError::PreparedQuery
    )?;

    let period_rows = get_all_periods(db_client, at).await?;

    period_rows.into_iter().map(
        |row| row
            .try_into()
            .map_err(AllPeriodsError::OtherDb)
    ).collect()
}
/// Does what it says. Gets all of the period rows unconditionally, returning an Exec error if it fails.
/// 
/// Optimally, `ap` should be a reference to a memoized query obtained by 
/// ```
/// use crate::database::prepared::read;
/// 
/// let result = read::all_periods(&ctx.db_context.client).await;
/// let memoized = match result {
///     Ok(query) => query,
///     Err(e) => todo!("handle error: {}", e),
/// };
/// ```
async fn get_all_periods(db_client: &Client, ap: &Statement) -> Result<Vec<Row>, AllPeriodsError> {
    db_client
        .query(ap, &[])
        .await
        .map_err(|error| {
            println!("Error: {:?}", error);
            AllPeriodsError::Exec(DbExecError::AllPeriods)
        })
}
