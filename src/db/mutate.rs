use anyhow::Result;
use bson::Document;
use mongodb::{options::CreateCollectionOptions, Client, Collection};
use ua_rlib::models::img::Img;

use crate::utils::consts::{DB_COLLECTION_WIMG, DB_NAME};

/// Setups the databse, ensuring that the wanted collections (tabels) exist, if not, creates them.
pub async fn setup(client: &Client) -> Result<()> {
    let db = client.database(DB_NAME);

    let collections = db.list_collection_names(None).await?;

    if !collections.contains(&(DB_COLLECTION_WIMG.to_string())) {
        db.create_collection(DB_COLLECTION_WIMG, CreateCollectionOptions::default())
            .await?;
    }

    Ok(())
}

/// Adds an image to the database.
///
/// Works by converting the struct into a json object (same as bson),
/// and storing it in the MongoDB
pub async fn add_img(client: &Client, img: Img) -> Result<()> {
    let db = client.database(DB_NAME);

    let collection = db.collection::<Document>(DB_COLLECTION_WIMG);

    collection
        .insert_one(bson::to_document(&img)?, None)
        .await?;

    Ok(())
}
