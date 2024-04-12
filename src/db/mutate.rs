use anyhow::Result;
use bson::Document;
use log::info;
use mongodb::{options::CreateCollectionOptions, Client, Collection};
use rocket::serde::{Deserialize, Serialize};

use crate::utils::consts::{DB_COLLECTION_WIMG, DB_NAME};

/// Setups the database, ensuring that the wanted collections (tables) exist, if not, creates them.
pub async fn setup(client: &Client) -> Result<()> {
    let db = client.database(DB_NAME);

    let collections = db.list_collection_names(None).await?;

    if !collections.contains(&(DB_COLLECTION_WIMG.to_string())) {
        db.create_collection(DB_COLLECTION_WIMG, CreateCollectionOptions::default())
            .await?;
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImgWrapper {
    pub img: Vec<u8>,
    pub iid: String,
}

/// Adds an image to the database.
///
/// Works by converting the struct into a json object (same as bson),
/// and storing it in the MongoDB
pub async fn add_img(client: &Client, img: Vec<u8>, id: &str) -> Result<()> {
    info!("Adding img with id: `{id}`");
    let db = client.database(DB_NAME);

    let collection = db.collection::<Document>(DB_COLLECTION_WIMG);

    collection
        .insert_one(
            bson::to_document(&ImgWrapper {
                img,
                iid: id.to_string(),
            })?,
            None,
        )
        .await?;

    Ok(())
}
