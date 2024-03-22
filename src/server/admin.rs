use actix_web::{App, HttpServer};

use crate::config::get_config;

pub async fn start_admin_http_server() {
    let config = &get_config().admin;
    HttpServer::new(|| App::new())
        .bind((config.address, config.port))
        .unwrap()
        .run()
        .await
        .unwrap();
}
