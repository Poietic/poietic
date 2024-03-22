use std::{ops::Deref, sync::Arc};

use surrealdb::{engine::any::Any, Surreal};

use super::connection_manager::pooled::PooledConnectionManager;

pub struct ConnectionHandle {
    connection_manager: Option<Arc<PooledConnectionManager>>,
    connection: Surreal<Any>,
}

impl ConnectionHandle {
    pub fn new(connection_manager: Option<Arc<PooledConnectionManager>>, connection: Surreal<Any>) -> Self {
        Self { connection_manager, connection }
    }
}

impl Deref for ConnectionHandle {
    type Target = Surreal<Any>;
    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

impl Drop for ConnectionHandle {
    fn drop(&mut self) {
        match self.connection_manager.clone() {
            Some(connection_manager) => {
                connection_manager.release_connection(self.connection.clone())
            }
            None => {}
        }
    }
}
