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
    html::{HtmlElement, HtmlNode, TextNode},
};

#[derive(Default)]
pub struct Heading;

impl SyncComponent for Heading {
    fn render(&self, params: RenderParams) -> RenderResult {
        let Some(JsonValue::Number(importance)) = params.get("importance") else {
            return Err(RenderError::BadParams);
        };
        let Some(importance) = importance.as_u64() else {
            return Err(RenderError::BadParams);
        };
        if !(1..=6).contains(&importance) {
            return Err(RenderError::BadParams);
        }
        let Some(JsonValue::String(text)) = params.get("text") else {
            return Err(RenderError::BadParams);
        };
        let text_element = HtmlElement::Text(TextNode::new(text.clone()));
        Ok(HtmlElement::Node(HtmlNode::new(
            format!("h{}", importance),
            Default::default(),
            vec![text_element],
        )?))
    }
}
