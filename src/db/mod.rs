pub mod fetch;
pub mod mutate;

use anyhow::{Context, Error, Result};
use bson::{doc, DeserializerOptions};
use mongodb::{options::ClientOptions, Client};

use crate::utils::consts::DB_URL;

/// Creates a connection to the database
///
/// Pings the database, ensuring a client is only made if the server is up and running
pub async fn create_client() -> Result<Client> {
    let client = Client::with_options(ClientOptions::parse(DB_URL).await?)?;

    match client
        .database("admin")
        .run_command(doc! { "ping": 1 }, None)
        .await
    {
        Ok(_) => Ok(client),
        Err(err) => Err(Error::msg(format!(
            "Failed pinging server, got error: `{:?}`",
            err
        ))),
    }
}

/// Creates a connection to the database
/// Does not verify if the connection is valid
pub async fn create_client_debug() -> Result<Client> {
    Ok(Client::with_options(ClientOptions::parse(DB_URL).await?)?)
}

/*
Incase we want more fancy options, will probably not happen.
 */
pub fn get_bson_deserializer_option() -> DeserializerOptions {
    DeserializerOptions::builder().build()
}
