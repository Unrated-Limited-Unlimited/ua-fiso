use log::info;
use rocket::data::{Data, ToByteUnit};
use rocket::response::status;
use rocket::Responder;
use rocket::{get, post};

use crate::{
    db::{fetch::get_img_by_id, mutate::add_img},
    CLIENT,
};
use rocket::http::Status;

#[derive(Responder)]
#[response(status = 200, content_type = "image/jpeg")]
pub struct ImgResponse(Vec<u8>);

// TODO: Change to return img instead of img-type
/// Gets image by id
///
/// Returns a Result, meaning this function can error (like `Either` in Haskell)
/// The type in Result, is a json-object containing the Img-Struct.
#[get("/img/<id>")]
pub async fn get_img(id: &str) -> Result<ImgResponse, status::Custom<String>> {
    if let Some(client) = CLIENT.get().and_then(|mutex| mutex.try_lock().ok()) {
        if let Some(img) = get_img_by_id(&client, id.to_string())
            .await
            .ok()
            .and_then(|mut v| v.pop())
        {
            return Ok(ImgResponse(img));
        }
    }

    return Err(status::Custom(
        Status::NotFound,
        format!("File not found: {id}"),
    ));
}

/// Upload img
///
/// Returns use rocket::response::status;
#[post("/img/<id>", data = "<data>")]
pub async fn post_img(id: &str, data: Data<'_>) -> Result<(), status::Custom<String>> {
    info!("Uploading image with id: `{id}`");
    let mut img = Vec::new();
    if let Err(err) = data.open(200.mebibytes()).stream_to(&mut img).await {
        return Err(status::Custom(
            Status::InternalServerError,
            format!("Failed reading data: {err:?}"),
        ));
    }

    if let Some(client) = CLIENT.get().and_then(|mutex| mutex.try_lock().ok()) {
        match add_img(&client, img, id).await {
            Ok(_) => {
                return Ok(());
            }
            Err(err) => {
                return Err(status::Custom(
                    Status::InternalServerError,
                    format!("Failed: {err:?}"),
                ));
            }
        }
    }

    return Err(status::Custom(
        Status::InternalServerError,
        format!("Failed, no error"),
    ));
}
