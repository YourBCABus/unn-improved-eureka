use tokio_postgres::Transaction;
use uuid::Uuid;

use crate::graphql_scalars::period::PeriodId;
use crate::utils::list_to_value;
use crate::database::prelude::*;

use crate::preludes::graphql::*;

use crate::macros::{
    handle_prepared,
    make_unit_enum_error,
    make_static_enum_error,
};

make_unit_enum_error! {
    /// Database execution errors
    pub DbExecError
        TransactionInit => "transaction_init"
        TransactionCommit => "transaction_commit"
        ClearTeachers => "clear_teachers"
        Delete => "delete_period"
}

make_static_enum_error! {
    prefix: "deletePeriod";
    /// This struct contains all the possible error types that can occur when executing the delete_teacher mutation.
    /// 2 are client errors (C), and 2 are server errors (S).
    pub DeletePeriodError;
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

/// Executes the mutation delete_period. Takes Context and an ID.
/// Returns nothing.
/// TODO: Make this require auth.
pub async fn delete_period(
    db_client: &Client,
    id: PeriodId,
) -> Result<(), DeletePeriodError> {
    let dpq = modifying::delete_period_query(db_client).await;
    let ctfp = modifying::clear_teachers_for_period_query(db_client).await;

    let (dpq, ctfp) = handle_prepared!(
        dpq, ctfp;
        DeletePeriodError::PreparedQueryError
    )?;
    
    let (id, _) = id.try_into_uuid().map_err(DeletePeriodError::IdFormatError)?;


    let transaction = db_client.transaction().await;
    let transaction = transaction.map_err(|_| DeletePeriodError::ExecError(DbExecError::TransactionInit))?;

    clear_period_teachers(&id, &transaction, ctfp).await?;
    delete_period_by_id(&id, &transaction, dpq).await?;

    transaction.commit().await.map_err(|_| DeletePeriodError::ExecError(DbExecError::TransactionCommit))?;
    
    Ok(())
}

async fn clear_period_teachers(id: &Uuid, transaction: &Transaction<'_>, ctfp: &Statement) -> Result<u64, DeletePeriodError> {
    transaction
        .execute(ctfp, &[&id])
        .await
        .map_err(|_| DeletePeriodError::ExecError(DbExecError::ClearTeachers))
}

/// Attempts to permanantly delete a period from the DB.
/// 
/// May fail with an `IdDoesNotExist` error in the case of it not being a valid existent ID.
/// 
/// Optimally, `dpq` should be a reference to a memoized query.
/// TODO: FINISH DOCUMENTING.
async fn delete_period_by_id(id: &Uuid, transaction: &Transaction<'_>, dpq: &Statement) -> Result<(), DeletePeriodError> {
    let rows_modified = transaction
        .execute(dpq, &[&id])
        .await
        .map_err(|a| dbg!(a))
        .map_err(|_| DeletePeriodError::ExecError(DbExecError::Delete))?;
    if rows_modified == 1 {
        Ok(())
    } else {
        Err(DeletePeriodError::IdDoesNotExist(*id))
    }
}
