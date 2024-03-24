use actix_web::{post, web::Json, HttpResponse, Responder};
use serde::Deserialize;

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