use actix_web::{
    post,
    web::{route, scope, Json, Path},
    App, HttpResponse, HttpServer, Responder, Route, Scope,
};
use serde::Deserialize;
use serde_json::Value as JsonValue;

use crate::config::get_config;

#[derive(Debug, Deserialize)]
struct CreateCompositionBody {
    content: JsonValue,
}

#[post("/poietic/create-composition")]
async fn create_composition(body: Json<CreateCompositionBody>) -> impl Responder {
    todo!("Create the composition");
    HttpResponse::Ok()
}

#[derive(Debug, Deserialize)]
struct CreatePageBody {
    path: String,
    composition_id: String,
}

#[post("/poietic/create-page")]
async fn create_page(body: Json<CreatePageBody>) -> impl Responder {
    todo!("Create the page");
    HttpResponse::Ok()
}

#[actix_web::post("/{namespace}/{api_function}")]
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
