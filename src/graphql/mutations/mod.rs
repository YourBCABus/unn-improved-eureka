//! This module contains solely the [MutationRoot] struct.
//! It only exists for organizational purposes.

mod add_teacher;
mod add_period;
mod delete_teacher;
mod update_teacher;
mod update_teacher_absence;



use crate::graphql_types::{
    teachers::*,
    periods::*,
    juniper_types::IntoFieldError,
    *,
};


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
    ) -> juniper::FieldResult<TeacherMetadata> {
        let mut db_context_mut = ctx.get_db_mut().await;
        
        add_teacher
        ::add_teacher(&mut db_context_mut.client, &name)
        .await
        .map_err(IntoFieldError::into_field_error)
    }
    
    async fn update_teacher(
        ctx: &Context,
        id: TeacherId,
        name: String,
    ) -> juniper::FieldResult<TeacherMetadata> {
        let mut db_context_mut = ctx.get_db_mut().await;

        update_teacher
            ::update_teacher(&mut db_context_mut.client, id, &name)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    async fn update_teacher_absence(
        ctx: &Context,
        id: TeacherId,
        names: Vec<PeriodName>,
        fully_absent: bool,
    ) -> juniper::FieldResult<Teacher> {
        let mut db_context_mut = ctx.get_db_mut().await;

        update_teacher_absence
            ::update_teacher_absence(&mut db_context_mut.client, id, &names, fully_absent)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    async fn delete_teacher(
        ctx: &Context,
        id: TeacherId,
    ) -> juniper::FieldResult<bool> {
        let mut db_context_mut = ctx.get_db_mut().await;

        delete_teacher
            ::delete_teacher(&mut db_context_mut.client, id)
            .await
            .map_err(IntoFieldError::into_field_error)?;
        Ok(true)
    }


    async fn add_period(
        ctx: &Context,
        name: String,
    ) -> juniper::FieldResult<Period> {
        let mut db_context_mut = ctx.get_db_mut().await;
        
        add_period
            ::add_period(&mut db_context_mut.client, &name)
            .await
            .map_err(IntoFieldError::into_field_error)
    }
}