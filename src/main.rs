use eyre::{Context, Result};

mod db;
mod endpoints;
#[cfg(test)]
mod tests;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let client = db::create_client().await?;
    db::mutate::setup(&client).await?;

    Ok(())
}
