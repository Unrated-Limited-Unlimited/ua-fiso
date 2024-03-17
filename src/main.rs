use anyhow::Result;
use api::{
    admin_page::root,
    get_version,
    img::{get_img, post_img},
};
use db::mutate::setup;
use mongodb::Client;
use rocket::routes;
use tokio::sync::{Mutex, OnceCell};

mod api;
mod db;
#[cfg(test)]
mod tests;
mod utils;

/// Database connection
///
/// The same connection is used through the entire application
pub static CLIENT: OnceCell<Mutex<Client>> = OnceCell::const_new();

/// Entry point for the microservice
///
/// Sets up the database, and then starts the server
#[rocket::main]
async fn main() -> Result<()> {
    // Database setup
    let mutex = CLIENT
        .get_or_init(|| async {
            Mutex::new(db::create_client().await.expect("Failed creating client"))
        })
        .await;

    let client = mutex.try_lock()?;

    setup(&client).await?;

    // Server setup
    let rocket = rocket::build()
        .mount("/", routes![root])
        .mount("/api", routes![get_version, get_img, post_img])
        .launch();

    rocket.await?;

    Ok(())
}
