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

use std::fmt::Display;

use actix_web::ResponseError;

use crate::{
    component::render_error::RenderError, database::database_error::DatabaseError, html::HtmlError,
};

#[derive(Debug, Clone)]
pub enum PoieticError {
    HtmlError(HtmlError),
    RenderError(RenderError),
    DatabaseError(DatabaseError),
}

impl From<HtmlError> for PoieticError {
    fn from(inner: HtmlError) -> Self {
        PoieticError::HtmlError(inner)
    }
}

impl From<RenderError> for PoieticError {
    fn from(inner: RenderError) -> Self {
        match inner {
            RenderError::HtmlError(inner) => PoieticError::HtmlError(inner),
            other => PoieticError::RenderError(other),
        }
    }
}

impl From<DatabaseError> for PoieticError {
    fn from(inner: DatabaseError) -> Self {
        PoieticError::DatabaseError(inner)
    }
}

impl Display for PoieticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for PoieticError {}
