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

#[derive(Debug, Clone, PartialEq)]
pub struct TextNode {
    text: String,
}

impl TextNode {
    pub fn new(text: String) -> Self {
        Self { text: text }
    }
    pub fn dump_html(&self) -> String {
        self.text.replace('<', "&lt;").replace('>', "&gt;")
    }
}

#[cfg(test)]
mod test {
    use crate::html::TextNode;

    #[test]
    fn html_injection() {
        let text = "<div>EvilInjection</div>".to_string();
        let escaped = TextNode::new(text).dump_html();
        assert_eq!("&lt;div&gt;EvilInjection&lt;/div&gt;", escaped);
    }
}
