use anyhow::Result;
use api::{admin_page::root, img::get_img};
use db::create_client;
use mongodb::Client;
use rocket::routes;
use tokio::{
    runtime::Runtime,
    sync::{Mutex, OnceCell},
};

mod api;
mod db;
#[cfg(test)]
mod tests;
mod utils;

/// Database connection
pub static CLIENT: OnceCell<Mutex<Client>> = OnceCell::const_new();

/// Entry point for the microservice
///
/// Sets up the database, and then starts the server
#[rocket::main]
async fn main() -> Result<()> {
    // Database setup
    CLIENT
        .get_or_init(|| async {
            Mutex::new(db::create_client().await.expect("Failed creating client"))
        })
        .await;

    // Server setup
    let rocket = rocket::build()
        .mount("/", routes![root])
        .mount("/api", routes![get_img])
        .launch();

    rocket.await?;

    Ok(())
}
