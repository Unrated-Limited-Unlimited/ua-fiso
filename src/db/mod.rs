pub mod fetch;
pub mod mutate;

use anyhow::{Error, Result};
use bson::doc;
use mongodb::{options::ClientOptions, Client};

use crate::utils::consts::DB_URL;

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
