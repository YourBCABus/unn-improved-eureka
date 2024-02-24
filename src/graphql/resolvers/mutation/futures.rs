use async_graphql::Context;
use chrono::NaiveDate;
use uuid::Uuid;


use crate::graphql::resolvers::{get_db, run_query};
use crate::graphql::req_id;


use async_graphql::Result as GraphQlResult;

#[allow(clippy::too_many_arguments)]
pub async fn set_teacher_future_absence(
    ctx: &Context<'_>,
    start: NaiveDate,
    end: Option<NaiveDate>,
    id: Uuid,
    periods: Vec<Uuid>,
    fully_absent: bool,
    comment: Option<String>,
) -> GraphQlResult<bool> {
    use crate::database::prepared::future_absences::set_future_day as set_future_absence_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.set_future_absence_in_db(
            start, end.unwrap_or(start), id,
            &periods, fully_absent, comment,
        )
        else (req_id(ctx)) "Failed to set future absence in the database for teacher {id}: {}"
    )?;
    
    Ok(true)
}

pub async fn clear_teacher_future_absence(
    ctx: &Context<'_>,
    start: NaiveDate,
    end: Option<NaiveDate>,
    id: Uuid,
) -> GraphQlResult<bool> {
    use crate::database::prepared::future_absences::clear_future_day as clear_future_absence_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.clear_future_absence_in_db(start, end.unwrap_or(start), id)
        else (req_id(ctx)) "Failed to clear future absence in the database for teacher {id}: {}"
    )?;
    
    Ok(true)
}

pub async fn sync_and_flush_futures(
    ctx: &Context<'_>,
) -> GraphQlResult<bool> {
    use crate::database::prepared::future_absences::flush_today as sync_and_flush_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.sync_and_flush_in_db()
        else (req_id(ctx)) "Failed in syncing and flushing futures at {}: {}", chrono::Utc::now().to_rfc2822()
    )?;
    
    Ok(true)
}
