//! This module provides access to most of the structs/wrapper structs necessary for running and setting up GraphQL
//! For a blob import, it is encouraged to use [`crate::graphql::prelude`] or
//! [`crate::preludes::graphql`] instead of `crate::graphql` for code cleanliness and consistency.
//! 
//! 
//! 

pub mod resolvers;


use cfg::graphql_complexity_limit_usize_panic;
use server::AppState;

use self::{
    resolvers::query::QueryRoot,
    resolvers::mutation::MutationRoot,
};

use async_graphql::{
    Schema as GenericSchema,
    EmptySubscription,
};

/// A Schema alias type used by the `GraphQLRequest` handler to run a GraphQL query.
pub type Schema = GenericSchema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn schema(app_state: AppState) -> Schema {
    GenericSchema::build(
        QueryRoot,
        MutationRoot,
        EmptySubscription,
    )
        .data(app_state)
        .limit_complexity(graphql_complexity_limit_usize_panic().await)
        .finish()
}

pub fn save_schema(schema: &Schema, path: &str) {
    if let Err(err) = std::fs::write(path, schema.sdl()) {
        logging::warn!("Schema failed to save to {path}: {err}");
    } else {
        logging::info!("Schema saved to {path}");
    }
}
