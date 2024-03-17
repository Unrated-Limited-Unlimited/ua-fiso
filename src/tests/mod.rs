use anyhow::{Context, Result};
use serde_json::Value;
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};
use toml;

use crate::utils::consts::VERSION;

/// Ensures the versions specified match
#[tokio::test]
pub async fn test_version() -> Result<()> {
    let file = File::open("Cargo.toml").await?;

    let mut reader = BufReader::new(file);

    let mut buffer = String::new();

    reader.read_to_string(&mut buffer).await?;

    match toml::from_str::<Value>(&buffer) {
        Ok(t) => assert_eq!(
            format!(
                "v{}",
                t["package"]["version"]
                    .as_str()
                    .expect("Failed getting version from Cargo.toml")
            ),
            VERSION
        ),
        Err(err) => panic!("{:?}", err),
    }

    Ok(())
}
