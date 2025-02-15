use async_graphql::Context;
use graphql::{ get_school_id, req_id };
use uuid::Uuid;


use db::{ get_db, run_query };

// use crate::graphql::structs::TimeRangeInput;
use crate::types::{Period, TimeRange};

use async_graphql::Result as GraphQlResult;

pub async fn add_period(
    ctx: &Context<'_>,

    name: String,
    default_time: TimeRange,
    is_temp: bool,
) -> GraphQlResult<Period> {
    use crate::queries::period::create_period as add_period_to_db;

    let mut db_conn = get_db!(ctx);
    let school_id = get_school_id(ctx).await?;

    run_query!(
        db_conn.add_period_to_db(school_id, &name, default_time, is_temp)
        else (req_id(ctx)) "Database error: {}"
    )
}

pub async fn update_period_name(
    ctx: &Context<'_>,

    id: Uuid,
    name: String,
) -> GraphQlResult<Period> {
    use crate::queries::period::update_period_name as update_period_name_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.update_period_name_in_db(id, &name)
        else (req_id(ctx)) "Database error: {}"
    )
}

pub async fn update_period_time(
    ctx: &Context<'_>,

    id: Uuid,
    time: TimeRange,
) -> GraphQlResult<Period> {
    use crate::queries::period::update_period_time as update_period_time_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.update_period_time_in_db(id, time)
        else (req_id(ctx)) "Failed to get : {}"
    )
}
pub async fn set_period_temp_time(
    ctx: &Context<'_>,

    id: Uuid,
    temp_time: TimeRange,
) -> GraphQlResult<Period> {
    use crate::queries::period::set_period_temp_time as set_period_temp_time_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.set_period_temp_time_in_db(id, temp_time)
        else (req_id(ctx)) "Database error: {}"
    )
}
pub async fn clear_period_temp_time(
    ctx: &Context<'_>,

    id: Uuid,
) -> GraphQlResult<Period> {
    use crate::queries::period::clear_period_temp_time as clear_period_temp_time_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.clear_period_temp_time_in_db(id)
        else (req_id(ctx)) "Database error: {}"
    )
}
pub async fn clear_all_temp_times(
    ctx: &Context<'_>,
) -> GraphQlResult<()> {
    use crate::queries::period::flush_all_temp_times as clear_all_temp_times_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.clear_all_temp_times_in_db()
        else (req_id(ctx)) "Database error: {}"
    )
}

pub async fn clear_all_temp_periods(
    ctx: &Context<'_>,
) -> GraphQlResult<()> {
    use crate::queries::period::flush_all_temp_periods as clear_all_temp_periods_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.clear_all_temp_periods_in_db()
        else (req_id(ctx)) "Database error: {}"
    )
}

pub async fn delete_period(
    ctx: &Context<'_>,

    id: Uuid,
) -> GraphQlResult<u64> {
    use crate::queries::period::delete_period as delete_period_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.delete_period_in_db(id)
        else (req_id(ctx)) "Database error: {}"
    )
}
