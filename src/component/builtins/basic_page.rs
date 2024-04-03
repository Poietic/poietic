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
        render_composition, AsyncComponent, JsonValue, RenderError, RenderFuture, RenderParams,
    },
    html::{HtmlElement, HtmlNode, TextNode},
};

#[derive(Default)]
pub struct BasicPage;

impl AsyncComponent for BasicPage {
    fn render(&self, params: RenderParams) -> RenderFuture {
        Box::pin(async move {
            let Some(JsonValue::String(title)) = params.get("title") else {
                return Err(RenderError::BadParams);
            };

            let Some(JsonValue::Array(nav_links)) = params.get("nav_links") else {
                return Err(RenderError::BadParams);
            };

            let mut nav_link_output = Vec::<HtmlElement>::with_capacity(nav_links.len());

            for nav_link in nav_links {
                if let JsonValue::Object(obj) = nav_link {
                    if let Some(component_value) = obj.get("component") {
                        if let JsonValue::String(component_str) = component_value {
                            if component_str == "poietic:Link" {
                                nav_link_output.push(render_composition(nav_link.clone()).await?);
                                continue;
                            }
                        }
                    }
                }
                return Err(RenderError::BadParams);
            }

            let Some(JsonValue::Array(content)) = params.get("content") else {
                return Err(RenderError::BadParams);
            };

            let mut content_output = Vec::<HtmlElement>::with_capacity(content.len());
            for child in content {
                content_output.push(render_composition(child.clone()).await?);
            }

            Ok(HtmlElement::Node(HtmlNode::new(
                "div".to_string(),
                Default::default(),
                vec![
                    HtmlElement::Node(HtmlNode::new(
                        "header".to_string(),
                        Default::default(),
                        vec![
                            HtmlElement::Node(HtmlNode::new(
                                "h1".to_string(),
                                Default::default(),
                                vec![HtmlElement::Text(TextNode::new(title.clone()))],
                            )?),
                            HtmlElement::Node(HtmlNode::new(
                                "nav".to_string(),
                                Default::default(),
                                nav_link_output
                            )?),
                        ]
                    )?),
                    HtmlElement::Node(HtmlNode::new(
                        "main".to_string(),
                        Default::default(),
                        content_output,
                    )?),
                ],
            )?))
        })
    }
}
