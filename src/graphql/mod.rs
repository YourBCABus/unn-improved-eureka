//! This module provides access to most of the structs/wrapper structs necessary for running and setting up GraphQL
//! For a blob import, it is encouraged to use [crate::graphql::prelude] or [crate::preludes::graphql] instead of
//! `crate::graphql` for code cleanliness and consistency.
//! 
//! 
//! 

pub mod structs;
pub mod prelude;

pub mod resolvers;

use tokio::sync::Mutex;

use super::database::DbContext;

use crate::preludes::{
    graphql::*,
    verification::AuthenticationMethods,
};

use std::{convert::Infallible, sync::Arc, ops::DerefMut};


/// A Schema alias type used by the GraphQLRequest handler to run a GraphQL query.
pub type Schema = GraphQLRoot<'static, QueryRoot, MutationRoot, NoSubscription<Context>>;

/// A GraphQL context composed of
/// - auth context
/// - db context
pub struct Context {
    /// Database shared context. See [DbContext] for details.
    pub db_context: Arc<Mutex<DbContext>>,
    /// Per-request auth context. See [AuthenticationMethods] for details.
    pub auth_context: AuthenticationMethods,
}

impl Context {
    /// Gets the MutexGuard for the db_context field.
    /// 
    /// Uses [tokio's Mutex][tokio::sync::Mutex] to retain the mutex across await points.
    pub async fn get_db_mut(&self) -> impl DerefMut<Target = DbContext> + '_ {
        self.db_context.lock().await
    }
}

impl juniper::Context for Context {}


/// What is essentially the linkage between [warp]'s Filters and [juniper]'s query execution.
/// - `schema` - Shared `improved-eureka` [Schema], optimally shared between requests by cloning the `Arc`.
/// - `db_ctx` - Shared postgres database [Context], also optimally shared between requests.
/// - `auth_ctx` - Authentication flags for each request. Generated at the beginning of every request.
/// - `req` - the opaque juniper type for a graphql request, deserialized from JSON
/// 
/// This function is only really supposed to be called at the end of a filter chain with and_then.
/// It should never fail, and especially never panic.
/// 
pub async fn exec_graphql(
    schema: Arc<Schema>,
    db_ctx: Arc<Mutex<DbContext>>,
    auth_ctx: AuthenticationMethods,
    req: GraphQLRequest,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    let context = Context { db_context: db_ctx, auth_context: auth_ctx };
    let res = req
        .execute(
            &schema,
            &context,
        ).await;
    
    match serde_json::to_string(&res) {
        Ok(json) => Ok(Box::new(json)),
        Err(err) => Ok(Box::new(
            warp::reply::with_status(err.to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR)
        )),
    }
}
