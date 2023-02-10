use tokio_postgres::Transaction;
use uuid::Uuid;

use crate::preludes::graphql::helpers::teachers::PopulateTeacherAbsenceError;
use crate::utils::list_to_value;
use crate::database::prelude::*;

use crate::{
    preludes::graphql::*,
    graphql_types::{
        teachers::*,
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
        GetTeacher => "get_teacher_data"
        GetAllPeriods => "get_all_periods"
        GetSomePeriods => "get_some_periods"
        SetPeriods => "set_periods"
        Modify => "modify_teacher"
}

make_static_enum_error! {
    /// This struct contains all the possible error types that can occur when executing the update_teacher mutation.
    /// 2 are client errors (C), and 3 are server errors (S).
    pub UpdateTeacherAbsenceError;
        /// C - This id was not parseble in the correct format (UUID).
        TeacherIdFormatError(String)
            => "The teacher ID was incorrectly formatted",
                "bad_id_format" ==> |id| {
                    "id": id,
                };
        /// C - There was no teacher associated with this ID.
        TeacherIdDoesNotExist(String)
            => "There is no teacher associated with this ID",
                "id_does_not_exist" ==> |id| {
                    "id": id,
                };
        /// C - There was no teacher associated with this ID.
        PeriodNameDoesNotExist(String)
            => "There is no period associated with this name",
                "name_does_not_exist" ==> |id| {
                    "name": id,
                };
                
        /// TODO: Make this work
        PopulateError(PopulateTeacherAbsenceError)
            => "Absence state failed to populate",
                "populate_failed" ==> |error|  {
                    "part_failed": error,
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
        Other(String)
            => "Unknown Error",
                "server_err" ==> |error_type| {
                    "err": error_type,
                };
}

/// Executes the mutation update_teacher. Takes Context, an ID, and the new TeacherInput struct.
/// Returns the changed teacher.
/// TODO: Make this require auth.
pub async fn update_teacher_absence(
    db_client: &mut Client,
    teacher_id: TeacherId,
    period_names: &[PeriodName],
    fully_absent: bool,
) -> Result<Teacher, UpdateTeacherAbsenceError> {
    let (
        gtbi,
        pbn, apq,
        cpft, atp,
        utq,
    ) = tokio::join!(
        read::get_teacher_by_id_query(db_client),

        read::period_by_name_query(db_client),
        read::all_periods_query(db_client),
        
        modifying::clear_periods_for_teacher_query(db_client),
        modifying::add_teacher_periods(db_client),
        
        modifying::update_teacher_query(db_client),
    );

    let (
        gtbi,
        pbn, apq,
        cpft, atp,
        utq,
    ) = handle_prepared!(
        gtbi,
        pbn, apq,
        cpft, atp,
        utq;
        UpdateTeacherAbsenceError::PreparedQueryError
    )?;
    
    let (teacher_id, _) = teacher_id.try_into_uuid().map_err(UpdateTeacherAbsenceError::TeacherIdFormatError)?;
    
    let transaction = db_client.transaction().await.unwrap();


    let teacher_row = get_teacher_by_id_query(&teacher_id, &transaction, gtbi).await?;

    let periods = if fully_absent {
        get_all_periods_query(&transaction, apq).await?
    } else {
        get_periods_by_names_query(period_names.iter().map(|n| n.name_str()), &transaction, pbn).await?
    };
    
    clear_teacher_periods(&teacher_id, &transaction, cpft).await?;
    set_teacher_periods(&teacher_id, periods.iter().map(|p| p.id.uuid()), &transaction, atp).await?;

    update_teacher_query(&teacher_id, (teacher_row.name.name_str(), !periods.is_empty(), fully_absent), &transaction, utq).await?;

    let teacher_row = get_teacher_by_id_query(&teacher_id, &transaction, gtbi).await?;

    let teacher = helpers::teachers::populate_absence(teacher_row, transaction.client()).await
        .map_err(UpdateTeacherAbsenceError::PopulateError)?;

    if let Err(e) = transaction.commit().await {
        Err(UpdateTeacherAbsenceError::Other("transaction failed to commit".to_string()))
    } else {
        Ok(teacher)
    }

}


async fn get_teacher_by_id_query(id: &Uuid, transaction: &Transaction<'_>, gtbi: &Statement) -> Result<TeacherRow, UpdateTeacherAbsenceError> {
    let query_result = transaction.query_opt(gtbi, &[&id]).await;


    if let Ok(teacher) = query_result {
        if let Some(teacher) = teacher {
            teacher
                .try_into()
                .map_err(UpdateTeacherAbsenceError::Other)
        } else {
            Err(UpdateTeacherAbsenceError::TeacherIdDoesNotExist(id.to_string()))
        }            
    } else {
        Err(UpdateTeacherAbsenceError::ExecError(DbExecError::GetTeacher))
    }
}

async fn get_all_periods_query(transaction: &Transaction<'_>, apq: &Statement) -> Result<Vec<Period>, UpdateTeacherAbsenceError> {
    match transaction.query(apq, &[]).await {
        Ok(v) => v
            .into_iter()
            .map(|row| row.try_into())
            .collect::<Result<_, _>>()
            .map_err(|e| UpdateTeacherAbsenceError::Other(format!("DESERIALIZATION ERROR: {}", e))),
        Err(_) => Err(UpdateTeacherAbsenceError::ExecError(DbExecError::GetAllPeriods)),
    }
}

async fn get_periods_by_names_query(names: impl Iterator<Item = &str>, transaction: &Transaction<'_>, pbn: &Statement) -> Result<Vec<Period>, UpdateTeacherAbsenceError> {
    let period_futures = names
        .map(|name| async move {
            match transaction.query_opt(pbn, &[&name]).await {
                Ok(Some(period_row)) => match period_row.try_into() {
                    Ok(period) => Ok(period),
                    Err(e) => Err(UpdateTeacherAbsenceError::Other(format!("DESERIALIZATION ERROR: {}", e))),
                },
                Ok(_) => Err(UpdateTeacherAbsenceError::PeriodNameDoesNotExist(name.to_string())),
                Err(_) => Err(UpdateTeacherAbsenceError::ExecError(DbExecError::GetSomePeriods)),
            }
            
        });

    let results = futures::future::join_all(period_futures).await;

    results.into_iter().collect()
}

async fn clear_teacher_periods(teacher_id: &Uuid, transaction: &Transaction<'_>, cpft: &Statement) -> Result<u64, UpdateTeacherAbsenceError> {
    transaction
        .execute(cpft, &[&teacher_id])
        .await
        .map_err(|_| UpdateTeacherAbsenceError::ExecError(DbExecError::Modify))
}

async fn set_teacher_periods(teacher_id: &Uuid, period_ids: impl Iterator<Item = Uuid>, transaction: &Transaction<'_>, atp: &Statement) -> Result<(), UpdateTeacherAbsenceError> {
    let period_futures = period_ids
        .map(|period_id| async move {
            match transaction.execute(atp, &[&period_id, teacher_id]).await {
                Ok(1) => Ok(()),
                _ => Err(UpdateTeacherAbsenceError::ExecError(DbExecError::SetPeriods)),
            }
        });

    let results = futures::future::join_all(period_futures).await;

    results.into_iter().collect()
}

async fn update_teacher_query(
    id: &Uuid,
    (name, is_absent, fully_absent): (&str, bool, bool),
    transaction: &Transaction<'_>,
    utq: &Statement,
) -> Result<(), UpdateTeacherAbsenceError> {
    let rows_modified = transaction
        .execute(utq, &[&id, &name, &is_absent, &fully_absent])
        .await
        .map_err(|_| UpdateTeacherAbsenceError::ExecError(DbExecError::Modify))?;

    if rows_modified == 1 {
        Ok(())
    } else {
        Err(UpdateTeacherAbsenceError::TeacherIdDoesNotExist(id.to_string()))
    }
}
