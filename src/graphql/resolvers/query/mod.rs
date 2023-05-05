//! This module contains solely the [QueryRoot] struct.
//! It only exists for organizational purposes.

mod get_teacher;
mod all_teachers;
mod all_periods;


use crate::graphql_types::{
    scalars::teacher::*,
    juniper_types::IntoFieldError,
    *,
};

use super::{period::PeriodMetadata, teacher::TeacherMetadata};

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

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    async fn get_teacher(
        ctx: &Context,
        first_name: Option<String>,
        last_name: Option<String>,
        id: Option<TeacherId>
    ) -> juniper::FieldResult<TeacherMetadata> {
        let mut db_context_mut = ctx.get_db_mut().await;

        get_teacher
            ::get_teacher(&mut db_context_mut.client, first_name, last_name, id)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    async fn all_teachers(
        ctx: &Context,
    ) -> juniper::FieldResult<Vec<TeacherMetadata>> {
        let mut db_context_mut = ctx.get_db_mut().await;

        all_teachers
            ::all_teachers(&mut db_context_mut.client)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    async fn all_periods(
        ctx: &Context,
    ) -> juniper::FieldResult<Vec<PeriodMetadata>> {
        let mut db_context_mut = ctx.get_db_mut().await;

        all_periods
            ::all_periods(&mut db_context_mut.client)
            .await
            .map_err(IntoFieldError::into_field_error)
    }
}




