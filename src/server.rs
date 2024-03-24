use actix_web::{
    web::{Data, ServiceConfig},
    App, HttpServer,
};

use crate::{
    config::http_server_config::HttpServerConfig,
    database::connection::connection_manager::ConnectionManager,
};

pub mod admin;
pub mod public;

pub async fn start_http_server(
    connection_manager: ConnectionManager,
    server_config: &HttpServerConfig,
    configure_actix: fn(&mut ServiceConfig),
) {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(connection_manager.clone()))
            .configure(configure_actix)
    })
    .bind((server_config.address, server_config.port))
    .unwrap()
    .run()
    .await
    .unwrap();
}
