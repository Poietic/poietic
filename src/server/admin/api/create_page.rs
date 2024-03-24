use actix_web::{post, web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::error::PoieticError;

#[derive(Debug, Deserialize)]
struct CreatePageRequestBody {
    path: String,
    composition_id: String,
}

#[derive(Debug, Serialize)]
struct CreatePageResponseBody {
    id: String,
}

#[post("/poietic/create-page")]
async fn create_page(body: Json<CreatePageRequestBody>) -> Result<impl Responder, PoieticError> {
    let body = body.into_inner();
    use crate::database::data_access::page::create_page;
    let page = create_page(body.path, body.composition_id).await?;
    Ok(HttpResponse::Ok().json(CreatePageResponseBody {
        id: page.id.to_string(),
    }))
}
