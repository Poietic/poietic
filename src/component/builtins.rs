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

use std::sync::Arc;

use crate::component::builtins::composition_builder::CompositionBuilder;

use self::{
    basic_page::BasicPage, component_list::ComponentList, heading::Heading, link::Link,
    paragraph::Paragraph,
};

use super::Component;

pub mod basic_page;
pub mod component_list;
pub mod composition_builder;
pub mod heading;
pub mod link;
pub mod paragraph;
#[cfg(test)]
mod test;

pub fn get_builtin_components() -> Vec<(String, Component)> {
    vec![
        ("Heading".to_string(), Component::Sync(Arc::new(Heading))),
        (
            "Paragraph".to_string(),
            Component::Sync(Arc::new(Paragraph)),
        ),
        (
            "ComponentList".to_string(),
            Component::Async(Arc::new(ComponentList)),
        ),
        ("Link".to_string(), Component::Sync(Arc::new(Link))),
        (
            "CompositionBuilder".to_string(),
            Component::Sync(Arc::new(CompositionBuilder)),
        ),
        (
            "BasicPage".to_string(),
            Component::Async(Arc::new(BasicPage)),
        ),
    ]
}
