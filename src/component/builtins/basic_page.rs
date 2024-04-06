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

use crate::{
    component::{
        render_composition_list, AsyncComponent, JsonValue, RenderError, RenderFuture,
        RenderParams, RenderResult,
    },
    html::HtmlElement,
};

pub struct BasicPage;

impl BasicPage {
    fn extract_title(params: &RenderParams) -> Result<&str, RenderError> {
        match params.get("title") {
            Some(JsonValue::String(title)) => Ok(title),
            _ => Err(RenderError::BadParams),
        }
    }
    fn is_not_poietic_link(value: &JsonValue) -> bool {
        let Some(object) = value.as_object() else {
            return true;
        };
        let Some(JsonValue::String(component_type)) = object.get("component") else {
            return true;
        };
        component_type != "poietic:Link"
    }
    fn extract_nav_links(params: &RenderParams) -> Result<&[JsonValue], RenderError> {
        let Some(JsonValue::Array(nav_links)) = params.get("nav_links") else {
            return Err(RenderError::BadParams);
        };
        if nav_links.iter().any(Self::is_not_poietic_link) {
            return Err(RenderError::BadParams);
        }
        return Ok(nav_links);
    }
    fn extract_content(params: &RenderParams) -> Result<&[JsonValue], RenderError> {
        match params.get("content") {
            Some(JsonValue::Array(content)) => Ok(content),
            _ => Err(RenderError::BadParams),
        }
    }
    fn build_header(title: &str, nav_links: Vec<HtmlElement>) -> RenderResult {
        let title_text = HtmlElement::create_text(title.to_string());
        let title =
            HtmlElement::create_node("h1".to_string(), Default::default(), vec![title_text])?;
        let nav = HtmlElement::create_node("nav".to_string(), Default::default(), nav_links)?;
        Ok(HtmlElement::create_node(
            "header".to_string(),
            Default::default(),
            vec![title, nav],
        )?)
    }
    fn build(header: HtmlElement, content: Vec<HtmlElement>) -> RenderResult {
        let main = HtmlElement::create_node("main".to_string(), Default::default(), content)?;
        Ok(HtmlElement::create_node(
            "div".to_string(),
            Default::default(),
            vec![header, main],
        )?)
    }
}

impl AsyncComponent for BasicPage {
    fn render(&self, params: RenderParams) -> RenderFuture {
        Box::pin(async move {
            let title = Self::extract_title(&params)?;
            let nav_links = Self::extract_nav_links(&params)?;
            let content = Self::extract_content(&params)?;

            let rendered_nav_links = render_composition_list(nav_links).await?;
            let rendered_content = render_composition_list(content).await?;
            let header = Self::build_header(title, rendered_nav_links)?;

            Self::build(header, rendered_content)
        })
    }
}
