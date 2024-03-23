#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HtmlError {
    IllegalTag,
    IllegalAttributeName,
}
