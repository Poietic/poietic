use std::sync::OnceLock;

use self::base_config::BaseConfig;

pub mod base_config;
pub mod database_config;
pub mod http_server_config;

static CONFIG: OnceLock<BaseConfig> = OnceLock::new();

pub fn get_config() -> &'static BaseConfig {
    CONFIG.get_or_init(|| BaseConfig::load().unwrap())
}
