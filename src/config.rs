use std::{fs::read_to_string, path::Path};

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Counter {
    pub label: String,
    pub query: String,
    pub last_updated: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub github_token: String,
    pub counters: Vec<Counter>,
}

pub fn load_config(path: &Path) -> Result<Config> {
    Ok(toml::from_str(read_to_string(path)?.as_str())?)
}
