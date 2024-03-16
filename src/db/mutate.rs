use eyre::{Context, Result};
use mongodb::{options::CreateCollectionOptions, Client};

use crate::utils::consts::{DB_COLLECTION_WIMG, DB_W_IMG_NAME};

pub async fn setup(client: &Client) -> Result<()> {
    let db = client.database(DB_W_IMG_NAME);

    let collections = db
        .list_collection_names(None)
        .await
        .wrap_err("Failed getting collection names")?;

    if !collections.contains(&(DB_COLLECTION_WIMG.to_string())) {
        db.create_collection(DB_COLLECTION_WIMG, CreateCollectionOptions::default())
            .await
            .wrap_err(format!(
                "Failed creating collection `{}`",
                DB_COLLECTION_WIMG
            ))?;
    }

    Ok(())
}

pub async fn add_img(client: &Client, img: &[u8]) -> Result<()> {
    let db = client.database(DB_W_IMG_NAME);

    Ok(())
}
