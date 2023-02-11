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
        TransactionInit => "init_transaction"
        TransactionComm => "commit_transaction"
        Absences => "clear_absences"
        AbsenceMeta => "clear_absence_metadata"
}

make_static_enum_error! {
    prefix: "deleteTeacher";
    /// This struct contains all the possible error types that can occur when executing the clear_absences mutation.
    /// 2 are server errors (S).
    pub ClearAbsencesError;
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

pub async fn clear_absences(
    db_client: &mut Client,
) -> Result<(), ClearAbsencesError> {
    use ClearAbsencesError::*;
    use DbExecError::*;

    let ca = modifying::clear_absences(db_client).await;
    let cam = modifying::clear_absence_metadata(db_client).await;

    let (ca, cam) = handle_prepared!(
        ca, cam;
        ClearAbsencesError::PreparedQueryError
    )?;
    let transaction = db_client
        .transaction().await
        .map_err(|_| ExecError(TransactionInit))?;

    transaction.execute(ca, &[]).await.map_err(|_| ExecError(Absences))?;
    transaction.execute(cam, &[]).await.map_err(|_| ExecError(AbsenceMeta))?;

    transaction.commit()
        .await.map_err(|_| ExecError(TransactionComm))?;
    Ok(())
}
