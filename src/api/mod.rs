use rocket::get;

use crate::utils::consts::VERSION;

pub mod img;

pub mod admin_page;

#[get("/ver")]
pub async fn get_version() -> String {
    VERSION.to_string()
}
