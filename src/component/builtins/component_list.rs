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
