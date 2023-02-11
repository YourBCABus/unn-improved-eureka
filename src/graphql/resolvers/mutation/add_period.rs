use std::borrow::Cow;

use uuid::Uuid;

use crate::graphql::resolvers::period::PeriodMetadata;
use crate::graphql::resolvers::time_range::TimeRange;
use crate::graphql_types::inputs::TimeRangeInput;
use crate::utils::list_to_value;
use crate::database::prelude::*;

use crate::utils::structs::PeriodRow;
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
        Get => "retrieve_period_data"
        Modify => "modify_period_table"
}

make_static_enum_error! {
    prefix: "addPeriod";
    /// This struct contains all the possible error types that can occur when executing the all_teachers query.
    /// 1 is a client error (C), and 3 are server errors (S).
    pub AddPeriodError;
        /// C - Something else went wrong with the database.
        DuplicateError(Uuid)
            => "There is already a period with this name",
                "name_entry_found" ==> |id| {
                    "id": id.to_string(),
                };
        
        /// TODO: Document
        TimeError((f64, f64))
            => "Failed to interpret time range.",
                "time_out_of_range" ==> |range| {
                    "start": range.0,
                    "end": range.1,
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


/// Executes the mutation add_period. Takes Context and an ID.
/// Returns Period.
/// TODO: Make this require auth.
/// 
/// ```ignore
/// let name = "Period One";
/// let db_client = /* get your database client here */;
/// 
/// match add_period(&db_client, name).await {
///     Err(AddTeacherError::DuplicateError(id)) => println!("Duplicate found with id: {}", id),
///     Err(err) => eprintln!("Some other error was encountered: {:?}", err),
///     Ok(teacher) => println!("Period added successfully: {:?}", teacher),
/// }
/// ```
pub async fn add_period(
    db_client: &mut Client,
    name: &str,
    default_time_range: TimeRangeInput,
) -> Result<PeriodMetadata, AddPeriodError> {

    let (pbn, ap) = tokio::join!(
        read::get_period_by_name_query(db_client),
        modifying::add_period_query(db_client),
    );

    let (pbn, ap) = handle_prepared!(
        pbn, ap;
        AddPeriodError::PreparedQueryError
    )?;

    if let Some(duplicate_period) = get_period_by_name(db_client, name, pbn).await? {
        Err(AddPeriodError::DuplicateError(duplicate_period.id.uuid()))
    } else if let Some(modify_error) = add_period_to_db(
        db_client,
        name,
        &TimeRange::try_from(default_time_range).map_err(AddPeriodError::TimeError)?,
        ap
    ).await {
        Err(modify_error)
    } else if let Some(period) = get_period_by_name(db_client, name, pbn).await? {
        Ok(period)
    } else {
        Err(AddPeriodError::ExecError(DbExecError::Get))
    }
}

/// Attempts to add a period to the DB.
/// 
/// Optimally, `ap` should be a reference to a memoized query.
/// ```ignore
/// let db_client = /* get your database client here */;
/// let ap_query = get_memoized_ap_query(&db_client).await.unwrap();
///
/// let name = "Period One";
/// 
/// match add_period_to_db(&db_client, name, ap_query).await {
///     Some(err) => eprintln!("Adding the period failed: {:?}", err),
///     None => println!("Period added successfully"),
/// }
/// ```
async fn add_period_to_db(db_client: &Client, name: &str, default_time_range: &TimeRange, ap: &Statement) -> Option<AddPeriodError> {
    use AddPeriodError::*;
    use self::DbExecError::*;

    let query_result = db_client.execute(ap, &[
        &name,
        &default_time_range.start(),
        &default_time_range.end(),
    ]).await;
    if let Ok(1) = query_result {
        None
    } else {
        Some(ExecError(Modify))
    }
}

/// Gets the full metedata for a period from the db.
/// 
/// Optimally, `gpbn` should be a reference to a memoized query.
/// ```ignore
/// let db_client = /* get your database client here */;
/// let gpbn_query = get_memoized_gpbn_query(&db_client).await.unwrap();
///
/// let name = "Period One";
/// 
/// match get_period_by_name(&db_client, name, gpbn_query).await {
///     Ok(period) => eprintln!("Period data: {:?}", period),
///     Err(err) => eprintln!("Failed to retrieve period: {:?}", err),
/// }
/// ```
async fn get_period_by_name(db_client: &Client, name: &str, gpbn: &Statement) -> Result<Option<PeriodMetadata>, AddPeriodError> {
    use AddPeriodError::*;
    use self::DbExecError::*;

    let query_result = db_client.query_opt(gpbn, &[&name]).await;
    if let Ok(opt_period) = query_result {
        if let Some(period) = opt_period {
            Ok(Some(period.try_into().map(From::<PeriodRow>::from).map_err(Other)?))
        } else {
            Ok(None)
        }
    } else {
        Err(ExecError(Get))
    }
}
