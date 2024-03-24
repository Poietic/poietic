use actix_web::{
    web::{route, scope, Data, Path, ServiceConfig},
    HttpResponse, Responder, Route, Scope,
};

use crate::{
    component::render_composition,
    database::{connection::connection_manager::ConnectionManager, data_access::{composition::get_composition_from_page, page::get_page_at_path}},
    error::PoieticError,
};

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
        .service(api_route_service)
        .default_service(create_404_handler())
}

#[actix_web::get("/{page_path:.*}")]
async fn page_route_service(connection_manager: Data<ConnectionManager>, page_path: Path<String>) -> Result<impl Responder, PoieticError> {
    let connection_manager = connection_manager.into_inner();
    let page = get_page_at_path(connection_manager.as_ref(), &page_path).await?;
    let composition = get_composition_from_page(connection_manager.as_ref(), &page).await?;
    let rendered_tree = render_composition(composition.content).await?;
    let output_html = rendered_tree.dump_html();
    Ok(output_html)
}

pub fn configure_public_app(config: &mut ServiceConfig) {
    config
        .service(create_api_scope())
        .service(page_route_service)
        .default_service(create_404_handler());
}
