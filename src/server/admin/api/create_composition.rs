use actix_web::{post, web::{Data, Json}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{database::connection::connection_manager::ConnectionManager, error::PoieticError};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCompositionRequestBody {
    pub content: JsonValue,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCompositionResponseBody {
    pub id: String,
}

#[post("/poietic/create-composition")]
async fn create_composition(
    connection_manager: Data<ConnectionManager>,
    body: Json<CreateCompositionRequestBody>,
) -> Result<impl Responder, PoieticError> {
    let body = body.into_inner();
    use crate::database::data_access::composition::create_composition;
    let composition = create_composition(&connection_manager, body.content).await?;
    Ok(HttpResponse::Ok().json(CreateCompositionResponseBody {
        id: composition.id.to_string(),
    }))
}
