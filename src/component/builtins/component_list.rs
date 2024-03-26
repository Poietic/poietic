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
    component::{
        render_composition, AsyncComponent, JsonValue, RenderError, RenderFuture, RenderParams,
    },
    html::{HtmlElement, HtmlNode},
};

#[derive(Default)]
pub struct ComponentList;

impl AsyncComponent for ComponentList {
    fn render(&self, params: RenderParams) -> RenderFuture {
        Box::pin(async move {
            let Some(JsonValue::Array(children)) = params.get("children") else {
                return Err(RenderError::BadParams);
            };
            let mut children_output = Vec::<HtmlElement>::with_capacity(children.len());
            for child in children {
                children_output.push(render_composition(child.clone()).await?);
            }
            Ok(HtmlElement::Node(HtmlNode::new(
                "div".to_string(),
                Default::default(),
                children_output,
            )?))
        })
    }
}
