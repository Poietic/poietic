use serde_json::Map;
pub use serde_json::Value as JsonValue;
use std::{future::Future, pin::Pin, sync::Arc};

use crate::html::HtmlElement;

use self::{component_dictionary::get_component, render_error::RenderError};

mod builtins;
mod component_dictionary;
pub mod render_error;

pub type RenderParams = Map<String, JsonValue>;
pub type RenderResult = Result<HtmlElement, RenderError>;
pub type RenderFuture = Pin<Box<dyn Future<Output = RenderResult>>>;

pub trait SyncComponent: Send + Sync {
    fn render(&self, params: RenderParams) -> RenderResult;
}

pub trait AsyncComponent: Send + Sync {
    fn render(&self, params: RenderParams) -> RenderFuture;
}

#[derive(Clone)]
pub enum Component {
    Sync(Arc<dyn SyncComponent>),
    Async(Arc<dyn AsyncComponent>),
}

pub async fn render_composition(composition: JsonValue) -> RenderResult {
    let Some(JsonValue::String(component_name)) = composition.get("component") else {
        return Err(RenderError::BadParams);
    };
    let Some(JsonValue::Object(params)) = composition.get("params") else {
        return Err(RenderError::BadParams);
    };
    let Ok(component) = get_component(&component_name).await else {
        return Err(RenderError::BadParams);
    };
    match component {
        Component::Async(component) => component.render(params.clone()).await,
        Component::Sync(component) => component.render(params.clone()),
    }
}
