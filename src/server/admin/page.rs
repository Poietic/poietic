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

use actix_web::HttpResponse;

use crate::{
    component::render_composition,
    error::PoieticError,
    html::page_template::{
        page_template_config::{PageTemplateConfig, PageTemplateConfigBuilder},
        template::render_page,
    },
};

pub mod composition_builder;
pub mod index;

const ADMIN_PAGE_COMPOSITION_TEMPLATE: &str = r#"
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
                    "title": "Composition builder",
                    "target": "/composition-builder"
                }
            }
        ],
        "content": $content
    }
}
"#;

#[actix_web::get("/assets/poietic/scripts/poietic.js")]
pub async fn get_poietic_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(include_str!("page/poietic.js"))
}

pub async fn build_admin_page(content: &str) -> Result<HttpResponse, PoieticError> {
    let composition_json = ADMIN_PAGE_COMPOSITION_TEMPLATE.replace("$content", content);
    let composition_json_value = serde_json::from_str(&composition_json).unwrap();
    let composition_html = render_composition(&composition_json_value).await?;
    let output = render_page(admin_page_template_config(), composition_html);
    Ok(HttpResponse::Ok().content_type("text/html").body(output))
}

pub fn admin_page_template_config() -> PageTemplateConfig {
    PageTemplateConfigBuilder::new()
        .language("en".to_string())
        .charset()
        .title("Poietic admin".to_string())
        .add_script("/assets/poietic/scripts/poietic.js".to_string())
        .build()
}
