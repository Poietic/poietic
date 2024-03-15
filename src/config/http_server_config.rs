use serde::Deserialize;

#[derive(Deserialize)]
pub struct HttpServerConfig {
    port: u16
}