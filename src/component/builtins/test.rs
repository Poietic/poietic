use crate::component::{render_composition, JsonValue};

#[tokio::test]
async fn composition_rendering() {
    let composition: JsonValue = serde_json::from_str(
        r#"{
            "component": "poietic:ComponentList",
            "params": {
                "children": [
                    {
                        "component": "poietic:Heading",
                        "params": {
                            "importance": 1,
                            "text": "Lorem ipsum"
                        }
                    },
                    {
                        "component": "poietic:Paragraph",
                        "params": {
                            "content": "Lorem ipsum, dolor sit amet."
                        }
                    }
                ]
            }
        }"#,
    )
    .unwrap();
    let expected_output =
        "<div><h1>Lorem ipsum</h1><p>Lorem ipsum, dolor sit amet.</p></div>";
    let output = render_composition(composition).await.unwrap().dump_html();
    assert_eq!(expected_output, output);
}
