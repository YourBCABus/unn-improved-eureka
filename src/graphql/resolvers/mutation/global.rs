use async_graphql::Context;
use graphql::{get_school_id, req_id};

use db::{ get_db, run_query };

use async_graphql::Result as GraphQlResult;

pub async fn set_spreadsheet_id(
    ctx: &Context<'_>,
    id: String,
) -> GraphQlResult<bool> {
    use crate::queries::config::set_sheet_id as set_sheet_id_in_db;

    let mut db_conn = get_db!(ctx);
    let school_id = get_school_id(ctx).await?;

    run_query!(
        db_conn.set_sheet_id_in_db(school_id, &id)
        else (req_id(ctx)) "Database error: {}"
    )?;
    
    Ok(true)
}

pub async fn set_report_to(
    ctx: &Context<'_>,
    report_to: String,
) -> GraphQlResult<bool> {
    use crate::queries::config::set_report_to as set_report_to_in_db;

    let mut db_conn = get_db!(ctx);
    let school_id = get_school_id(ctx).await?;

    run_query!(
        db_conn.set_report_to_in_db(school_id, &report_to)
        else (req_id(ctx)) "Database error: {}"
    )?;
    
    Ok(true)
}
