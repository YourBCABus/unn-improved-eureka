//! This module provides access to most of the structs/wrapper structs necessary for running and setting up GraphQL
//! For a blob import, it is encouraged to use [crate::graphql::prelude] or [crate::preludes::graphql] instead of
//! `crate::graphql` for code cleanliness and consistency.
//! 
//! 
//! 

pub mod structs;

pub mod resolvers;


use crate::state::AppState;

use self::{
    resolvers::query::QueryRoot,
    resolvers::mutation::MutationRoot,
};

use async_graphql::{
    Schema as GenericSchema,
    EmptySubscription,
};





/// A Schema alias type used by the GraphQLRequest handler to run a GraphQL query.
pub type Schema = GenericSchema<QueryRoot, MutationRoot, EmptySubscription>;


// /// What is essentially the linkage between [actix_web]'s requests and [juniper]'s query execution.
// /// - `state` - `improved-eureka` [AppState] shared between requests.
// /// - `req` - the opaque juniper type for a graphql request, deserialized from JSON
// /// 
// /// This function is only really supposed to be called at the end of a filter chain with and_then.
// /// It should never fail, and especially never panic.
// /// 
// pub async fn exec_graphql(
//     state: AppState,
//     req: GraphQLRequest,
// ) -> impl Responder {
//     let res = req
//         .execute(
//             &state.schema,
//             &state,
//         ).await;

    
//     match serde_json::to_string(&res) {
//         Ok(json) => if res.is_ok() {
//             Ok(HttpResponse::Ok().body(json))
//         } else {
//             Ok(HttpResponse::BadRequest().body(json))
//         },
//         Err(err) => {
//             Ok(HttpResponse::InternalServerError().body(err.to_string()))
//         },
//     }
// }


pub fn schema(app_state: AppState) -> Schema {
    GenericSchema::build(
        QueryRoot,
        MutationRoot,
        EmptySubscription,
    )
        .data(app_state)
        .finish()
}

pub fn save_schema(schema: &Schema, path: &str) {
    if let Err(err) = std::fs::write(path, schema.sdl()) {
        crate::logging::warn!("Schema failed to save to {path}: {err}");
    } else {
        crate::logging::info!("Schema saved to {path}");
    }
}

fn req_id(context: &async_graphql::Context) -> uuid::Uuid {
    const HEADER_NAME: &str = "internal-request-id";

    if let Some(id) = context.insert_http_header(HEADER_NAME, "") {
        let id = match id.to_str() {
            Ok(id) => match uuid::Uuid::parse_str(id) {
                Ok(id) => id,
                Err(_) => uuid::Uuid::new_v4(),
            },
            Err(_) => uuid::Uuid::new_v4(),
        };
        context.insert_http_header(HEADER_NAME, id.hyphenated().to_string());
        id
    } else {
        let id = uuid::Uuid::new_v4();
        context.insert_http_header(HEADER_NAME, id.hyphenated().to_string());
        id
    }
}
