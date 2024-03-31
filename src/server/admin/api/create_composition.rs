// Copyright 2024 Lech Mazur
//
// This file is part of Poietic.
//
// Poietic is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License, version 2,
// as published by the Free Software Foundation.
//
// Poietic is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Poietic. If not, see <https://www.gnu.org/licenses/>.

use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{database::connection::connection_manager::ConnectionManager, error::PoieticError};
use crate::database::data_access::composition::CompositionRepository;

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
    let composition = connection_manager.create_composition(body.content).await?;
    Ok(HttpResponse::Ok().json(CreateCompositionResponseBody {
        id: composition.id.to_string(),
    }))
}
