use anyhow::Result;
use db::create_client;
use mongodb::Client;
use once_cell::unsync::Lazy;
use tokio::{runtime::Runtime, sync::Mutex};

mod api;
mod db;
#[cfg(test)]
mod tests;
mod utils;

pub const CLIENT: Lazy<Mutex<Client>> = Lazy::new(|| {
    let rt = Runtime::new().expect("Failed creating const CLIENT");

    let res = rt
        .block_on(async { create_client().await })
        .expect("Failed creating client");

    Mutex::new(res)
});

#[tokio::main]
async fn main() -> Result<()> {
    let client = db::create_client().await?;
    db::mutate::setup(&client).await?;

    Ok(())
}
