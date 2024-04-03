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
                    },
                    {
                        "component": "poietic:Link",
                        "params": {
                            "title": "Home",
                            "target": "/"
                        }
                    }
                ]
            }
        }"#,
    )
    .unwrap();
    let expected_output =
        "<div><h1>Lorem ipsum</h1><p>Lorem ipsum, dolor sit amet.</p><a href=\"/\">Home</a></div>";
    let output = render_composition(composition).await.unwrap().dump_html();
    assert_eq!(expected_output, output);
}

#[tokio::test]
async fn basic_page() {
    let basic_page: JsonValue = serde_json::from_str(
        r#"
        {
            "component": "poietic:BasicPage",
            "params": {
                "title": "Poietic",
                "nav_links": [
                    {
                        "component": "poietic:Link",
                        "params": {
                            "title": "Home",
                            "target": "/"
                        }
                    },
                    {
                        "component": "poietic:Link",
                        "params": {
                            "title": "About us",
                            "target": "/about"
                        }
                    }
                ],
                "content": [
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

    let expected_output = "<div><header><h1>Poietic</h1><nav><a href=\"/\">Home</a><a href=\"/about\">About us</a></nav></header><main><p>Lorem ipsum, dolor sit amet.</p></main></div>";
    let output = render_composition(basic_page).await.unwrap().dump_html();
    assert_eq!(expected_output, output);
}
