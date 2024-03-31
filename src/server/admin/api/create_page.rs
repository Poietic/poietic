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

use crate::{database::connection::connection_manager::ConnectionManager, error::PoieticError};
use crate::database::data_access::page::PageRepository;

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
    let page = connection_manager.create_page(body.path, body.composition_id).await?;
    Ok(HttpResponse::Ok().json(CreatePageResponseBody {
        id: page.id.to_string(),
    }))
}
