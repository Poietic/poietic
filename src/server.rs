use actix_web::{web::ServiceConfig, App, HttpServer};

use crate::config::http_server_config::HttpServerConfig;

pub mod admin;
pub mod public;

pub async fn start_http_server(
    server_config: &HttpServerConfig,
    configure_actix: fn(&mut ServiceConfig),
) {
    HttpServer::new(move || App::new().configure(configure_actix))
        .bind((server_config.address, server_config.port))
        .unwrap()
        .run()
        .await
        .unwrap();
}
