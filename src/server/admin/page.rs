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

use actix_web::{web::Path, HttpResponse};
use tokio::{fs::File, io::AsyncReadExt};

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

#[actix_web::get("/assets/poietic/scripts/{script_name:[0-9A-Za-z_]+[.]js}")]
pub async fn get_script(script_name: Path<String>) -> HttpResponse {
    let Ok(mut file) = File::open(format!("assets/admin/scripts/{}", script_name)).await else {
        return HttpResponse::NotFound().finish();
    };
    let mut file_content = String::new();
    let Ok(_) = file.read_to_string(&mut file_content).await else {
        return HttpResponse::InternalServerError().finish();
    };
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(file_content)
}

#[actix_web::get("/assets/poietic/styles/{style_name:[0-9A-Za-z_]+[.]css}")]
pub async fn get_style(style_name: Path<String>) -> HttpResponse {
    let Ok(mut file) = File::open(format!("assets/admin/styles/{}", style_name)).await else {
        return HttpResponse::NotFound().finish();
    };
    let mut file_content = String::new();
    let Ok(_) = file.read_to_string(&mut file_content).await else {
        return HttpResponse::InternalServerError().finish();
    };
    HttpResponse::Ok()
        .content_type("text/css")
        .body(file_content)
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
        .add_stylesheet("/assets/poietic/styles/poietic.css".to_string())
        .build()
}
