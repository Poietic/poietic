use std::collections::HashMap;

use super::{
    html_safety::{
        get_attribute_blacklist, get_safe_html_tag_set, ILLEGAL_HTML_ATTRIBUTE_NAME_CHARACTERS,
    },
    HtmlElement, HtmlError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlNode {
    tag: String,
    attributes: HashMap<String, String>,
    children: Vec<HtmlElement>,
}

impl HtmlNode {
    pub fn new(
        tag: String,
        attributes: HashMap<String, String>,
        children: Vec<HtmlElement>,
    ) -> Result<Self, HtmlError> {
        Self::validate_tag(&tag)?;
        for (attribute_name, _) in &attributes {
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
            return Err(HtmlError::IllegalAttributeName);
        }
        Ok(())
    }

    fn escape_attribute_value(text: &str) -> String {
        text.replace('\\', "\\\\").replace('\"', "\\\"")
    }

    pub fn dump_html(&self) -> String {
        format!(
            "<{0}{1}>{2}</{0}>",
            self.tag,
            self.attributes
                .iter()
                .map(|(key, value)| format!(" {}=\"{}\"", key, Self::escape_attribute_value(value)))
                .collect::<String>(),
            self.children
                .iter()
                .map(HtmlElement::dump_html)
                .collect::<String>()
        )
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
    fn atribute_html_injection() {
        let text = "Lorem ipsum\">Evil injection<div attr=\"";
        let escaped = HtmlNode::escape_attribute_value(text);
        assert_eq!("Lorem ipsum\\\">Evil injection<div attr=\\\"", escaped);

        let text = "Lorem ipsum\\\">Evil injection<div attr=\\\"";
        let escaped = HtmlNode::escape_attribute_value(text);
        assert_eq!(
            "Lorem ipsum\\\\\\\">Evil injection<div attr=\\\\\\\"",
            escaped
        );
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
                Err(HtmlError::IllegalAttributeName),
                HtmlNode::validate_attribute_name(attribute_name)
            );
        }

        for attribute_name in ATTRIBUTE_BLACKLIST.iter().cloned() {
            assert_eq!(
                Err(HtmlError::IllegalAttributeName),
                HtmlNode::validate_attribute_name(attribute_name)
            );
        }
    }
}
