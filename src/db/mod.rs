pub mod fetch;
pub mod mutate;

use anyhow::Result;
use mongodb::{options::ClientOptions, Client};

use crate::utils::consts::DB_URL;

pub async fn create_client() -> Result<Client> {
    Ok(Client::with_options(ClientOptions::parse(DB_URL).await?)?)
}
