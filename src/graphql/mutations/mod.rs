//! This module contains solely the [MutationRoot] struct.
//! It only exists for organizational purposes.

mod add_teacher;
mod delete_teacher;

use super::prelude::*;


/// This is a memberless struct implementing all the mutations for `improved-eureka`.
/// This includes:
/// - `add_teacher(name?, id?) -> Teacher`
/// - `delete_teacher() -> bool`
/// 
/// Generally, it will only be used as part of a [schema][super::Schema].
pub struct MutationRoot;

#[juniper::graphql_object(context = Context)]
impl MutationRoot {
    async fn add_teacher(
        ctx: &Context,
        name: String,
    ) -> juniper::FieldResult<Teacher> {
        add_teacher
            ::add_teacher(ctx, &name)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    async fn delete_teacher(
        ctx: &Context,
        id: TeacherId,
    ) -> juniper::FieldResult<bool> {
        delete_teacher
            ::delete_teacher(ctx, id)
            .await
            .map_err(IntoFieldError::into_field_error)?;
        Ok(true)
    }
}