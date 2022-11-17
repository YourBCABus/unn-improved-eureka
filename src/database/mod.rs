pub mod prepared;

pub mod prelude;

use tokio_postgres::{Client, error::Error as SqlCommunicationError, connect as db_connect, NoTls};
use tokio::spawn as async_spawn;

pub async fn connect_with(host: &str, user: &str) -> Result<Client, SqlCommunicationError> {
    let (client, connection) = db_connect(
        &format!("host={host} user={user}"),
        NoTls,
    ).await?;

    async_spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub struct DbContext {
    pub client: Client,
}
