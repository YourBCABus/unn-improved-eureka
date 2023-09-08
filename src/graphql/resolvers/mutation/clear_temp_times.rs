use crate::utils::list_to_value;
use crate::database::prelude::*;

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
        Times => "clear_temp_times"
}

make_static_enum_error! {
    prefix: "deleteTeacher";
    /// This struct contains all the possible error types that can occur when executing the clear_temp_times mutation.
    /// 2 are server errors (S).
    pub ClearTempTimesError;
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

pub async fn clear_temp_times(
    db_client: &mut Client,
) -> Result<(), ClearTempTimesError> {
    use ClearTempTimesError::*;
    use DbExecError::Times;

    let ctt = modifying::clear_temp_times(db_client).await;

    let ctt = handle_prepared!(
        ctt;
        PreparedQueryError
    )?;

    db_client.execute(ctt, &[]).await.map_err(|_| ExecError(Times))?;
    
    Ok(())
}
