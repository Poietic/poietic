use std::fs::read_to_string;

use serde::Deserialize;

use super::{database_config::DatabaseConfig, http_server_config::HttpServerConfig};

#[derive(Clone, Deserialize)]
pub struct BaseConfig {
    pub public: HttpServerConfig,
    pub admin: HttpServerConfig,
    pub database: DatabaseConfig
}

#[derive(Debug)]
pub enum ConfigLoadError {
    IncorrectConfig,
    CannotAccessFile
}

impl BaseConfig {
    pub fn load() -> Result<Self, ConfigLoadError> {
        let contents = read_to_string("config.yaml")
            .map_err(|_| ConfigLoadError::CannotAccessFile)?;
        
        serde_yaml::from_str(contents.as_str())
            .map_err(|_| ConfigLoadError::IncorrectConfig)
    }
}