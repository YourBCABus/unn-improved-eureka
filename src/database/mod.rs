//! 
//! This module contains relevant utility functions for connecting to and reading/writing database information.
//! 
//! 
//! Most of the time, using glob imports from [crate::preludes::database] or [self::prelude] is good enough.
//! 
//! ## Most important items

//! - [connect_with], for getting a shared local postgres [DbContext].
// FIXME: Update for allowing user postgres passwords.

//! - [prepared::read] and [prepared::modifying], containing memoized functions for readonly and mutating SQL queries respectively.

pub mod prelude;

pub mod prepared;
pub mod table_schemas;


use tokio_postgres::{
    Client as PostgresClient,
    error::Error as SqlCommunicationError,
    connect as db_connect,
    NoTls as PostgresNoTls,
};
use tokio::{spawn as async_spawn, sync::Mutex};


use std::sync::Arc;

/// Connect with is a convenience function that wraps the functionality of
/// - connecting to the server
/// - spawning the headless async connection service
/// - creating a [DbContext] for shared use within GraphQl
/// 
/// General usage would look somewhat like this:
/// ```
/// let postgres_connect_result = connect_with("localhost", "improved-eureka").await;
/// let db_ctx = match postgres_connect_result {
///     Ok(client) => client,
///     Err(e) => todo!("failed to connect to db: {}", e),
/// };
/// ```
pub async fn connect_with(host: &str, user: &str) -> Result<Arc<Mutex<DbContext>>, SqlCommunicationError> {
    let (client, connection) = db_connect(
        &format!("host={host} user={user}"),
        PostgresNoTls,
    ).await?;

    async_spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(Arc::new(Mutex::new(DbContext{ client })))
}

/// At the moment, this is just a wrapper struct around a [PostgresClient].
/// It may add more properties or methods in the future, especially ones relating to usage of SQL.
pub struct DbContext {
    /// This is the thread-safe async PostgreSQL Client connection to use for all of the local persistent storage on `improved-eureka`.
    /// More fields may be added in the future.
    pub client: PostgresClient,
}
