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
    graphql_types::scalars::period::*,
};

use crate::macros::{
    handle_prepared,
    make_unit_enum_error,
    make_static_enum_error,
};

make_unit_enum_error! {
    /// Database execution errors
    pub DbExecError
        Get => "get_period_data"
        Modify => "modify_period"
}

make_static_enum_error! {
    prefix: "updatePeriod";
    /// This struct contains all the possible error types that can occur when executing the update_teacher mutation.
    /// 2 are client errors (C), and 3 are server errors (S).
    pub UpdatePeriodError;
        /// C - This id was not parseble in the correct format (UUID).
        IdFormatError(PeriodId)
            => "The period ID was incorrectly formatted",
                "bad_id_format" ==> |id| {
                    "id": id.id_str(),
                };
        /// C - There was no period associated with this ID.
        IdDoesNotExist(Uuid)
            => "There is no period associated with this ID",
                "id_does_not_exist" ==> |id| {
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

pub async fn update_period(
    db_client: &mut Client,
    id: PeriodId,
    name: Option<&str>,
    default_time: Option<TimeRangeInput>,
    temp_time: (bool, Option<TimeRangeInput>),
) -> Result<PeriodMetadata, UpdatePeriodError> {
    let (gpbi, upq) = tokio::join!(
        read::get_period_by_id_query(db_client),
        modifying::update_period_query(db_client),
    );

    let (gpbi, upq) = handle_prepared!(
        gpbi, upq;
        UpdatePeriodError::PreparedQueryError
    )?;
    
    let (id, _) = id.try_into_uuid().map_err(UpdatePeriodError::IdFormatError)?;

    let old_period_state = get_period_by_id(&id, db_client, gpbi).await?;
    
    let new_period_name = name.unwrap_or_else(|| old_period_state.name.name_str());
    let new_default_range = match default_time {
        Some(new_default) => TimeRange::try_from(new_default).map_err(UpdatePeriodError::TimeError)?,
        None => old_period_state.default_range,
    };
    let new_temp_range = match temp_time {
        (true, Some(new_default)) => Some(TimeRange::try_from(new_default).map_err(UpdatePeriodError::TimeError)?),
        (true, None) => None,
        (false, _) => old_period_state.temp_range,
    };
    update_period_query(&id, new_period_name, new_default_range, new_temp_range.as_ref(), db_client, upq).await?;

    get_period_by_id(&id, db_client, gpbi).await
}

async fn get_period_by_id(id: &Uuid, db_client: &Client, gpbi: &Statement) -> Result<PeriodMetadata, UpdatePeriodError> {
    let query_result = db_client.query_opt(gpbi, &[&id]).await;

    if let Ok(teacher) = query_result {
        if let Some(teacher) = teacher {
            teacher
                .try_into()
                .map(From::<PeriodRow>::from)
                .map_err(UpdatePeriodError::Other)
        } else {
            Err(UpdatePeriodError::IdDoesNotExist(*id))
        }            
    } else {
        Err(UpdatePeriodError::ExecError(DbExecError::Get))
    }
}

async fn update_period_query(
    id: &Uuid,
    name: &str,
    time: TimeRange,
    temp_range: Option<&TimeRange>,
    db_client: &Client,
    utq: &Statement,
) -> Result<(), UpdatePeriodError> {
    let rows_modified = db_client
        .execute(utq, &[
            &id, &name,
            &time.start(), &time.end(),
            &temp_range.map(TimeRange::start), &temp_range.map(TimeRange::end)
        ])
        .await
        .map_err(|_| UpdatePeriodError::ExecError(DbExecError::Modify))?;

    if rows_modified == 1 {
        Ok(())
    } else {
        Err(UpdatePeriodError::IdDoesNotExist(*id))
    }
}