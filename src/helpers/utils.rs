use anyhow::{Context, Result};
use std::fs;

pub async fn get_version() -> Result<String> {
    let version = fs::read_to_string("VERSION.txt")
        .context("Failed to read VERSION.txt")?
        .trim()
        .to_string();

    Ok(version)
}
