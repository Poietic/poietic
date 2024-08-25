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

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use surrealdb::{
    engine::any::{self, Any},
    Surreal,
};

use crate::database::{
    connection::connection_handle::ConnectionHandle, database_error::DatabaseError,
};

#[derive(Debug, Clone)]
pub struct PooledConnectionManager {
    connections: Arc<Mutex<VecDeque<Surreal<Any>>>>,
    address: Arc<str>,
}

impl PooledConnectionManager {
    pub async fn new(address: &str, pool_size: usize) -> Result<Self, DatabaseError> {
        let mut connections = VecDeque::<Surreal<Any>>::with_capacity(pool_size);
        for _ in 0..pool_size {
            connections.push_back(Self::create_connection(address).await?)
        }
        let connection_manager = PooledConnectionManager {
            connections: Arc::new(Mutex::new(connections)),
            address: Arc::from(address),
        };
        Ok(connection_manager)
    }
    pub async fn get_connection(&self) -> Result<ConnectionHandle, DatabaseError> {
        let connection = { self.connections.lock().unwrap().pop_front() };
        let connection_handle = match connection {
            Some(connection) => {
                let connection = match connection.health().await {
                    Ok(()) => connection,
                    Err(_) => Self::create_connection(&self.address).await?,
                };
                ConnectionHandle::new_pooled(Some(self.clone()), connection)
            }
            None => {
                ConnectionHandle::new_pooled(None, Self::create_connection(&self.address).await?)
            }
        };
        Ok(connection_handle)
    }

    async fn create_connection(address: &str) -> Result<Surreal<Any>, DatabaseError> {
        let connection = any::connect(address).await?;
        connection.use_ns("poietic").use_db("poietic").await?;
        Ok(connection)
    }

    pub(in crate::database::connection) fn release_connection(&self, connection: Surreal<Any>) {
        self.connections.lock().unwrap().push_back(connection);
    }
}
