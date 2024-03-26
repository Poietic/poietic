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

use std::time::SystemTime;

use chrono::DateTime;
use serde_json::Value as JsonValue;
use surrealdb::sql::{Id, Thing};

use crate::database::{
    connection::connection_manager::ConnectionManager,
    database_error::DatabaseError,
    model::{Composition, Page},
};

pub async fn get_composition_from_page(
    connection_manager: &ConnectionManager,
    page: &Page,
) -> Result<Composition, DatabaseError> {
    let connection = connection_manager.get_connection().await?;
    let page: Option<Composition> = connection.select(&page.composition).await?;
    match page {
        Some(page) => Ok(page),
        None => Err(DatabaseError::RecordNotFound),
    }
}

pub async fn create_composition(
    connection_manager: &ConnectionManager,
    content: JsonValue,
) -> Result<Composition, DatabaseError> {
    let connection = connection_manager.get_connection().await?;
    let id = Thing::from(("composition", Id::ulid()));
    let current_time = DateTime::from(SystemTime::now());
    let composition = Composition {
        id,
        content,
        last_modified_at: current_time.into(),
    };
    connection
        .create::<Option<Composition>>(&composition.id)
        .content(&composition)
        .await?;
    Ok(composition)
}
