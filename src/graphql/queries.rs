pub mod get_teacher;
pub mod all_teachers;

use super::prelude::*;


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