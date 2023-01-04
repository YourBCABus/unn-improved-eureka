//! This module contains solely the [QueryRoot] struct.
//! It only exists for organizational purposes.

mod get_teacher;
mod all_teachers;

use super::prelude::*;

/// This is a memberless struct implementing all the queries for `improved-eureka`.
/// This includes:
/// - `get_teacher(name?, id?) -> Teacher`
/// - `all_teachers() -> Teacher[]`
/// 
/// Generally, it will only be used as part of a [schema][super::Schema].
pub struct QueryRoot;

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    async fn get_teacher(
        ctx: &Context,
        name: Option<TeacherName>,
        id: Option<TeacherId>
    ) -> juniper::FieldResult<Teacher> {
        get_teacher
            ::get_teacher(ctx, name, id)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    async fn all_teachers(
        ctx: &Context,
    ) -> juniper::FieldResult<Vec<Teacher>> {
        all_teachers
            ::all_teachers(ctx)
            .await
            .map_err(IntoFieldError::into_field_error)
    }
}