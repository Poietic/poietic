use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use crate::html::{HtmlElement, HtmlError};

mod builtins;
mod component_dictionary;

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

pub type RenderParams = HashMap<String, serde_json::Value>;
pub type RenderResult = Result<HtmlElement, RenderError>;

pub trait SyncComponent: Send + Sync {
    fn render(&self, params: RenderParams) -> RenderResult;
}

pub trait AsyncComponent: Send + Sync {
    fn render(&self, params: RenderParams) -> Pin<Box<dyn Future<Output = RenderResult>>>;
}

#[derive(Clone)]
pub enum Component {
    Sync(Arc<dyn SyncComponent>),
    Async(Arc<dyn AsyncComponent>),
}
