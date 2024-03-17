use rocket::{get, serde::json::Json};
use rocket_errors::anyhow::Result;
use ua_rlib::models::img::Img;

use crate::{db::fetch::get_img_by_id, CLIENT};

#[get("/img/<id>")]
pub async fn get_img(id: &str) -> Result<Json<Img>> {
    Ok(Json(
        get_img_by_id(&(*CLIENT.lock().await), id.to_string()).await?,
    ))
}
