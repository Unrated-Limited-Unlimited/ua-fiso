use rocket::get;

#[get("/")]
pub fn root() -> String {
    "Hello, World!".to_string()
}
