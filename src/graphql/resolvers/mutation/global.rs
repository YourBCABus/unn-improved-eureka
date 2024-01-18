use async_graphql::Context;

use crate::graphql::resolvers::mutation::ensure_auth;
use crate::graphql::resolvers::{get_db, run_query};
use crate::graphql::req_id;

use async_graphql::Result as GraphQlResult;

pub async fn set_spreadsheet_id(
    ctx: &Context<'_>,
    id: String,
) -> GraphQlResult<bool> {
    use crate::database::prepared::config::set_sheet_id as set_sheet_id_in_db;

    let mut db_conn = get_db!(ctx);
    ensure_auth!(ctx, db: &mut db_conn);

    run_query!(
        db_conn.set_sheet_id_in_db(&id)
        else (req_id(ctx)) "Database error: {}"
    )?;
    
    Ok(true)
}

pub async fn set_report_to(
    ctx: &Context<'_>,
    report_to: String,
) -> GraphQlResult<bool> {
    use crate::database::prepared::config::set_report_to as set_report_to_in_db;

    let mut db_conn = get_db!(ctx);
    ensure_auth!(ctx, db: &mut db_conn);

    run_query!(
        db_conn.set_report_to_in_db(&report_to)
        else (req_id(ctx)) "Database error: {}"
    )?;
    
    Ok(true)
}
