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

pub mod page_template_config;
pub mod template;

use std::collections::BTreeMap;
use std::fmt::Write;

use crate::html::html_safety::EscapeHtml;

use super::html_error::IllegalAttributeNameError;

pub struct Meta {
    attributes: BTreeMap<String, String>,
}

impl Meta {
    pub fn new(attributes: BTreeMap<String, String>) -> Result<Self, IllegalAttributeNameError> {
        if let Some(illegal_attr) = attributes
            .keys()
            .find(|attr| !ALLOWED_META_ATTRIBUTES.contains(&attr.as_str()))
        {
            return Err(IllegalAttributeNameError(illegal_attr.clone()));
        }
        Ok(Self { attributes })
    }

    pub fn dump_html(&self) -> String {
        dump_non_container_tag("meta", &self.attributes)
    }
}

pub struct Link {
    attributes: BTreeMap<String, String>,
}

impl Link {
    pub fn new(attributes: BTreeMap<String, String>) -> Result<Self, IllegalAttributeNameError> {
        if let Some(illegal_attr) = attributes
            .keys()
            .find(|attr| !ALLOWED_LINK_ATTRIBUTES.contains(&attr.as_str()))
        {
            return Err(IllegalAttributeNameError(illegal_attr.clone()));
        }
        Ok(Self { attributes })
    }

    pub fn dump_html(&self) -> String {
        dump_non_container_tag("link", &self.attributes)
    }
}

fn dump_non_container_tag(tag_name: &str, attributes: &BTreeMap<String, String>) -> String {
    let attributes_string = attributes
        .iter()
        .fold(String::new(), |mut output, (key, value)| {
            let _ = write!(
                output,
                " {}=\"{}\"",
                key.escape_html(),
                value.escape_html()
            );
            output
        });
    format!("<{}{attributes_string}/>", tag_name.escape_html())
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
    "charset",
    "content_security_policy",
    "content_type",
    "default_style",
    "x_ua_compatible",
    "refresh",
];
