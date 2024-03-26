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
