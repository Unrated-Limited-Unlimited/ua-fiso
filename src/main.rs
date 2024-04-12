use anyhow::Result;
use api::{
    get_version,
    img::{get_img, post_img},
};
use colored::Colorize;
use db::mutate::setup;
use dotenv::dotenv;
use log::{error, info, warn, Record};
use mongodb::Client;
use rocket::{routes, Config};
use std::io::Write;
use std::net::Ipv4Addr;
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
/// Sets up the database, and then starts the server, so it's stored behind a mutex,
/// to ensure no race condition happens.
#[rocket::main]
async fn main() -> Result<()> {
    // Loading dotenv
    dotenv()?;

    // Logger setup
    env_logger::builder()
        .format(|buf: &mut env_logger::fmt::Formatter, record: &Record| {
            let lvl = match record.level() {
                log::Level::Error => record.level().as_str().red(),
                log::Level::Warn => record.level().as_str().yellow(),
                log::Level::Info => record.level().as_str().blue(),
                _ => record.level().as_str().into(),
            };

            writeln!(buf, "[{}]: {}", lvl, record.args())
        })
        .init();

    info!("Initialized logger");
    // Database setup
    let mutex = CLIENT
        .get_or_init(|| async {
            match db::create_client().await {
                Ok(c) => {
                    info!("Created client");
                    Mutex::new(c)
                }
                Err(e) => {
                    error!("Failed creating client, got error: {:?}", e);
                    panic!("Failed creating client");
                }
            }
        })
        .await;

    let client = mutex.try_lock()?;

    match setup(&client).await {
        Ok(_) => info!("Finished DB setup"),
        Err(_) => warn!("Failed DB setup"),
    }

    drop(client);

    // Server setup
    info!("Initializing rocket server");
    let rocket = rocket::build()
        .mount("/api", routes![get_version, get_img, post_img])
        .configure(Config {
            port: 8001,
            address: Ipv4Addr::new(0, 0, 0, 0).into(),
            ..Default::default()
        })
        .launch();

    info!("Launching rocket server");
    rocket.await?;

    Ok(())
}
