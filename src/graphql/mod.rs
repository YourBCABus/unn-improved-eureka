//! This module provides access to most of the structs/wrapper structs necessary for running and setting up GraphQL
//! For a blob import, it is encouraged to use [`crate::graphql::prelude`] or
//! [`crate::preludes::graphql`] instead of `crate::graphql` for code cleanliness and consistency.
//! 
//! 
//! 

pub mod structs;

pub mod resolvers;


use crate::verification::google::user_allowed;
use crate::env::graphql_complexity_limit_usize_panic;
use crate::state::AppState;

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
        .limit_complexity(graphql_complexity_limit_usize_panic())
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

pub struct IdSecretScopes(crate::verification::scopes::Scopes);
pub struct IdTokenScopes(crate::verification::scopes::Scopes);

async fn get_scopes_id_secret(context: &async_graphql::Context<'_>) -> async_graphql::Result<crate::verification::scopes::Scopes> {
    use crate::verification::{
        ClientIdHeader, ClientSecretHeader,
        scopes::Scopes, id_secret::client_allowed,
    };
    use tokio::sync::OnceCell;
    use async_graphql::Error as GraphQlError;


    let Ok(scopes_cell) = context.data::<OnceCell<IdSecretScopes>>() else {
        crate::logging::error!("OnceCell Missing from context!");
        return Ok(Scopes::new());
    };

    scopes_cell.get_or_try_init(|| async {
        let Ok(app_state) = context.data::<crate::state::AppState>() else {
            let err = GraphQlError::new("Internal server error (App State)");
            crate::logging::error!("{err:?}");
            return Err(err);
        };
        let mut db_pool = match app_state.db().acquire().await {
            Ok(db_pool) => db_pool,
            Err(e) => {
                crate::logging::error!("DB Error: {e:?}");
                return Err(GraphQlError::new("Internal server error (DB)"));
            },
        };

        let id = context.data::<ClientIdHeader>().map(|id| id.inner());
        let secret = context.data::<ClientSecretHeader>().map(|secret| secret.as_bytes());

        let id_ok = id.is_ok();
        let secret_ok = secret.is_ok();

        let school_id = get_school_id(context).await?;
        if let (Ok(id), Ok(secret)) = (id, secret) {
            match client_allowed(
                school_id,
                id,
                secret, 
                &mut db_pool,
            ).await {
                Some(scopes) => Ok(IdSecretScopes(scopes)),
                None => {
                    crate::logging::error!("Client not allowed");
                    Ok(IdSecretScopes(Scopes::new()))
                },
            }
        } else {
            crate::logging::info!("No client id or secret, id: {id_ok}, secret: {secret_ok}");
            Ok(IdSecretScopes(Scopes::new()))
        }
    }).await.map(|scopes| scopes.0)
}

async fn get_scopes_id_token(context: &async_graphql::Context<'_>) -> async_graphql::Result<crate::verification::scopes::Scopes> {
    use crate::verification::{
        scopes::Scopes,
        IdTokenHeader,
    };
    use tokio::sync::OnceCell;
    use async_graphql::Error as GraphQlError;


    let Ok(scopes_cell) = context.data::<OnceCell<IdTokenScopes>>() else {
        crate::logging::error!("OnceCell Missing from context!");
        return Ok(Scopes::new());
    };

    scopes_cell.get_or_try_init(|| async {
        let Ok(app_state) = context.data::<crate::state::AppState>() else {
            let err = GraphQlError::new("Internal server error (App State)");
            crate::logging::error!("{err:?}");
            return Err(err);
        };
        let mut db_pool = match app_state.db().acquire().await {
            Ok(db_pool) => db_pool,
            Err(e) => {
                crate::logging::error!("DB Error: {e:?}");
                return Err(GraphQlError::new("Internal server error (DB)"));
            },
        };

        let id_token = context.data::<IdTokenHeader>();

        // TODO: Gate to schools
        let school_id = get_school_id(context).await?;
        if let Ok(id_token) = id_token {
            match user_allowed(
                &mut db_pool,
                id_token.clone(),
            ).await  {
                Some(scopes) => Ok(IdTokenScopes(scopes)),
                None => {
                    crate::logging::error!("User not allowed");
                    Ok(IdTokenScopes(Scopes::new()))
                },
            }
        } else {
            crate::logging::info!("No user ID token");
            Ok(IdTokenScopes(Scopes::new()))
        }
    }).await.map(|scopes| scopes.0)
}


mod school_id {
    use uuid::Uuid;

    pub struct SchoolId(std::sync::RwLock<Uuid>);

    impl std::fmt::Debug for SchoolId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SchoolId({})", self.inner())
        }
    }
    
    impl SchoolId {
        pub fn new(id: Uuid) -> Self {
            Self(std::sync::RwLock::new(id))
        }
    
        fn inner(&self) -> Uuid {
            self.0.try_read().map(|g| *g).unwrap_or_default()
        }
    
        fn set(&self, id: Uuid) {
            if let Ok(mut guard) = self.0.try_write() {
                *guard = id;
            }
        }
    }

    pub async fn get_school_id(context: &async_graphql::Context<'_>) -> async_graphql::Result<Uuid> {
        use async_graphql::Error as GraphQlError;
    
        let uuid = context.data::<SchoolId>().map(|id| id.inner())?;
        if uuid.is_nil() {
            let Ok(app_state) = context.data::<crate::state::AppState>() else {
                let err = GraphQlError::new("Internal server error (App State)");
                crate::logging::error!("{err:?}");
                return Err(err);
            };
            let mut db_pool = match app_state.db().acquire().await {
                Ok(db_pool) => db_pool,
                Err(e) => {
                    crate::logging::error!("DB Error: {e:?}");
                    return Err(GraphQlError::new("Internal server error (DB)"));
                },
            };
    
            let Ok(default_school_id) = crate::database::prepared::config::get_default_school_id(&mut db_pool).await else {
                return Err(async_graphql::Error::new("Failed to get school id"));
            };

            if let Ok(school_id) = context.data::<SchoolId>() {
                school_id.set(default_school_id);
            }

            Ok(default_school_id)
        } else {
            Ok(uuid)
        }
    }

    pub fn with_school_id(req: async_graphql::Request, school_id: Uuid) -> async_graphql::Request {
        req.data(SchoolId::new(school_id))
    }
}

pub use school_id::{ get_school_id, with_school_id };
