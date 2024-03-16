#[derive(Debug, Clone, PartialEq)]
pub struct TextNode {
    text: String,
}

impl TextNode {
    pub fn new(text: String) -> Self {
        Self { text: text }
    }
    pub fn dump_html(&self) -> String {
        self.text
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace(' ', "&nbsp;")
    }
}

#[cfg(test)]
mod test {
    use crate::html::TextNode;

    #[test]
    fn html_injection() {
        let text = "<div>EvilInjection</div>".to_string();
        let escaped = TextNode::new(text).dump_html();
        assert_eq!("&lt;div&gt;EvilInjection&lt;/div&gt;", escaped);
    }

    #[test]
    fn non_breaking_spaces() {
        let text = " ".to_string();
        let escaped = TextNode::new(text).dump_html();
        assert_eq!("&nbsp;", escaped);
    }
}
