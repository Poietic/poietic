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

use crate::{
    component::{JsonValue, RenderError, RenderParams, RenderResult, SyncComponent},
    html::HtmlElement,
};

pub struct Paragraph;

impl Paragraph {
    fn extract_content(params: &RenderParams) -> Result<&str, RenderError> {
        match params.get("content") {
            Some(JsonValue::String(content)) => Ok(content),
            _ => Err(RenderError::BadParams),
        }
    }
}

impl SyncComponent for Paragraph {
    fn render(&self, params: RenderParams) -> RenderResult {
        let content = Self::extract_content(&params)?;
        let text_element = HtmlElement::create_text(content.to_string());
        Ok(HtmlElement::create_node(
            "p".to_string(),
            Default::default(),
            vec![text_element],
        )?)
    }
}
