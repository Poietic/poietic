use actix_web::{App, HttpServer};
use config::http_server_config::HttpServerConfig;

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
async fn main() -> () {
    let config = BaseConfig::load().unwrap();
    tokio::spawn(start_http_server(config.client));
    tokio::spawn(start_http_server(config.admin));
}
