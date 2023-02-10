use uuid::Uuid;

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
        Get => "retrieve_period_data"
        Modify => "modify_period_table"
}

make_static_enum_error! {
    /// This struct contains all the possible error types that can occur when executing the all_teachers query.
    /// 1 is a client error (C), and 3 are server errors (S).
    pub AddPeriodError;
        /// C - Something else went wrong with the database.
        DuplicateError(Uuid)
            => "There is already a period with this name",
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
pub async fn add_period(
    db_client: &mut Client,
    name: &str,
) -> Result<Period, AddPeriodError> {

    let (pbn, ap) = tokio::join!(
        read::period_by_name_query(db_client),
        modifying::add_period_query(db_client),
    );

    let (pbn, ap) = handle_prepared!(
        pbn, ap;
        AddPeriodError::PreparedQueryError
    )?;



    if let Some(duplicate_period) = get_period_by_name(db_client, name, pbn).await? {
        Err(AddPeriodError::DuplicateError(duplicate_period.id.uuid()))
    } else if let Some(modify_error) = add_period_to_db(db_client, name, ap).await {
        Err(modify_error)
    } else if let Some(period) = get_period_by_name(db_client, name, pbn).await? {
        Ok(period)
    } else {
        Err(AddPeriodError::ExecError(DbExecError::Get))
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
async fn add_period_to_db(db_client: &Client, name: &str, ap: &Statement) -> Option<AddPeriodError> {
    use AddPeriodError::*;
    use self::DbExecError::*;


    let query_result = db_client.execute(ap, &[&name]).await;
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

async fn get_period_by_name(db_client: &Client, name: &str, gpbn: &Statement) -> Result<Option<Period>, AddPeriodError> {
    use AddPeriodError::*;
    use self::DbExecError::*;

    let query_result = db_client.query_opt(gpbn, &[&name]).await;
    if let Ok(opt_period) = query_result {
        if let Some(period) = opt_period {
            Ok(Some(period.try_into().map_err(|_| Other())?))
        } else {
            Ok(None)
        }
    } else {
        Err(ExecError(Get))
    }
}