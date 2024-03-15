use serde::Deserialize;

use super::{database_config::DatabaseConfig, http_server_config::HttpServerConfig};

#[derive(Deserialize)]
pub struct BaseConfig {
    client: HttpServerConfig,
    admin: HttpServerConfig,
    database: DatabaseConfig
}