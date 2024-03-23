use std::sync::Arc;

use surrealdb::{
    engine::any::{self, Any},
    Surreal,
};
use tokio::sync::Mutex;

use crate::database::{
    connection::connection_handle::ConnectionHandle, database_error::DatabaseError,
};

#[derive(Debug, Clone)]
pub struct PooledConnectionManager {
    connections: Arc<Mutex<Vec<Surreal<Any>>>>,
    address: Arc<str>,
}

impl PooledConnectionManager {
    pub async fn new(address: &str, pool_size: usize) -> Result<Self, DatabaseError> {
        let mut connections = Vec::<Surreal<Any>>::with_capacity(pool_size);
        for _ in 0..pool_size {
            connections.push(Self::create_connection(address).await?)
        }
        let connection_manager = PooledConnectionManager {
            connections: Arc::new(Mutex::new(connections)),
            address: Arc::from(address),
        };
        Ok(connection_manager)
    }
    pub async fn get_connection(&self) -> Result<ConnectionHandle, DatabaseError> {
        let connection_handle = match self.connections.lock().await.pop() {
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
        tokio::spawn(self.clone().release_connection_async(connection));
    }
    pub(in crate::database::connection) async fn release_connection_async(
        self,
        connection: Surreal<Any>,
    ) {
        self.connections.lock().await.push(connection);
    }
}
