use std::net::IpAddr;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DatabaseConfig {
    pub address: IpAddr,
    pub port: u16,
    pub user: String,
    pub password: String
}