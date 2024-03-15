use actix_web::{App, HttpServer};
use config::http_server_config::HttpServerConfig;
use tokio::task::{JoinError, JoinSet};

use crate::config::base_config::BaseConfig;

mod config;

async fn start_http_server(config: HttpServerConfig) {
    HttpServer::new(|| App::new())
        .bind((config.address, config.port))
        .unwrap()
        .run()
        .await.unwrap();
}

#[actix_web::main]
async fn main() -> Result<(), JoinError> {
    let config = BaseConfig::load().unwrap();
    let mut http_server_set = JoinSet::new();
    http_server_set.spawn(start_http_server(config.client));
    http_server_set.spawn(start_http_server(config.admin));
    while let Some(res) = http_server_set.join_next().await {
        let _ = res?;
    }
    Ok(())
}
