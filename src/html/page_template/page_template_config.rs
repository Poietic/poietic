// Copyright 2024 Jakub Duda
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

use super::{Link, Meta};

pub struct PageTemplateConfig {
    pub language: String,
    pub custom_template: Option<String>,
    pub title: String,
    pub scripts: Vec<String>,
    pub links: Vec<Link>,
    pub meta_vec: Vec<Meta>
}