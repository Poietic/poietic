mod html_error;
mod html_node;
mod html_safety;
mod text_node;

pub use self::html_error::HtmlError;
pub use self::html_node::HtmlNode;
pub use self::text_node::TextNode;

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlElement {
    Node(HtmlNode),
    Text(TextNode),
}

impl HtmlElement {
    pub fn dump_html(&self) -> String {
        match self {
            HtmlElement::Node(html_node) => html_node.dump_html(),
            HtmlElement::Text(text_node) => text_node.dump_html(),
        }
    }
}
