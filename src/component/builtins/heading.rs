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

pub struct Heading;
impl Heading {
    fn extract_importance(params: &RenderParams) -> Result<u64, RenderError> {
        let Some(JsonValue::Number(importance)) = params.get("importance") else {
            return Err(RenderError::BadParams);
        };
        let Some(importance) = importance.as_u64() else {
            return Err(RenderError::BadParams);
        };
        if !(1..=6).contains(&importance) {
            return Err(RenderError::BadParams);
        }
        Ok(importance)
    }
    fn extract_text(params: &RenderParams) -> Result<&str, RenderError> {
        match params.get("text") {
            Some(JsonValue::String(text)) => Ok(text),
            _ => Err(RenderError::BadParams),
        }
    }
}

impl SyncComponent for Heading {
    fn render(&self, params: RenderParams) -> RenderResult {
        let importance = Self::extract_importance(&params)?;
        let text = Self::extract_text(&params)?;
        let text_element = HtmlElement::create_text(text.to_string());
        Ok(HtmlElement::create_node(
            format!("h{}", importance),
            Default::default(),
            vec![text_element],
        )?)
    }
}
