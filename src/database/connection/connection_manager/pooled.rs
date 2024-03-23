use std::sync::Arc;

use surrealdb::{
    engine::any::{self, Any},
    Surreal,
};
use tokio::sync::Mutex;

use crate::database::{connection::connection_handle::ConnectionHandle, database_error::DatabaseError};

#[derive(Debug)]
pub struct PooledConnectionManager {
    connections: Mutex<Vec<Surreal<Any>>>,
    address: String,
}

impl PooledConnectionManager {
    pub async fn new(address: &str, pool_size: usize) -> Result<Arc<Self>, DatabaseError> {
        let mut connections = Vec::<Surreal<Any>>::with_capacity(pool_size);
        for _ in 0..pool_size {
            connections.push(Self::create_connection(address).await?)
        }
        let connection_manager = PooledConnectionManager {
            connections: Mutex::new(connections),
            address: address.to_string(),
        };
        Ok(Arc::new(connection_manager))
    }
    pub async fn get_connection(self: Arc<Self>) -> Result<ConnectionHandle, DatabaseError> {
        let connection_handle = match self.connections.lock().await.pop() {
            Some(connection) => ConnectionHandle::new(Some(self.clone()), connection),
            None => ConnectionHandle::new(None, Self::create_connection(&self.address).await?),
        };
        Ok(connection_handle)
    }
    async fn create_connection(address: &str) -> Result<Surreal<Any>, DatabaseError> {
        let connection = any::connect(address).await?;
        connection.use_ns("poietic").use_db("poietic").await?;
        Ok(connection)
    }
    pub(in crate::database::connection) fn release_connection(self: Arc<Self>, connection: Surreal<Any>) {
        let self_clone = self.clone();
        tokio::spawn(async move {
            let connection = match connection.health().await {
                Ok(()) => connection,
                Err(_) => Self::create_connection(&self_clone.address).await?,
            };
            self_clone.connections.lock().await.push(connection);
            Ok::<(), DatabaseError>(())
        });
    }
}
