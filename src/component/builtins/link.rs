// Copyright 2024 Lech Mazur, Adam Wasiak
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

use std::collections::HashMap;

use crate::{
    component::{JsonValue, RenderError, RenderParams, RenderResult, SyncComponent},
    html::HtmlElement,
};

pub struct Link;

impl Link {
    fn extract_title(params: &RenderParams) -> Result<&str, RenderError> {
        match params.get("title") {
            Some(JsonValue::String(title)) => Ok(title),
            _ => Err(RenderError::BadParams),
        }
    }
    fn extract_target(params: &RenderParams) -> Result<&str, RenderError> {
        match params.get("target") {
            Some(JsonValue::String(target)) => Ok(target),
            _ => Err(RenderError::BadParams),
        }
    }
}

impl SyncComponent for Link {
    fn render(&self, params: RenderParams) -> RenderResult {
        let title = Self::extract_title(&params)?;
        let target = Self::extract_target(&params)?;
        Ok(HtmlElement::create_node(
            "a".to_string(),
            HashMap::from([("href".to_string(), target.to_string())]),
            vec![HtmlElement::create_text(title.to_string())],
        )?)
    }
}
