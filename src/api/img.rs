use rocket::{get, post, serde::json::Json};
use rocket_errors::anyhow::{AnyhowError, Result};
use ua_rlib::models::img::Img;

use crate::{
    db::{fetch::get_img_by_id, mutate::add_img},
    CLIENT,
};

// TODO: Change to return img instead of img-type
/// Gets image by id
///
/// Returns a Result, meaning this function can error (like `Either` in Haskell)
/// The type in Result, is a json-object containing the Img-Struct.
#[get("/img/<id>")]
pub async fn get_img(id: &str) -> Result<Json<Vec<Img>>> {
    let mutex = CLIENT
        .get()
        .ok_or(AnyhowError(anyhow::Error::msg("Failed getting mutex lock")))?;

    let client = mutex.try_lock()?;
    let img = get_img_by_id(&client, id.to_string()).await?;

    Ok(Json(img))
}

/// Upload img
///
/// Returns a result, giving the caller a sign if the function was successfull or not.
#[post("/img", data = "<img>")]
pub async fn post_img(img: Json<Img>) -> Result<()> {
    let mutex = CLIENT
        .get()
        .ok_or(AnyhowError(anyhow::Error::msg("Failed getting mutex lock")))?;

    let client = mutex.try_lock()?;

    add_img(&client, img.0).await?;

    Ok(())
}
