use async_graphql::Context;
use uuid::Uuid;


use crate::graphql::resolvers::mutation::ensure_auth;
use crate::graphql::resolvers::{get_db, run_query};
use crate::graphql::req_id;

use crate::types::Teacher;

use async_graphql::Result as GraphQlResult;

pub async fn add_teacher_associated_oauth(
    ctx: &Context<'_>,
    id: Uuid,
    provider: String,
    sub: String,
) -> GraphQlResult<Teacher> {
    use crate::database::prepared::teacher::add_teacher_oauth as add_teacher_associated_oauth_in_db;
    use crate::database::prepared::teacher::get_teacher as get_teacher_from_db;

    let mut db_conn = get_db!(ctx);
    ensure_auth!(ctx, db: &mut db_conn);

    run_query!(
        db_conn.add_teacher_associated_oauth_in_db(id, provider.clone(), sub)
        else (req_id(ctx)) "Failed to add {provider} oauth for teacher {id}: {}"
    )?;
    run_query!(
        db_conn.get_teacher_from_db(id)
        else (req_id(ctx)) "Failed to refetch updated teacher {id}: {}"
    )
}

pub async fn remove_teacher_associated_oauth(
    ctx: &Context<'_>,
    id: Uuid,
    provider: String,
) -> GraphQlResult<Teacher> {
    use crate::database::prepared::teacher::remove_teacher_oauth as remove_teacher_associated_oauth_in_db;
    use crate::database::prepared::teacher::get_teacher as get_teacher_from_db;

    let mut db_conn = get_db!(ctx);
    ensure_auth!(ctx, db: &mut db_conn);

    run_query!(
        db_conn.remove_teacher_associated_oauth_in_db(id, provider.clone())
        else (req_id(ctx)) "Failed to remove {provider} oauth for teacher {id}: {}"
    )?;
    run_query!(
        db_conn.get_teacher_from_db(id)
        else (req_id(ctx)) "Failed to refetch updated teacher {id}: {}"
    )
}
