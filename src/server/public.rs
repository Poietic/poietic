use actix_web::{
    web::{route, scope, Path},
    App, HttpResponse, HttpServer, Responder, Route, Scope,
};

use crate::{
    component::render_composition,
    config::get_config,
    database::data_access::{composition::get_composition_from_page, page::get_page_at_path},
    error::PoieticError,
};

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
async fn page_route_service(path: Path<String>) -> Result<impl Responder, PoieticError> {
    let page_path = path.into_inner();
    let page = get_page_at_path(&page_path).await?;
    let composition = get_composition_from_page(&page).await?;
    let rendered_tree = render_composition(composition.content).await?;
    let output_html = rendered_tree.dump_html();
    Ok(output_html)
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
