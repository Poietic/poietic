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

use std::str::FromStr;

use surrealdb::sql::{Id, Thing};

use crate::database::{
    connection::connection_manager::ConnectionManager, database_error::DatabaseError, model::Page,
};

pub async fn get_page_at_path(
    connection_manager: &ConnectionManager,
    path: &str,
) -> Result<Page, DatabaseError> {
    let connection = connection_manager.get_connection().await?;
    let page: Option<Page> = connection
        .query("SELECT * FROM page WHERE path = $path")
        .bind(("path", path))
        .await?
        .take(0)?;
    match page {
        Some(page) => Ok(page),
        None => Err(DatabaseError::RecordNotFound),
    }
}

pub async fn create_page(
    connection_manager: &ConnectionManager,
    path: String,
    composition_id: String,
) -> Result<Page, DatabaseError> {
    let connection = connection_manager.get_connection().await?;
    let id = Thing::from(("page", Id::ulid()));
    let page = Page {
        id,
        composition: Thing::from_str(&composition_id).unwrap(),
        path,
    };
    connection
        .create::<Option<Page>>(&page.id)
        .content(&page)
        .await?;
    Ok(page)
}
