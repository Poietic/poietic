use crate::{
    component::{JsonValue, RenderError, RenderParams, RenderResult, SyncComponent},
    html::{HtmlElement, HtmlNode, TextNode},
};

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
