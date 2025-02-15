use async_graphql::Context;
use db::get_db;
use uuid::Uuid;

use crate::scopes::Scopes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientInfo { pub id: Uuid, pub secret: String }

pub async fn get_scopes(
    context: &Context<'_>,
) -> async_graphql::Result<Scopes> {
    use crate::{ client::client_allowed, user::user_allowed };
    use tokio::sync::OnceCell;

    let path_node = context.path_node.map(|node| node.to_string()).unwrap_or_default();

    let Ok(scopes_cell) = context.data::<OnceCell<Scopes>>() else {
        logging::error!("OnceCell<Scopes> missing from context @ path node {path_node}!");
        logging::report!("OnceCell<Scopes> missing from context": { "path_node": path_node });
        return Ok(Scopes::new());
    };

    scopes_cell.get_or_try_init(|| async {
        logging::trace!("Getting scopes for the first time this request");
        let mut db_conn = get_db!(context);
        let school_id = graphql::get_school_id(context).await?;

        // Check for and get client scopes
        if let Some(client) = context.data_opt::<ClientInfo>() {
            logging::trace!("Received client credentials with id {}", logging::fmt_client_id(client.id));

            if let Some(new_scopes) = client_allowed(
                &mut db_conn,
                school_id,
                client.id,
                client.secret.as_bytes(),
            ).await {
                logging::trace!("Valid client, caching scopes");
                return Ok(new_scopes);
            } else {
                logging::warn!("Invalid client, authentication error");
                return Err(async_graphql::Error::new("Invalid client"));
            }
        } else {
            logging::trace!("No client credentials");
        }

        // Check for and get user scopes
        if let Some(token_header) = context.data_opt::<crate::IdTokenHeader>() {
            logging::trace!("Received user id token");

            match user_allowed(
                &mut db_conn,
                school_id,
                token_header.clone(),
            ).await {
                Ok(new_scopes) => {
                    logging::trace!("Valid user, caching scopes");
                    return Ok(new_scopes);
                }
                Err(e) => {
                    logging::warn!("Invalid user, authentication error: {e}");
                    logging::report!("Invalid user, authentication error": { "e": e });
                    return Err(async_graphql::Error::new("Invalid user"));
                }
            }
        } else {
            logging::trace!("No user credentials");
        }

        Ok(Scopes::new())
    }).await.copied()
}

#[macro_export]
macro_rules! ensure_auth {
    ($ctx:ident, [$($scopes:ident),+]) => {
        {
            let scopes = ::auth::context::get_scopes($ctx).await?;
            $(
                if !scopes.$scopes && !scopes.admin {
                    return Err(async_graphql::Error::new("Unauthorized"));
                }
            )+
        }
    };
}
