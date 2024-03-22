use actix_web::{
    web::{route, scope, Path},
    App, HttpResponse, HttpServer, Responder, Route, Scope,
};

use crate::config::get_config;

#[actix_web::post("{namespace}/{api_function}")]
async fn api_route_service(path: Path<(String, String)>) -> impl Responder {
    let (namespace, api_function) = path.into_inner();
    todo!("Handle API routes");
    ""
}

fn create_404_handler() -> Route {
    route().to(HttpResponse::NotFound)
}

fn create_api_scope() -> Scope {
    scope("/api")
        .service(api_route_service)
        .default_service(create_404_handler())
}

#[actix_web::get("{page_path:.*}")]
async fn page_route_service(path: Path<String>) -> impl Responder {
    let page_path = path.into_inner();
    todo!("Handle page routes");
    ""
}

pub async fn start_public_http_server() {
    let config = &get_config().public;
    HttpServer::new(|| {
        App::new()
            .service(create_api_scope())
            .service(page_route_service)
            .default_service(create_404_handler())
    })
    .bind((config.address, config.port))
    .unwrap()
    .run()
    .await
    .unwrap();
}
