// Copyright 2024 Lech Mazur, Adam Wasiak
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

use std::{collections::HashMap, sync::OnceLock};

use tokio::sync::RwLock;

use super::{builtins::get_builtin_components, Component};

pub struct ComponentNamespace {
    components: HashMap<String, Component>,
}

impl ComponentNamespace {
    pub fn new(components: HashMap<String, Component>) -> Self {
        Self { components }
    }
}

struct ComponentDictionary {
    component_namespaces: HashMap<String, ComponentNamespace>,
}

impl ComponentDictionary {
    fn new(component_namespaces: HashMap<String, ComponentNamespace>) -> Self {
        Self {
            component_namespaces,
        }
    }
}

static COMPONENTS: OnceLock<RwLock<ComponentDictionary>> = OnceLock::new();

pub enum ComponentLookupError {
    BadComponentName,
    NamespaceNotFound,
    ComponentNotFound,
}

fn create_component_dictionary() -> RwLock<ComponentDictionary> {
    let builtin_components = get_builtin_components();
    let poietic_namespace = ComponentNamespace::new(builtin_components.into_iter().collect());
    RwLock::new(ComponentDictionary::new(
        [("poietic".to_string(), poietic_namespace)].into(),
    ))
}

pub async fn get_component(name: &str) -> Result<Component, ComponentLookupError> {
    let Some((namespace_name, component_name)) = name.split_once(':') else {
        return Err(ComponentLookupError::BadComponentName);
    };
    let component_map = COMPONENTS
        .get_or_init(create_component_dictionary)
        .read()
        .await;
    let Some(namespace) = component_map.component_namespaces.get(namespace_name) else {
        return Err(ComponentLookupError::NamespaceNotFound);
    };
    let Some(component) = namespace.components.get(component_name) else {
        return Err(ComponentLookupError::ComponentNotFound);
    };
    Ok(component.clone())
}

pub enum AddComponentNamespaceError {
    NameTaken,
}

pub async fn add_component_namespace(
    name: String,
    namespace: ComponentNamespace,
) -> Result<(), AddComponentNamespaceError> {
    let mut component_map = COMPONENTS
        .get_or_init(create_component_dictionary)
        .write()
        .await;
    if component_map.component_namespaces.contains_key(&name) {
        return Err(AddComponentNamespaceError::NameTaken);
    }
    component_map.component_namespaces.insert(name, namespace);
    Ok(())
}
