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

mod html_error;
mod html_node;
mod html_safety;
mod text_node;

pub use self::html_error::HtmlError;
pub use self::html_node::HtmlNode;
pub use self::text_node::TextNode;

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlElement {
    Node(HtmlNode),
    Text(TextNode),
}

impl HtmlElement {
    pub fn dump_html(&self) -> String {
        match self {
            HtmlElement::Node(html_node) => html_node.dump_html(),
            HtmlElement::Text(text_node) => text_node.dump_html(),
        }
    }
}
