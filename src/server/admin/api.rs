use actix_web::{
    post,
    web::{route, scope, Path},
    HttpResponse, Responder, Route, Scope,
};

pub mod create_composition;
pub mod create_page;

#[post("/{namespace}/{api_function}")]
pub async fn api_route_service(path: Path<(String, String)>) -> impl Responder {
    let (namespace, api_function) = path.into_inner();
    todo!("Handle API routes");
    ""
}

fn create_404_handler() -> Route {
    route().to(HttpResponse::NotFound)
}

pub fn create_api_scope() -> Scope {
    scope("/api")
        .service(create_composition::create_composition)
        .service(create_page::create_page)
        .service(api_route_service)
        .default_service(create_404_handler())
}
