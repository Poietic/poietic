use actix_web::{
    web::{route, scope},
    App, HttpResponse, HttpServer, Route, Scope,
};
mod api;

use crate::config::get_config;

use self::api::{api_route_service, create_composition, create_page};

fn create_404_handler() -> Route {
    route().to(HttpResponse::NotFound)
}

fn create_api_scope() -> Scope {
    scope("/api")
        .service(create_composition)
        .service(create_page)
        .service(api_route_service)
        .default_service(create_404_handler())
}

pub async fn start_admin_http_server() {
    let config = &get_config().admin;
    HttpServer::new(|| {
        App::new()
            .service(create_api_scope())
            .default_service(create_404_handler())
    })
    .bind((config.address, config.port))
    .unwrap()
    .run()
    .await
    .unwrap();
}
