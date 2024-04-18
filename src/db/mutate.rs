use crate::{
    db::fetch,
    utils::consts::{DB_COLLECTION_WIMG, DB_NAME},
};
use anyhow::Result;
use bson::Document;
use log::info;
use mongodb::Client;
use ua_rlib::models::img::ImgWrapper;

/// Adds an image to the database.
///
/// Works by converting the struct into a json object (same as bson),
/// and storing it in the MongoDB
pub async fn add_img(client: &Client, img: Vec<u8>, id: &str) -> Result<bool> {
    info!("Adding img with id: `{id}`");
    let db = client.database(DB_NAME);

    let res = fetch::get_img_by_id(client, id.to_string())
        .await
        .is_ok_and(|e| !e.is_empty());

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

    Ok(res)
}
