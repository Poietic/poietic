use crate::html::HtmlError;

#[derive(Debug, Clone, PartialEq)]
pub enum RenderError {
    BadParams,
    HtmlError(HtmlError),
    Unknown,
}

impl From<HtmlError> for RenderError {
    fn from(html_error: HtmlError) -> Self {
        Self::HtmlError(html_error)
    }
}
