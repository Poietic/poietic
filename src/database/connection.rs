use std::{ops::Deref, sync::Arc};

use surrealdb::{
    engine::any::{self, Any},
    Surreal,
};
use tokio::sync::{Mutex, OnceCell};

use crate::config::get_config;

use super::database_error::DatabaseError;

pub struct ConnectionManager {
    connections: Mutex<Vec<Surreal<Any>>>,
    address: String,
}

impl ConnectionManager {
    async fn new(address: &str, pool_size: usize) -> Result<Arc<Self>, DatabaseError> {
        let mut connections = Vec::<Surreal<Any>>::with_capacity(pool_size);
        for _ in 0..pool_size {
            connections.push(Self::create_connection(address).await?)
        }
        let connection_manager = Self {
            connections: Mutex::new(connections),
            address: address.to_string(),
        };
        Ok(Arc::new(connection_manager))
    }
    async fn create_connection(address: &str) -> Result<Surreal<Any>, DatabaseError> {
        let connection = any::connect(address).await?;
        connection.use_ns("poietic").use_db("poietic").await?;
        Ok(connection)
    }
    async fn get_connection(self: Arc<Self>) -> Result<ConnectionHandle, DatabaseError> {
        let connection_handle = match self.connections.lock().await.pop() {
            Some(connection) => ConnectionHandle {
                connection,
                connection_manager: Some(self.clone()),
            },
            None => ConnectionHandle {
                connection: Self::create_connection(&self.address).await?,
                connection_manager: None,
            },
        };
        Ok(connection_handle)
    }
    fn release_connection(self: Arc<Self>, connection: Surreal<Any>) {
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

pub struct ConnectionHandle {
    connection_manager: Option<Arc<ConnectionManager>>,
    connection: Surreal<Any>,
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

static CONNECTION_MANAGER: OnceCell<Arc<ConnectionManager>> = OnceCell::const_new();

async fn create_connection_manager() -> Arc<ConnectionManager> {
    ConnectionManager::new(&get_config().database.address, 4)
        .await
        .unwrap()
}

async fn get_connection_manager() -> Arc<ConnectionManager> {
    CONNECTION_MANAGER
        .get_or_init(create_connection_manager)
        .await
        .clone()
}

pub async fn get_connection() -> Result<ConnectionHandle, DatabaseError> {
    get_connection_manager().await.get_connection().await
}
