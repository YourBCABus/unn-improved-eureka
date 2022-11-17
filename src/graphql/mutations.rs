pub mod add_teacher;
pub mod delete_teacher;

use super::prelude::*;


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