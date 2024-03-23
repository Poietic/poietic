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
