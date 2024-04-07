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

use std::collections::BTreeMap;

use super::{
    html_safety::{
        get_attribute_blacklist, get_safe_html_tag_set, EscapeHtml,
        ILLEGAL_HTML_ATTRIBUTE_NAME_CHARACTERS,
    },
    HtmlElement, HtmlError,
};

use std::fmt::Write;

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlNode {
    tag: String,
    attributes: BTreeMap<String, String>,
    children: Vec<HtmlElement>,
}

impl HtmlNode {
    pub fn new(
        tag: String,
        attributes: BTreeMap<String, String>,
        children: Vec<HtmlElement>,
    ) -> Result<Self, HtmlError> {
        Self::validate_tag(&tag)?;
        for attribute_name in attributes.keys() {
            Self::validate_attribute_name(attribute_name)?;
        }
        Ok(Self {
            tag,
            attributes,
            children,
        })
    }

    fn validate_tag(tag: &str) -> Result<(), HtmlError> {
        if !get_safe_html_tag_set().contains(&tag) {
            Err(HtmlError::IllegalTag)
        } else {
            Ok(())
        }
    }

    fn validate_attribute_name(name: &str) -> Result<(), HtmlError> {
        if name.is_empty()
            || name.contains(|character: char| {
                character.is_control()
                    || character.is_whitespace()
                    || ILLEGAL_HTML_ATTRIBUTE_NAME_CHARACTERS.contains(&character)
            })
            || get_attribute_blacklist().contains(name)
        {
            return Err(HtmlError::illegal_attribute(name.to_owned()));
        }
        Ok(())
    }

    pub fn dump_html(&self) -> String {
        format!(
            "<{0}{1}>{2}</{0}>",
            self.tag,
            self.attributes
                .iter()
                .fold(String::new(), |mut acc, (key, value)| {
                    let _ = write!(acc, " {}=\"{}\"", key, value.escape_html());
                    acc
                }),
            self.children
                .iter()
                .map(HtmlElement::dump_html)
                .collect::<String>()
        )
    }

    pub fn append_class(&mut self, name: &str) -> Result<(), HtmlError> {
        if name.contains(' ') {
            return Err(HtmlError::IllegalAttributeValue);
        }
        let new_class_attr = match self.attributes.get("class") {
            Some(prev_class_attr) => format!("{} {}", prev_class_attr, name),
            None => name.to_string(),
        };
        self.attributes.insert("class".to_string(), new_class_attr);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::html::{html_node::HtmlError, html_safety::ATTRIBUTE_BLACKLIST, HtmlNode};

    #[test]
    fn create_html_node() {
        let node = HtmlNode::new("p".to_string(), [].into(), [].into());
        assert!(node.is_ok());
    }

    #[test]
    fn unsafe_tag() {
        for tag in ["script", "style"] {
            assert_eq!(Err(HtmlError::IllegalTag), HtmlNode::validate_tag(tag));
        }
    }

    #[test]
    fn unsafe_attribute_name() {
        for attribute_name in ["", "\"", "'", "a/", "x=", ">", "\0", "<"] {
            assert_eq!(
                Err(HtmlError::illegal_attribute(attribute_name.to_owned())),
                HtmlNode::validate_attribute_name(attribute_name)
            );
        }

        for attribute_name in ATTRIBUTE_BLACKLIST.iter().cloned() {
            assert_eq!(
                Err(HtmlError::illegal_attribute(attribute_name.to_owned())),
                HtmlNode::validate_attribute_name(attribute_name)
            );
        }
    }
}
