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
pub struct IllegalAttributeNameError(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlError {
    IllegalTag,
    IllegalAttributeName(IllegalAttributeNameError),
}

impl HtmlError {
    pub fn illegal_attribute(tag: String) -> Self {
        Self::IllegalAttributeName(IllegalAttributeNameError(tag))
    }
}
