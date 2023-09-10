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

use crate::database::prepared::teacher::get_teacher;
use crate::types::Period;
use crate::{
    types::Teacher,
    state::AppState,
};

use crate::graphql::structs::{
    GraphQlTeacherName,
    GraphQlPronounSet, TimeRangeInput,
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
    
    async fn update_teacher_absence(
        &self,
        ctx_accessor: &Context<'_>,
        id: Uuid,
        periods: Vec<Uuid>,
        fully_absent: bool,
    ) -> GraphQlResult<Teacher> {
        use crate::database::prepared::absences::update_absences_for_teacher as update_absences_for_teacher_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        update_absences_for_teacher_in_db(&mut db_conn, id, &periods, fully_absent)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })?;
        
        get_teacher(&mut db_conn, id)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }

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


    async fn add_period(
        &self,
        ctx_accessor: &Context<'_>,

        name: String,
        default_time: TimeRangeInput,
    ) -> GraphQlResult<Period> {
        use crate::database::prepared::period::create_period as add_period_to_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        add_period_to_db(&mut db_conn, &name, [default_time.start, default_time.end])
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }

    async fn update_period_name(
        &self,
        ctx_accessor: &Context<'_>,

        id: Uuid,
        name: String,
    ) -> GraphQlResult<Period> {
        use crate::database::prepared::period::update_period_name as update_period_name_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        update_period_name_in_db(&mut db_conn, id, &name)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }
    async fn update_period_time(
        &self,
        ctx_accessor: &Context<'_>,

        id: Uuid,
        time: TimeRangeInput,
    ) -> GraphQlResult<Period> {
        use crate::database::prepared::period::update_period_time as update_period_time_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        update_period_time_in_db(&mut db_conn, id, [time.start, time.end])
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }
    async fn set_period_temp_time(
        &self,
        ctx_accessor: &Context<'_>,

        id: Uuid,
        temp_time: TimeRangeInput,
    ) -> GraphQlResult<Period> {
        use crate::database::prepared::period::set_period_temp_time as set_period_temp_time_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        set_period_temp_time_in_db(&mut db_conn, id, [temp_time.start, temp_time.end])
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }

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