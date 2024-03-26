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

use std::sync::OnceLock;

use self::base_config::BaseConfig;

pub mod base_config;
pub mod database_config;
pub mod http_server_config;

static CONFIG: OnceLock<BaseConfig> = OnceLock::new();

pub fn get_config() -> &'static BaseConfig {
    CONFIG.get_or_init(|| BaseConfig::load().unwrap())
}
