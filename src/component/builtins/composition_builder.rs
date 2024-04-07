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

use crate::{
    component::{RenderParams, RenderResult, SyncComponent},
    html::HtmlElement,
};

pub struct CompositionBuilder;

impl SyncComponent for CompositionBuilder {
    fn render(&self, _: RenderParams) -> RenderResult {
        Ok(HtmlElement::create_node(
            "div".to_string(),
            Default::default(),
            vec![],
        )?)
    }
}
