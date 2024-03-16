use bson::{doc, Document};
use eyre::{Context, OptionExt, Result};
use mongodb::Client;
use ua_rlib::models::img::Img;

use crate::utils::consts::{DB_COLLECTION_WIMG, DB_NAME};

pub async fn get_img(client: &Client, id: String) -> Result<Img> {
    let filter = doc! { "id": &id };
    let db = client.database(DB_NAME);
    let collection = db.collection::<Document>(DB_COLLECTION_WIMG);

    collection
        .find_one(filter, None)
        .await
        .wrap_err("Failed filtering collection")?
        .ok_or_eyre(format!("No image found with id: `{}`", id))
}
