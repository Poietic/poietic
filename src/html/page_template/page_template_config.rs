// Copyright 2024 Jakub Duda
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

use std::collections::HashMap;

use super::{Link, Meta};

pub struct PageTemplateConfig {
    pub language: Option<String>,
    pub custom_template: Option<String>,
    pub title: String,
    pub scripts: Vec<String>,
    pub links: Vec<Link>,
    pub meta_vec: Vec<Meta>
}

pub struct PageTemplateConfigBuilder {
    language: Option<String>,
    custom_template: Option<String>,
    title: Option<String>,
    scripts: Vec<String>,
    links: Vec<Link>,
    meta_vec: Vec<Meta>
}

impl PageTemplateConfigBuilder {
    pub fn new() -> Self {
        Self {
            language: None,
            custom_template: None,
            title: None,
            scripts: vec![],
            links: vec![],
            meta_vec: vec![]
        }
    }

    pub fn language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    pub fn custom_template(mut self, custom_template: String) -> Self {
        self.custom_template = Some(custom_template);
        self
    }
    
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
    
    pub fn charset(mut self) -> Self {
        let mut attributes = HashMap::new();
        attributes.insert("charset".to_string(), "UTF-8".to_string());
        self.meta_vec.push(
            Meta { attributes }
        );
        self
    }
    
    pub fn add_meta(mut self, meta: Meta) -> Self {
        self.meta_vec.push(meta);
        self
    }

    pub fn add_link(mut self, link: Link) -> Self {
        self.links.push(link);
        self
    }

    pub fn add_stylesheet(mut self, href: String) -> Self {
        let mut attributes = HashMap::new();
        attributes.insert("rel".to_string(), "stylesheet".to_string());
        attributes.insert("type".to_string(), "text/css".to_string());
        attributes.insert("href".to_string(), href);
        self.links.push(
            Link { attributes }
        );
        self
    }

    pub fn build(mut self) -> Result<PageTemplateConfig, ()> {
        return Ok(PageTemplateConfig {
            language: self.language,
            custom_template: self.custom_template,
            title: self.title.unwrap_or("".to_string()),
            scripts: self.scripts,
            links: self.links,
            meta_vec: self.meta_vec
        })
    }
}