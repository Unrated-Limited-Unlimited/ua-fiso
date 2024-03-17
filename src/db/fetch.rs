use anyhow::Result;
use bson::doc;
use mongodb::Client;
use ua_rlib::models::img::Img;

use crate::utils::consts::{DB_COLLECTION_WIMG, DB_NAME};

pub async fn get_img_by_id(client: &Client, id: String) -> Result<Img> {
    let filter = doc! { "id": &id };
    let db = client.database(DB_NAME);
    let collection = db.collection::<Img>(DB_COLLECTION_WIMG);

    Ok(collection
        .find_one(filter, None)
        .await?
        .ok_or(anyhow::Error::msg(format!(
            "Failed finding img by id `{}´",
            id
        )))?)
}
