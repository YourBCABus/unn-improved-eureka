use std::collections::HashMap;

use async_graphql::Context;
use sqlx::types::JsonValue;


use crate::graphql::resolvers::attribs::Attribs;
use crate::graphql::resolvers::{ensure_auth, get_db, run_query};
use crate::graphql::req_id;


use async_graphql::Result as GraphQlResult;

pub type AttribsInner = HashMap<String, JsonValue>;

#[derive(Debug, Clone)]
pub struct AttribMutationRoot;

#[derive(Debug, Clone)]
pub struct RawAttribMutationRoot;

#[async_graphql::Object]
impl RawAttribMutationRoot {
    async fn set_key(
        &self,
        ctx: &Context<'_>,
        key: String,
        new_value: JsonValue,
    ) -> GraphQlResult<Attribs> {
        set_single_attrib(ctx, &key, new_value).await
    }

    async fn clear_key(
        &self,
        ctx: &Context<'_>,
        key: String,
    ) -> GraphQlResult<Attribs> {
        clear_single_attrib(ctx, &key).await
    }

    async fn set_attribs(
        &self,
        ctx: &Context<'_>,
        attribs: AttribsInner,
    ) -> GraphQlResult<Attribs> {
        set_attribs(ctx, attribs).await
    }
}

#[async_graphql::Object]
impl AttribMutationRoot {
    async fn raw(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQlResult<RawAttribMutationRoot> {
        ensure_auth!(ctx, [admin]);
        Ok(RawAttribMutationRoot)
    }

    async fn set_support_form_url(
        &self,
        ctx: &Context<'_>,
        new_value: JsonValue,
    ) -> GraphQlResult<Attribs> {
        ensure_auth!(ctx, [write_config]);
        set_single_attrib(ctx, "supportFormUrl", new_value).await
    }
}


pub async fn set_single_attrib(
    ctx: &Context<'_>,
    key: &str,
    new_value: JsonValue,
) -> GraphQlResult<Attribs> {
    use crate::database::prepared::config::{
        get_attribs as get_attribs_db,
        set_single_attrib as set_single_attrib_db,
    };

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.set_single_attrib_db(key, &new_value)
        else (req_id(ctx)) "Database error: {}"
    )?;

    let map = run_query!(
        db_conn.get_attribs_db()
        else (req_id(ctx)) "Database error: {}"
    )?;

    Ok(Attribs(map))
}

pub async fn clear_single_attrib(
    ctx: &Context<'_>,
    key: &str,
) -> GraphQlResult<Attribs> {
    use crate::database::prepared::config::{
        get_attribs as get_attribs_db,
        clear_single_attrib as clear_single_attrib_db,
    };

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.clear_single_attrib_db(key)
        else (req_id(ctx)) "Database error: {}"
    )?;

    let map = run_query!(
        db_conn.get_attribs_db()
        else (req_id(ctx)) "Database error: {}"
    )?;

    Ok(Attribs(map))
}

pub async fn set_attribs(
    ctx: &Context<'_>,
    attribs: AttribsInner,
) -> GraphQlResult<Attribs> {
    use crate::database::prepared::config::{
        get_attribs as get_attribs_db,
        set_attribs as set_attribs_db,
    };

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.set_attribs_db(attribs)
        else (req_id(ctx)) "Database error: {}"
    )?;

    let map = run_query!(
        db_conn.get_attribs_db()
        else (req_id(ctx)) "Database error: {}"
    )?;

    Ok(Attribs(map))
}


