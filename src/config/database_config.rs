use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DatabaseConfig {
    pub address: String,
    pub pool_size: Option<usize>,
}
