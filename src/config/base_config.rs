// Copyright 2024 Jakub Duda, Lech Mazur
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

use std::fs::read_to_string;

use serde::Deserialize;

use super::{database_config::DatabaseConfig, http_server_config::HttpServerConfig};

#[derive(Clone, Deserialize)]
pub struct BaseConfig {
    pub public: HttpServerConfig,
    pub admin: HttpServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug)]
pub enum ConfigLoadError {
    IncorrectConfig,
    CannotAccessFile,
}

impl BaseConfig {
    pub fn load() -> Result<Self, ConfigLoadError> {
        let contents =
            read_to_string("config.yaml").map_err(|_| ConfigLoadError::CannotAccessFile)?;

        serde_yaml::from_str(contents.as_str()).map_err(|_| ConfigLoadError::IncorrectConfig)
    }
}
