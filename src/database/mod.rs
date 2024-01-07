//! 
//! This module contains relevant utility functions for connecting to and reading/writing database information.
//! 
//! 
//! Most of the time, using glob imports from [`crate::preludes::database`] or [`self::prelude`] is good enough.
//! 
//! ## Most important items

//! - [`connect_with`], for getting a shared local postgres [`DbContext`].
// FIXME: Update for allowing user postgres passwords.

//! - [`prepared::read`] and [`prepared::modifying`], containing memoized functions for readonly and mutating SQL queries respectively.

#[allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
pub mod prepared;

// pub mod prepared;
// pub mod table_schemas;


use sqlx::PgPool;

/// Connect with is a convenience function that wraps the functionality of
/// - connecting to the server
/// - spawning the headless async connection service
/// - creating a [`DbContext`] for shared use within `GraphQl`
/// 
/// General usage would look somewhat like this:
/// ```
/// let postgres_connect_result = connect_with("localhost", "improved-eureka").await;
/// let db_ctx = match postgres_connect_result {
///     Ok(client) => client,
///     Err(e) => todo!("failed to connect to db: {}", e),
/// };
/// ```
/// 
/// # Errors
/// 
/// This function will return an error if it fails to connect to the database.
/// This could happen for a number of reasons, including
/// - Bad credentials (username/password) \[See [`crate::env::sql::username`]
///   and `.env`\]
/// - Bad db host/port \[See [`crate::env::sql::db_name`] and `.env`\]
/// - Postgres is not running \[try `psql --list`\]
/// - a multitude of other fun reasons
pub async fn connect_as(connection_name: &str) -> Result<PgPool, sqlx::Error> {
    use sqlx::postgres::{
        PgConnectOptions,
        PgPoolOptions
    };

    let connection_options = PgConnectOptions::new()
        .application_name(connection_name)
        // .host(crate::env::sql::db_url())
        .database(crate::env::sql::db_name())
        .username(crate::env::sql::username());

    let connection_options = if let Ok(password) = std::env::var("SQL_DB_PASS") {
        connection_options.password(&password)
    } else {
        connection_options
    };


    let options = PgPoolOptions::new()
        .min_connections(4)
        .max_connections(8);

    let client = options
        .connect_with(connection_options)
        .await?;

    Ok(client)
}

/// Unwrap connection is a convenience function that unwraps the result of [`connect_with`].
/// 
/// It will panic if the connection fails.
/// 
/// # Panics
/// 
/// This function will panic if the connection fails.
pub fn unwrap_connection(connection_result: Result<PgPool, sqlx::Error>) -> PgPool {
    match connection_result {
        Ok(client) => client,
        Err(e) => {
            crate::logging::error!("Failed to connect to eureka db: {e}");
            crate::logging::debug!("Eureka db error: {e:#?}");
            panic!("Failed to connect to eureka db: {e}");
        }
    }
}

pub type Ctx = sqlx::pool::PoolConnection<sqlx::Postgres>;


// /// At the moment, this is just a wrapper struct around a [PgPool].
// /// It may add more properties or methods in the future, especially ones relating to usage of SQL.
// pub struct DbContext {
//     /// This is the thread-safe async PostgreSQL Client connection to use for all of the local persistent storage on `improved-eureka`.
//     /// More fields may be added in the future.
//     pub client: sqlx::postgres::PgPool,
// }
