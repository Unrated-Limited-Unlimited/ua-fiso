use crate::db::mutate::ImgWrapper;
use anyhow::Result;
use bson::{doc, Document};
use log::info;
use mongodb::Client;
use rocket::futures::StreamExt;

use crate::utils::consts::{DB_COLLECTION_WIMG, DB_NAME};

use super::get_bson_deserializer_option;

/// Returns a list of all images that matches the given id.
///
/// Return type is a list, incase of images with same id
pub async fn get_img_by_id(client: &Client, id: String) -> Result<Vec<Vec<u8>>> {
    info!("Searching for id with id: `{id}`");
    let filter = doc! { "iid": &id };
    let db = client.database(DB_NAME);
    let collection = db.collection::<Document>(DB_COLLECTION_WIMG);

    Ok(collection
        .find(filter, None)
        .await?
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .filter_map(|r| match r {
            Ok(d) => {
                bson::from_document_with_options::<ImgWrapper>(d, get_bson_deserializer_option())
                    .ok()
            }
            Err(_) => None,
        })
        .map(|e| e.img)
        .collect::<Vec<_>>())
}
