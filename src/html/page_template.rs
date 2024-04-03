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

mod page_template_config;

use std::collections::HashMap;

pub struct Meta {
    attributes: HashMap<String, String>,
}

impl Meta {
    pub fn new(attributes: HashMap<String, String>) -> Result<Self, ()> {
        if attributes
            .keys()
            .any(|attr| !ALLOWED_META_ATTRIBUTES.contains(&attr.as_str()))
        {
            return Err(());
        }
        Ok(Self { attributes })
    }

    pub fn dump_html(&self) -> String {
        dump_non_container_tag("meta", &self.attributes)
    }
}

pub struct Link {
    attributes: HashMap<String, String>,
}

impl Link {
    pub fn new(attributes: HashMap<String, String>) -> Result<Self, ()> {
        if attributes
            .keys()
            .any(|attr| !ALLOWED_LINK_ATTRIBUTES.contains(&attr.as_str()))
        {
            return Err(());
        }
        Ok(Self { attributes })
    }

    pub fn dump_html(&self) -> String {
        dump_non_container_tag("link", &self.attributes)
    }
}

fn dump_non_container_tag(tag_name: &str, attributes: &HashMap<String, String>) -> String {
    let attributes_string = attributes
        .iter()
        .map(|(key, value)| format!(" {}=\"{}\"", key.escape_default(), value.escape_default()))
        .collect::<String>();
    format!("<{tag_name} {attributes_string}/>")
}

static ALLOWED_LINK_ATTRIBUTES: &[&str] = &[
    "crossorigin",
    "href",
    "hreflang",
    "media",
    "referrerpolicy",
    "rel",
    "sizes",
    "title",
    "type",
];

static ALLOWED_META_ATTRIBUTES: &[&str] = &[
    "http_equiv",
    "content",
    "content_security_policy",
    "content_type",
    "default_style",
    "x_ua_compatible",
    "refresh",
];
