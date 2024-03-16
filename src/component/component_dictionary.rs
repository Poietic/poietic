use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use tokio::sync::RwLock;

use super::{
    builtins::{Heading, Paragraph},
    Component,
};

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
    let builtin_components: &[(String, Component)] = &[
        (
            "Heading".to_string(),
            Component::Sync(Arc::new(Heading::default())),
        ),
        (
            "Paragraph".to_string(),
            Component::Sync(Arc::new(Paragraph::default())),
        ),
    ];
    let poietic_namespace = ComponentNamespace::new(builtin_components.iter().cloned().collect());
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
