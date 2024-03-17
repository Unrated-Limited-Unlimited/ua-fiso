#[cfg(not(debug_assertions))]
pub const DB_URL: &str = "mongodb://ua-fiso-db:27017";

#[cfg(debug_assertions)]
pub const DB_URL: &str = "mongodb://localhost:27018";

pub const DB_NAME: &str = "ua_db";

pub const DB_COLLECTION_WIMG: &str = "wimg";

pub const VERSION: &str = "v1.1.0";
