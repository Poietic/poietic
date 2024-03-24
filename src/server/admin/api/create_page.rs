use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use crate::{database::connection::connection_manager::ConnectionManager, error::PoieticError};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePageRequestBody {
    pub path: String,
    pub composition_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePageResponseBody {
    pub id: String,
}

#[post("/poietic/create-page")]
async fn create_page(
    connection_manager: Data<ConnectionManager>,
    body: Json<CreatePageRequestBody>,
) -> Result<impl Responder, PoieticError> {
    let body = body.into_inner();
    use crate::database::data_access::page::create_page;
    let page = create_page(connection_manager.as_ref(), body.path, body.composition_id).await?;
    Ok(HttpResponse::Ok().json(CreatePageResponseBody {
        id: page.id.to_string(),
    }))
}
