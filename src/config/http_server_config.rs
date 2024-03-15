use std::net::IpAddr;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct HttpServerConfig {
    pub address: IpAddr,
    pub port: u16
}