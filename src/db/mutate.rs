use bson::Document;
use eyre::{Context, Result};
use mongodb::{options::CreateCollectionOptions, Client, Collection};
use ua_rlib::models::img::Img;

use crate::utils::consts::{DB_COLLECTION_WIMG, DB_NAME};

pub async fn setup(client: &Client) -> Result<()> {
    let db = client.database(DB_NAME);

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

pub async fn add_img(client: &Client, img: Img) -> Result<()> {
    let db = client.database(DB_NAME);

    let collection = db.collection::<Document>(DB_COLLECTION_WIMG);

    collection
        .insert_one(
            bson::to_document(&img).wrap_err("Failed serializing img")?,
            None,
        )
        .await
        .wrap_err("Failed inserting img")
        .and(Ok(()))
}
