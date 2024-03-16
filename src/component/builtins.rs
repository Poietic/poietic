use crate::html::{HtmlElement, HtmlNode, TextNode};

use super::{
    render_composition, AsyncComponent, JsonValue, RenderError, RenderFuture, RenderParams,
    RenderResult, SyncComponent,
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

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn composition_rendering() {
        let composition: JsonValue = serde_json::from_str(
            "{
                \"component\": \"poietic:ComponentList\",
                \"params\": {
                    \"children\": [
                        {
                            \"component\": \"poietic:Heading\",
                            \"params\": {
                                \"importance\": 1,
                                \"text\": \"Lorem ipsum\"
                            }
                        },
                        {
                            \"component\": \"poietic:Paragraph\",
                            \"params\": {
                                \"content\": \"Lorem ipsum, dolor sit amet.\"
                            }
                        }
                    ]
                }
            }",
        )
        .unwrap();
        let expected_output =
            "<div><h1>Lorem&nbsp;ipsum</h1><p>Lorem&nbsp;ipsum,&nbsp;dolor&nbsp;sit&nbsp;amet.</p></div>";
        let output = render_composition(composition).await.unwrap().dump_html();
        assert_eq!(expected_output, output);
    }
}
