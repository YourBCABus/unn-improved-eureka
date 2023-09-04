//! This module contains solely the [MutationRoot] struct.
//! It only exists for organizational purposes.

// mod add_teacher;
// mod update_teacher;
// mod delete_teacher;

// mod add_period;
// mod update_period;
// mod delete_period;


// mod update_teacher_absence;
// mod clear_absences;
// mod clear_temp_times;

// use crate::graphql_types::{
//     scalars::teacher::*,
//     scalars::period::*,
//     juniper_types::IntoFieldError,
//     *, inputs::{TimeRangeInput, PronounSetInput},
// };

// use super::{teacher::TeacherMetadata, period::PeriodMetadata};

use async_graphql::{
    Object,

    Context,
    Result as GraphQlResult,
    Error as GraphQlError,
};
use uuid::Uuid;

use crate::{
    types::Teacher,
    state::AppState,
};

use crate::graphql::structs::{
    GraphQlTeacherName,
    GraphQlPronounSet,
};

/// This is a memberless struct implementing all the mutations for `improved-eureka`.
/// This includes:
/// - `add_teacher(name?, id?) -> Teacher`
/// - `delete_teacher() -> bool`
/// 
/// Generally, it will only be used as part of a [schema][super::Schema].
#[derive(Debug, Clone, Copy)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_teacher(
        &self,
        ctx_accessor: &Context<'_>,
        name: GraphQlTeacherName,
        pronouns: GraphQlPronounSet,
    ) -> GraphQlResult<Teacher> {
        use crate::database::prepared::teacher::create_teacher as add_teacher_to_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        let teacher = Teacher::new(
            uuid::Uuid::new_v4(),
            name.into(),
            pronouns.into(),
        );

        add_teacher_to_db(&mut db_conn, teacher)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }
    
    async fn update_teacher_name(
        &self,
        ctx_accessor: &Context<'_>,
        id: Uuid,
        name: GraphQlTeacherName,
    ) -> GraphQlResult<Teacher> {
        use crate::database::prepared::teacher::update_teacher_name as update_teacher_name_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        update_teacher_name_in_db(&mut db_conn, id, name.into())
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }

    async fn update_teacher_pronouns(
        &self,
        ctx_accessor: &Context<'_>,
        id: Uuid,
        pronouns: GraphQlPronounSet,
    ) -> GraphQlResult<Teacher> {
        use crate::database::prepared::teacher::update_teacher_pronouns as update_teacher_pronouns_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        update_teacher_pronouns_in_db(&mut db_conn, id, pronouns.into())
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }

    // #[graphql(arguments(
    //     id(),
    //     names(default = Vec::new()),
    //     fully_absent(),
    // ),)]
    // async fn update_teacher_absence(
    //     ctx: &Context,
    //     id: TeacherId,
    //     names: Vec<PeriodName>,
    //     fully_absent: bool,
    // ) -> juniper::FieldResult<TeacherMetadata> {
    //     let mut db_context_mut = ctx.get_db_mut().await;

    //     update_teacher_absence
    //         ::update_teacher_absence(&mut db_context_mut.client, id, &names, fully_absent)
    //         .await
    //         .map_err(IntoFieldError::into_field_error)
    // }

    // async fn delete_teacher(
    //     ctx: &Context,
    //     id: TeacherId,
    // ) -> juniper::FieldResult<bool> {
    //     let mut db_context_mut = ctx.get_db_mut().await;

    //     delete_teacher
    //         ::delete_teacher(&mut db_context_mut.client, id)
    //         .await
    //         .map_err(IntoFieldError::into_field_error)?;
    //     Ok(true)
    // }


    // async fn add_period(
    //     ctx: &Context,
    //     name: String,
    //     default_time: TimeRangeInput,
    // ) -> juniper::FieldResult<PeriodMetadata> {
    //     let mut db_context_mut = ctx.get_db_mut().await;
        
    //     add_period
    //         ::add_period(&mut db_context_mut.client, &name, default_time)
    //         .await
    //         .map_err(IntoFieldError::into_field_error)
    // }

    // async fn update_period(
    //     ctx: &Context,
    //     id: PeriodId,
    //     name: Option<String>,
    //     default_time: Option<TimeRangeInput>,
    // ) -> juniper::FieldResult<PeriodMetadata> {
    //     let mut db_context_mut = ctx.get_db_mut().await;

    //     update_period
    //         ::update_period(&mut db_context_mut.client, id, name.as_deref(), default_time, (false, None))
    //         .await
    //         .map_err(IntoFieldError::into_field_error)
    // }

    // async fn update_period_temp_time(
    //     ctx: &Context,
    //     id: PeriodId,
    //     temp_time: Option<TimeRangeInput>,
    // ) -> juniper::FieldResult<PeriodMetadata> {
    //     let mut db_context_mut = ctx.get_db_mut().await;

    //     update_period
    //         ::update_period(&mut db_context_mut.client, id, None, None, (true, temp_time))
    //         .await
    //         .map_err(IntoFieldError::into_field_error)
    // }

    // async fn delete_period(
    //     ctx: &Context,
    //     id: PeriodId,
    // ) -> juniper::FieldResult<bool> {
    //     let mut db_context_mut = ctx.get_db_mut().await;

    //     delete_period
    //         ::delete_period(&mut db_context_mut.client, id)
    //         .await
    //         .map_err(IntoFieldError::into_field_error)?;
    //     Ok(true)
    // }

    // async fn clear_absences(
    //     ctx: &Context,
    // ) -> juniper::FieldResult<bool> {
    //     let mut db_context_mut = ctx.get_db_mut().await;

    //     clear_absences
    //         ::clear_absences(&mut db_context_mut.client)
    //         .await
    //         .map_err(IntoFieldError::into_field_error)?;
    //     Ok(true)
    // }

    // async fn clear_temp_times(
    //     ctx: &Context,
    // ) -> juniper::FieldResult<bool> {
    //     let mut db_context_mut = ctx.get_db_mut().await;

    //     clear_temp_times
    //         ::clear_temp_times(&mut db_context_mut.client)
    //         .await
    //         .map_err(IntoFieldError::into_field_error)?;
    //     Ok(true)
    // }
}