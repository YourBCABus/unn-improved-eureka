mod futures;
mod global;
mod oauth;
mod period_management;
mod teacher_management;
mod attribs;

use async_graphql::{
    Object,

    Context,
    Result as GraphQlResult,
};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::database::prepared::teacher::get_teacher;
use crate::graphql::req_id;
use crate::types::{ Teacher, Period };

use crate::graphql::structs::{
    GraphQlTeacherName,
    GraphQlPronounSet, TimeRangeInput,
};

use super::{ get_db, run_query, ensure_auth };

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
        ctx: &Context<'_>,
        name: GraphQlTeacherName,
        pronouns: GraphQlPronounSet,
    ) -> GraphQlResult<Teacher> {
        ensure_auth!(ctx, [create_teacher]);

        teacher_management::add_teacher(ctx, name, pronouns).await
    }
    
    async fn update_teacher_name(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        name: GraphQlTeacherName,
    ) -> GraphQlResult<Teacher> {
        ensure_auth!(ctx, [write_teacher_name]);

        teacher_management::update_teacher_name(ctx, id, name).await
    }

    async fn update_teacher_pronouns(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        pronouns: GraphQlPronounSet,
    ) -> GraphQlResult<Teacher> {
        ensure_auth!(ctx, [write_teacher_pronouns]);

        teacher_management::update_teacher_pronouns(ctx, id, pronouns).await
    }
    
    async fn update_teacher_absence(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        periods: Vec<Uuid>,
        fully_absent: bool,
    ) -> GraphQlResult<Teacher> {
        use crate::database::prepared::absences::update_absences_for_teacher as update_absences_for_teacher_in_db;

        let mut db_conn = get_db!(ctx);
        ensure_auth!(ctx, [write_teacher_absence]);

        run_query!(
            db_conn.update_absences_for_teacher_in_db(id, &periods, fully_absent)
            else (req_id(ctx)) "Failed to update absence for teacher {id}: {}"
        )?;
        run_query!(
            db_conn.get_teacher(id)
            else (req_id(ctx)) "Failed to refetch updated teacher {id}: {}"
        )
    }

    async fn add_teacher_associated_oauth(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        provider: String,
        sub: String,
    ) -> GraphQlResult<Teacher> {
        ensure_auth!(ctx, [experimental, admin]);

        oauth::add_teacher_associated_oauth(ctx, id, provider, sub).await
    }

    async fn remove_teacher_associated_oauth(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        provider: String,
    ) -> GraphQlResult<Teacher> {
        ensure_auth!(ctx, [experimental, admin]);

        oauth::remove_teacher_associated_oauth(ctx, id, provider).await
    }


    #[allow(clippy::too_many_arguments)]
    async fn set_teacher_future_absence(
        &self,
        ctx: &Context<'_>,
        start: NaiveDate,
        end: Option<NaiveDate>,
        id: Uuid,
        periods: Vec<Uuid>,
        fully_absent: bool,
        comment: Option<String>,
    ) -> GraphQlResult<bool> {
        ensure_auth!(ctx, [experimental, admin]);

        futures::set_teacher_future_absence(
            ctx,
            start, end, id,
            periods, fully_absent, comment,
        ).await
    }

    async fn clear_teacher_future_absence(
        &self,
        ctx: &Context<'_>,
        start: NaiveDate,
        end: Option<NaiveDate>,
        id: Uuid,
    ) -> GraphQlResult<bool> {
        ensure_auth!(ctx, [experimental, admin]);

        futures::clear_teacher_future_absence(
            ctx,
            start, end, id,
        ).await
    }

    async fn sync_and_flush_futures(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQlResult<bool> {
        futures::sync_and_flush_futures(ctx).await
    }

    // Global config/settings
    async fn set_spreadsheet_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> GraphQlResult<bool> {
        ensure_auth!(ctx, [write_config]);

        global::set_spreadsheet_id(ctx, id).await
    }
    async fn set_report_to(
        &self,
        ctx: &Context<'_>,
        report_to: String,
    ) -> GraphQlResult<bool> {
        ensure_auth!(ctx, [write_config]);

        global::set_report_to(ctx, report_to).await
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
        ctx: &Context<'_>,

        name: String,
        default_time: TimeRangeInput,
    ) -> GraphQlResult<Period> {
        ensure_auth!(ctx, [create_period]);

        period_management::add_period(ctx, name, default_time).await
    }

    async fn update_period_name(
        &self,
        ctx: &Context<'_>,

        id: Uuid,
        name: String,
    ) -> GraphQlResult<Period> {
        ensure_auth!(ctx, [write_period_name]);

        period_management::update_period_name(ctx, id, name).await
    }
    async fn update_period_time(
        &self,
        ctx: &Context<'_>,

        id: Uuid,
        time: TimeRangeInput,
    ) -> GraphQlResult<Period> {
        ensure_auth!(ctx, [write_period_time]);

        period_management::update_period_time(ctx, id, time).await
    }
    async fn set_period_temp_time(
        &self,
        ctx: &Context<'_>,

        id: Uuid,
        temp_time: TimeRangeInput,
    ) -> GraphQlResult<Period> {
        ensure_auth!(ctx, [write_period_temp_time]);

        period_management::set_period_temp_time(ctx, id, temp_time).await
    }
    async fn clear_period_temp_time(
        &self,
        ctx: &Context<'_>,

        id: Uuid,
    ) -> GraphQlResult<Period> {
        ensure_auth!(ctx, [write_period_temp_time]);

        period_management::clear_period_temp_time(ctx, id).await
    }
    async fn clear_all_temp_times(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQlResult<bool> {
        ensure_auth!(ctx, [write_period_temp_time]);

        period_management::clear_all_temp_times(ctx).await?;
        Ok(true)
    }

    async fn clear_metrics(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQlResult<String> {
        ensure_auth!(ctx, [admin]);

        let metrics = ctx.data::<crate::state::AppState>()?.metrics();

        if metrics.clear(None).await.is_ok() {
            Ok("Metrics cleared".to_string())
        } else {
            Err(async_graphql::Error::new("Failed to clear metrics"))
        }
    }

    async fn attribs(&self) -> attribs::AttribMutationRoot {
        attribs::AttribMutationRoot
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
