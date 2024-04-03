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
    component::{
        AsyncComponent, JsonValue, RenderError, RenderFuture, RenderParams,
    },
    html::{HtmlElement, HtmlNode, TextNode},
};

#[derive(Default)]
pub struct Link;

impl AsyncComponent for Link {
    fn render(&self, params: RenderParams) -> RenderFuture {
        Box::pin(async move {
            let Some(JsonValue::String(title)) = params.get("title") else {
                return Err(RenderError::BadParams);
            };

            let Some(JsonValue::String(target)) = params.get("target") else {
                return Err(RenderError::BadParams);
            };

            Ok(HtmlElement::Node(HtmlNode::new(
                "a".to_string(),
                HashMap::from([
                    ("href".to_string(), target.clone())
                ]),
                vec![
                    HtmlElement::Text(TextNode::new(title.clone()))
                ]
            )?))
        })
    }
}
