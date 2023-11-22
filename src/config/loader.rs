use std::fs;
use anyhow::{Result, Context};
use crate::config::structs;

pub fn with_config(file_path: &str) -> Result<structs::AppConfig> {
    let config_str = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read config file: {}", file_path))?;
    let config = toml::from_str(&config_str)
        .with_context(|| "Failed to parse config file as TOML")?;
    Ok(config)
}