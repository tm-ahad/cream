use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use toml::Table; 

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Config {
    pub build: Vec<String>,
    pub packages: Table,
    pub env: HashMap<String, toml::Value>,
    pub port: u16
}

