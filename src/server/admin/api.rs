use actix_web::{post, web::Path, Responder};

mod create_composition;
pub use create_composition::*;
mod create_page;
pub use create_page::*;

#[post("/{namespace}/{api_function}")]
pub async fn api_route_service(path: Path<(String, String)>) -> impl Responder {
    let (namespace, api_function) = path.into_inner();
    todo!("Handle API routes");
    ""
}
