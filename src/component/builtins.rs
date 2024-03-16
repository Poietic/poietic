use serde_json::Value as JsonValue;

use crate::html::{HtmlElement, HtmlNode, TextNode};

use super::{SyncComponent, RenderError, RenderParams, RenderResult};

#[derive(Default)]
pub struct Paragraph;

impl SyncComponent for Paragraph {
    fn render(&self, params: RenderParams) -> RenderResult {
        let Some(JsonValue::String(content)) = params.get("content") else {
            return Err(RenderError::BadParams);
        };
        let text_element = HtmlElement::Text(TextNode::new(content.clone()));
        Ok(HtmlElement::Node(HtmlNode::new(
            "p".to_string(),
            Default::default(),
            vec![text_element],
        )?))
    }
}

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