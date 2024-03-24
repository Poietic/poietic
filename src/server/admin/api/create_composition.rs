use actix_web::{post, web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::error::PoieticError;

#[derive(Debug, Deserialize)]
struct CreateCompositionRequestBody {
    content: JsonValue,
}

#[derive(Debug, Serialize)]
struct CreateCompositionResponseBody {
    id: String,
}

#[post("/poietic/create-composition")]
async fn create_composition(
    body: Json<CreateCompositionRequestBody>,
) -> Result<impl Responder, PoieticError> {
    let body = body.into_inner();
    use crate::database::data_access::composition::create_composition;
    let composition = create_composition(body.content).await?;
    Ok(HttpResponse::Ok().json(CreateCompositionResponseBody {
        id: composition.id.to_string(),
    }))
}
