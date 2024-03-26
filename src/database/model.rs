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

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Thing,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Composition {
    pub id: Thing,
    pub content: JsonValue,
    pub last_modified_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub id: Thing,
    pub path: String,
    pub composition: Thing,
}
