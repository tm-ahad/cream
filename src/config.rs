use serde::Deserialize;
use toml::Table; 

#[derive(Debug, Deserialize)]
pub struct Config {
    pub build: Vec<String>,
    pub packages: Table,
    pub port: u16
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Config { build: self.build.clone(), port: self.port, packages: self.packages.clone() }
    }
}
