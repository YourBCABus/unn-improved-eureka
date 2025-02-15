//! This module contains solely the [`QueryRoot`] struct.
//! It only exists for organizational purposes.

// mod get_teacher;
// mod all_teachers;
// mod all_periods;

use crate::queries::privileges::get_privileges;

use crate::types::Privileges;
use crate::types::Teacher;
use crate::types::Period;
use crate::types::PackedAbsenceState;
use crate::types::TeacherAbsenceStateList;
use super::sparse_metrics_view::GraphQlSparseMetricsView;

use db::{ get_db, run_query };
use auth::ensure_auth;

use async_graphql::{
    Object,

    Context,
    Result as GraphQlResult,
    Error as GraphQlError,
};


use chrono::NaiveDate;
use graphql::get_school_id;
use graphql::req_id;
use uuid::Uuid;


/// This is a memberless struct implementing all the queries for `improved-eureka`.
/// This includes:
/// - `get_teacher(name?, id?) -> Teacher`
/// - `all_teachers() -> Teacher[]`
/// 
/// - `all_periods() -> Period[]`
/// - `get_period(name?, id?) -> Period`
/// 
/// Generally, it will only be used as part of a [schema][super::Schema].
#[derive(Debug, Clone, Copy)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_teacher(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Id of teacher")] id: Uuid,
    ) -> GraphQlResult<Teacher> {
        use crate::queries::teacher::get_teacher as get_teacher_from_db;

        ensure_auth!(ctx, [read_teacher]);

        let mut db_conn = get_db!(ctx);

        get_teacher_from_db(&mut db_conn, id)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Failed to get teacher from database {e}"))
            })
    }

    async fn all_teachers(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQlResult<Vec<Teacher>> {
        use crate::queries::teacher::get_all_teachers as get_all_teachers_from_db;        

        ensure_auth!(ctx, [read_teacher]);

        let mut db_conn = get_db!(ctx);
        let school_id = graphql::get_school_id(ctx).await?;

        get_all_teachers_from_db(&mut db_conn, school_id)
            .await
            .map_err(|e| {
                if matches!(e, sqlx::Error::RowNotFound) {
                    GraphQlError::new_with_source("Teacher not found")
                } else {
                    let e = e.to_string();
                    GraphQlError::new(format!("Failed to get teacher from database {e}"))
                }
            })
    }

    async fn get_teacher_by_oauth(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Provider of OAuth")] provider: String,
        #[graphql(desc = "Sub of OAuth")] sub: String,
    ) -> GraphQlResult<Teacher> {
        use crate::queries::teacher::get_teacher_by_oauth as get_teacher_by_oauth_from_db;

        ensure_auth!(ctx, [read_teacher, admin, experimental]);

        let mut db_conn = get_db!(ctx);

        run_query!(
            db_conn.get_teacher_by_oauth_from_db(provider, sub)
            else (req_id(ctx)) "Failed to get teacher from database: {}"
        )
    }

    async fn get_teacher_futures(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        start: NaiveDate,
        end: NaiveDate,
        #[graphql(desc = "Provider of OAuth")] provider: String,
        #[graphql(desc = "Sub of OAuth")] sub: String,
    ) -> GraphQlResult<Vec<PackedAbsenceState>> {
        use crate::queries::future_absences::get_future_days_for_teacher as get_futures_from_db;
        use crate::queries::teacher::check_teacher_oauth as check_oauth_db;

        ensure_auth!(ctx, [read_teacher, admin, experimental]);

        let mut db_conn = get_db!(ctx);
        let school_id = get_school_id(ctx).await?;

        let oauth_res = run_query!(
            db_conn.check_oauth_db(id, provider, sub)
            else (req_id(ctx)) "Not permitted to access this resource {:.0}"
        )?;
        if !oauth_res {
            return Err(GraphQlError::new("Not permitted to access this resource"));
        }

        run_query!(
            db_conn.get_futures_from_db(school_id, id, start, end)
            else (req_id(ctx)) "Failed to get teacher future absence data from database: {}"
        )
    }
    async fn get_all_teacher_futures(
        &self,
        ctx: &Context<'_>,
        start: NaiveDate,
        end: NaiveDate,
        #[graphql(desc = "Provider of OAuth")] provider: String,
        #[graphql(desc = "Sub of OAuth")] sub: String,
    ) -> GraphQlResult<Vec<TeacherAbsenceStateList>> {
        use crate::queries::future_absences::get_all_future_days as get_all_futures_from_db;
        use crate::queries::teacher::get_teacher_by_oauth as get_teacher_db;

        ensure_auth!(ctx, [read_teacher, admin, experimental]);

        let mut db_conn = get_db!(ctx);
        let school_id = get_school_id(ctx).await?;

        let teacher = run_query!(
            db_conn.get_teacher_db(provider.clone(), sub.clone())
            else (req_id(ctx)) "This oauth user doesn't exist {:.0}"
        )?;
        let teacher_perms = run_query!(
            db_conn.get_privileges(teacher.get_id())
            else (req_id(ctx)) "Not permitted to access this resource {:.0}"
        )?;

        if !teacher_perms.secretary && !teacher_perms.admin {
            return Err(GraphQlError::new("Not permitted to access this resource"));
        }

        run_query!(
            db_conn.get_all_futures_from_db(school_id, start, end)
            else (req_id(ctx)) "Failed to get teacher absence data from database: {}"
        )
    }

    async fn all_periods(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQlResult<Vec<Period>> {
        use crate::queries::period::get_all_periods as get_all_periods_from_db;        

        ensure_auth!(ctx, [read_period]);

        let mut db_conn = get_db!(ctx);
        let school_id = get_school_id(ctx).await?;

        get_all_periods_from_db(&mut db_conn, school_id)
            .await
            .map_err(|e| {
                if matches!(e, sqlx::Error::RowNotFound) {
                    GraphQlError::new_with_source("Period not found")
                } else {
                    let e = e.to_string();
                    GraphQlError::new(format!("Failed to get period from database {e}"))
                }
            })
    }




    async fn get_privs(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Provider of OAuth")] provider: String,
        #[graphql(desc = "Sub of OAuth")] sub: String,
    ) -> GraphQlResult<Privileges> {
        use crate::queries::privileges::get_privileges as get_privs_from_db;
        use crate::queries::teacher::get_teacher_by_oauth as get_teacher_db;

        ensure_auth!(ctx, [read_teacher, admin, experimental]);

        let mut db_conn = get_db!(ctx);

        let teacher = run_query!(
            db_conn.get_teacher_db(provider.clone(), sub.clone())
            else (req_id(ctx)) "This oauth user doesn't exist {:.0}"
        )?;
        
        run_query!(
            db_conn.get_privs_from_db(teacher.get_id())
            else (req_id(ctx)) "Failed to get permissions for oauth user: {}"
        )
    }

    async fn curr_spreadsheet_id(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQlResult<String> {
        use crate::queries::config::get_sheet_id as get_sheet_id_from_db;

        ensure_auth!(ctx, [read_period, read_teacher, read_teacher_name, read_teacher_absence, read_teacher_pronouns]);

        let mut db_conn = get_db!(ctx);
        let school_id = get_school_id(ctx).await?;

        run_query!(
            db_conn.get_sheet_id_from_db(school_id)
            else (req_id(ctx)) "Failed to get spreadsheet id from database: {}"
        )
    }

    async fn curr_report_to(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQlResult<String> {
        use crate::queries::config::get_report_to as get_report_to_from_db;

        ensure_auth!(ctx, [read_teacher, read_period]);

        let mut db_conn = get_db!(ctx);
        let school_id = get_school_id(ctx).await?;

        run_query!(
            db_conn.get_report_to_from_db(school_id)
            else (req_id(ctx)) "Failed to get \"report to\" location from database: {}"
        )
    }

    async fn get_metrics(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQlResult<GraphQlSparseMetricsView> {
        use super::sparse_metrics_view::{
            find_buckets_params_from_lookahead,
            buckets_valid,
        };
        let (step, range) = find_buckets_params_from_lookahead(&ctx.look_ahead())
            .unwrap_or((1.0, 0.0..1.0));

        buckets_valid(range.clone(), step)?;

        let metrics = ctx.data::<server::AppState>()?.metrics();

        if let Ok(output) = metrics.read(None, (range, step)).await {
            Ok(output.into())
        } else {
            Err(GraphQlError::new("Failed to read metrics"))
        }
    }

    async fn attribs(&self, ctx: &Context<'_>) -> GraphQlResult<super::attribs::Attribs> {
        use crate::queries::config::get_attribs as get_attribs_from_db;

        // ensure_auth!(ctx, [read_config]);
        let mut db_conn = get_db!(ctx);
        let school_id = get_school_id(ctx).await?;

        let attribs_inner = run_query!(
            db_conn.get_attribs_from_db(school_id)
            else (req_id(ctx)) "Failed to get attribs from database: {}"
        )?;

        Ok(super::attribs::Attribs(attribs_inner))
    }
}
