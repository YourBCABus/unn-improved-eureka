//! This module contains solely the [MutationRoot] struct.
//! It only exists for organizational purposes.

mod add_teacher;
mod update_teacher;
mod delete_teacher;

mod add_period;
mod update_period;
mod delete_period;


mod update_teacher_absence;
mod clear_absences;
mod clear_temp_times;

use crate::graphql_types::{
    scalars::teacher::*,
    scalars::period::*,
    juniper_types::IntoFieldError,
    *, inputs::{TimeRangeInput, PronounSetInput},
};

use super::{teacher::TeacherMetadata, period::PeriodMetadata};


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
        first_name: String,
        last_name: String,
        honorific: String,
        pronouns: PronounSetInput,
    ) -> juniper::FieldResult<TeacherMetadata> {
        let mut db_context_mut = ctx.get_db_mut().await;
    
        add_teacher
            ::add_teacher(&mut db_context_mut.client, &first_name, &last_name, &honorific, &pronouns)
            .await
            .map_err(IntoFieldError::into_field_error)
    }
    
    async fn update_teacher(
        ctx: &Context,
        id: TeacherId,
        first_name: Option<String>,
        last_name: Option<String>,
        honorific: Option<String>,
        pronouns: Option<PronounSetInput>,
    ) -> juniper::FieldResult<TeacherMetadata> {
        let mut db_context_mut = ctx.get_db_mut().await;

        update_teacher
            ::update_teacher(
                &mut db_context_mut.client,
                id,
                first_name.as_deref(),
                last_name.as_deref(),
                honorific.as_deref(),
                pronouns.as_ref(),
            )
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    #[graphql(arguments(
        id(),
        names(default = Vec::new()),
        fully_absent(),
    ),)]
    async fn update_teacher_absence(
        ctx: &Context,
        id: TeacherId,
        names: Vec<PeriodName>,
        fully_absent: bool,
    ) -> juniper::FieldResult<TeacherMetadata> {
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
        default_time: TimeRangeInput,
    ) -> juniper::FieldResult<PeriodMetadata> {
        let mut db_context_mut = ctx.get_db_mut().await;
        
        add_period
            ::add_period(&mut db_context_mut.client, &name, default_time)
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    async fn update_period(
        ctx: &Context,
        id: PeriodId,
        name: Option<String>,
        default_time: Option<TimeRangeInput>,
    ) -> juniper::FieldResult<PeriodMetadata> {
        let mut db_context_mut = ctx.get_db_mut().await;

        update_period
            ::update_period(&mut db_context_mut.client, id, name.as_deref(), default_time, (false, None))
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    async fn update_period_temp_time(
        ctx: &Context,
        id: PeriodId,
        temp_time: Option<TimeRangeInput>,
    ) -> juniper::FieldResult<PeriodMetadata> {
        let mut db_context_mut = ctx.get_db_mut().await;

        update_period
            ::update_period(&mut db_context_mut.client, id, None, None, (true, temp_time))
            .await
            .map_err(IntoFieldError::into_field_error)
    }

    async fn delete_period(
        ctx: &Context,
        id: PeriodId,
    ) -> juniper::FieldResult<bool> {
        let mut db_context_mut = ctx.get_db_mut().await;

        delete_period
            ::delete_period(&mut db_context_mut.client, id)
            .await
            .map_err(IntoFieldError::into_field_error)?;
        Ok(true)
    }

    async fn clear_absences(
        ctx: &Context,
    ) -> juniper::FieldResult<bool> {
        let mut db_context_mut = ctx.get_db_mut().await;

        clear_absences
            ::clear_absences(&mut db_context_mut.client)
            .await
            .map_err(IntoFieldError::into_field_error)?;
        Ok(true)
    }

    async fn clear_temp_times(
        ctx: &Context,
    ) -> juniper::FieldResult<bool> {
        let mut db_context_mut = ctx.get_db_mut().await;

        clear_temp_times
            ::clear_temp_times(&mut db_context_mut.client)
            .await
            .map_err(IntoFieldError::into_field_error)?;
        Ok(true)
    }
}