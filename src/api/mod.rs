use rocket::get;

use crate::utils::consts::VERSION;

pub mod img;

#[get("/meta/version")]
pub async fn get_version() -> String {
    VERSION.to_string()
}
