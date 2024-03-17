use rocket::{get, post, serde::json::Json};
use rocket_errors::anyhow::{AnyhowError, Result};
use ua_rlib::models::img::Img;

use crate::{
    db::{fetch::get_img_by_id, mutate::add_img},
    CLIENT,
};

#[get("/img/<id>")]
pub async fn get_img(id: &str) -> Result<Json<Vec<Img>>> {
    let mutex = CLIENT
        .get()
        .ok_or(AnyhowError(anyhow::Error::msg("Failed getting id")))?;

    let client = mutex.try_lock()?;
    let img = get_img_by_id(&client, id.to_string()).await?;

    Ok(Json(img))
}

#[post("/img", data = "<img>")]
pub async fn post_img(img: Json<Img>) -> Result<()> {
    let mutex = CLIENT
        .get()
        .ok_or(AnyhowError(anyhow::Error::msg("Failed getting id")))?;

    let client = mutex.try_lock()?;

    add_img(&client, img.0).await?;

    Ok(())
}
