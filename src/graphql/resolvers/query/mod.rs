//! This module contains solely the [QueryRoot] struct.
//! It only exists for organizational purposes.

// mod get_teacher;
// mod all_teachers;
mod all_periods;


use crate::state::AppState;
use crate::types::Teacher;

use super::super::structs::*;

use super::period::PeriodMetadata;
use juniper::{IntoFieldError, graphql_value, FieldError};

/// This is a memberless struct implementing all the queries for `improved-eureka`.
/// This includes:
/// - `get_teacher(name?, id?) -> Teacher`
/// - `all_teachers() -> Teacher[]`
/// 
/// - `all_periods() -> Period[]`
/// - `get_period(name?, id?) -> Period`
/// 
/// Generally, it will only be used as part of a [schema][super::Schema].
pub struct QueryRoot;

#[juniper::graphql_object(context = AppState)]
impl QueryRoot {
    async fn get_teacher(
        ctx: &AppState,
        id: JuniperUuid,
    ) -> juniper::FieldResult<Teacher> {
        use crate::database::prepared::teacher::get_teacher as get_teacher_from_db;

        let Ok(id) = id.try_uuid() else {
            return Err(IntoFieldError::into_field_error(
                graphql_value!({ "bad_uuid": id }),
            ));
        };

        let mut db_conn = ctx.db
            .acquire()
            .await
            .map_err(|e| FieldError::new(
                "Could not open connection to the database",
                graphql_value!({ "internal_error": e }),
            ))?;

        get_teacher_from_db(&mut db_conn, id)
            .await
            .map_err(|e| FieldError::new(
                "Failed to get teacher from database",
                graphql_value!({ "internal_error": e }),
            ))
    }

    async fn all_teachers(
        ctx: &AppState,
    ) -> juniper::FieldResult<Vec<Teacher>> {
        use crate::database::prepared::teacher::get_all_teachers as get_all_teachers_from_db;

        let mut db_conn = ctx.db
            .acquire()
            .await
            .map_err(|e| FieldError::new(
                "Could not open connection to the database",
                graphql_value!({ "internal_error": e }),
            ))?;

        get_all_teachers_from_db(&mut db_conn)
            .await
            .map_err(|e| FieldError::new(
                "Failed to get teachers from database",
                graphql_value!({ "internal_error": e }),
            ))
    }

    async fn all_periods(
        ctx: &AppState,
    ) -> juniper::FieldResult<Vec<PeriodMetadata>> {
        let mut db_context_mut = ctx.get_db_mut().await;

        all_periods
            ::all_periods(&mut db_context_mut.client)
            .await
            .map_err(IntoFieldError::into_field_error)
    }
}




