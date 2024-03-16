pub mod mutate;

use eyre::{Context, Result};
use mongodb::{options::ClientOptions, Client};

use crate::utils::consts::DB_URL;

pub async fn create_client() -> Result<Client> {
    Client::with_options(
        ClientOptions::parse(DB_URL)
            .await
            .wrap_err(format!("Failed parsing options from url: `{}`", DB_URL))?,
    )
    .wrap_err(format!("Failed creating client with options"))
}
