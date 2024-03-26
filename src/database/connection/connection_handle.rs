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

use std::ops::Deref;

use surrealdb::{engine::any::Any, Surreal};

use super::connection_manager::{
    pooled::PooledConnectionManager, unpooled::UnpooledConnectionManager,
};

pub enum ConnectionHandle {
    Pooled(PooledConnectionHandle),
    Unpooled(UnpooledConnectionHandle),
}

pub struct PooledConnectionHandle {
    connection_manager: Option<PooledConnectionManager>,
    connection: Surreal<Any>,
}

pub struct UnpooledConnectionHandle {
    connection_manager: Option<UnpooledConnectionManager>,
    connection: Surreal<Any>,
}

impl ConnectionHandle {
    pub fn new_pooled(
        connection_manager: Option<PooledConnectionManager>,
        connection: Surreal<Any>,
    ) -> Self {
        Self::Pooled(PooledConnectionHandle {
            connection_manager,
            connection,
        })
    }
    pub fn new_unpooled(
        connection_manager: UnpooledConnectionManager,
        connection: Surreal<Any>,
    ) -> Self {
        Self::Unpooled(UnpooledConnectionHandle {
            connection_manager: Some(connection_manager),
            connection,
        })
    }
}

impl Deref for ConnectionHandle {
    type Target = Surreal<Any>;
    fn deref(&self) -> &Self::Target {
        match self {
            ConnectionHandle::Pooled(pooled) => &pooled.connection,
            ConnectionHandle::Unpooled(unpooled) => &unpooled.connection,
        }
    }
}

impl Drop for ConnectionHandle {
    fn drop(&mut self) {
        match self {
            ConnectionHandle::Pooled(pooled) => match pooled.connection_manager.take() {
                Some(connection_manager) => {
                    connection_manager
                        .clone()
                        .release_connection(pooled.connection.clone());
                }
                None => {}
            },
            ConnectionHandle::Unpooled(unpooled) => match unpooled.connection_manager.take() {
                Some(connection_manager) => {
                    connection_manager
                        .clone()
                        .release_connection(unpooled.connection.clone());
                }
                None => {}
            },
        }
    }
}
