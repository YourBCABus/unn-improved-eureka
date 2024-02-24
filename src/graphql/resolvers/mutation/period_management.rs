use async_graphql::Context;
use uuid::Uuid;


use crate::graphql::resolvers::{get_db, run_query};
use crate::graphql::req_id;

use crate::graphql::structs::TimeRangeInput;
use crate::types::Period;

use async_graphql::Result as GraphQlResult;

pub async fn add_period(
    ctx: &Context<'_>,

    name: String,
    default_time: TimeRangeInput,
) -> GraphQlResult<Period> {
    use crate::database::prepared::period::create_period as add_period_to_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.add_period_to_db(&name, [default_time.start, default_time.end])
        else (req_id(ctx)) "Database error: {}"
    )
}

pub async fn update_period_name(
    ctx: &Context<'_>,

    id: Uuid,
    name: String,
) -> GraphQlResult<Period> {
    use crate::database::prepared::period::update_period_name as update_period_name_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.update_period_name_in_db(id, &name)
        else (req_id(ctx)) "Database error: {}"
    )
}

pub async fn update_period_time(
    ctx: &Context<'_>,

    id: Uuid,
    time: TimeRangeInput,
) -> GraphQlResult<Period> {
    use crate::database::prepared::period::update_period_time as update_period_time_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.update_period_time_in_db(id, [time.start, time.end])
        else (req_id(ctx)) "Failed to get : {}"
    )
}
pub async fn set_period_temp_time(
    ctx: &Context<'_>,

    id: Uuid,
    temp_time: TimeRangeInput,
) -> GraphQlResult<Period> {
    use crate::database::prepared::period::set_period_temp_time as set_period_temp_time_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.set_period_temp_time_in_db(id, [temp_time.start, temp_time.end])
        else (req_id(ctx)) "Database error: {}"
    )
}
pub async fn clear_period_temp_time(
    ctx: &Context<'_>,

    id: Uuid,
) -> GraphQlResult<Period> {
    use crate::database::prepared::period::clear_period_temp_time as clear_period_temp_time_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.clear_period_temp_time_in_db(id)
        else (req_id(ctx)) "Database error: {}"
    )
}
pub async fn clear_all_temp_times(
    ctx: &Context<'_>,
) -> GraphQlResult<()> {
    use crate::database::prepared::period::flush_all_temp_times as clear_all_temp_times_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.clear_all_temp_times_in_db()
        else (req_id(ctx)) "Database error: {}"
    )
}


