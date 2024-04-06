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
        render_composition_list, AsyncComponent, JsonValue, RenderError, RenderFuture,
        RenderParams, RenderResult,
    },
    html::HtmlElement,
};

pub struct ComponentList;

impl ComponentList {
    fn extract_children(
        params: &RenderParams,
    ) -> Result<&[JsonValue], RenderError> {
        match params.get("children") {
            Some(JsonValue::Array(children)) => Ok(children),
            _ => Err(RenderError::BadParams),
        }
    }
    fn build(children: Vec<HtmlElement>) -> RenderResult {
        Ok(HtmlElement::create_node(
            "div".to_string(),
            Default::default(),
            children,
        )?)
    }
}

impl AsyncComponent for ComponentList {
    fn render(&self, params: RenderParams) -> RenderFuture {
        Box::pin(async move {
            let children = Self::extract_children(&params)?;
            let rendered_children = render_composition_list(children).await?;
            Self::build(rendered_children)
        })
    }
}
