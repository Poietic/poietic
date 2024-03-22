use std::{ops::Deref, sync::Arc};

use surrealdb::{
    engine::any::{self, Any},
    Surreal,
};
use tokio::sync::{Mutex, OnceCell};

use crate::config::get_config;

use super::database_error::DatabaseError;

#[derive(Debug)]
pub struct PooledConnectionManager {
    connections: Mutex<Vec<Surreal<Any>>>,
    address: String,
}

impl PooledConnectionManager {
    async fn new(address: &str, pool_size: usize) -> Result<Arc<Self>, DatabaseError> {
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
    async fn create_connection(address: &str) -> Result<Surreal<Any>, DatabaseError> {
        let connection = any::connect(address).await?;
        connection.use_ns("poietic").use_db("poietic").await?;
        Ok(connection)
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

#[derive(Debug, Clone)]
pub struct UnpooledConnectionManager {
    connection: Surreal<Any>,
}

impl UnpooledConnectionManager {
    fn get_connection(&self) -> Result<ConnectionHandle, DatabaseError> {
        Ok(ConnectionHandle {
            connection_manager: None,
            connection: self.connection.clone(),
        })
    }

    async fn new(address: &str) -> Result<Self, DatabaseError> {
        Ok(Self {
            connection: any::connect(address).await?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionManager {
    Pooled(Arc<PooledConnectionManager>),
    Unpooled(UnpooledConnectionManager),
}

impl ConnectionManager {
    pub async fn new(address: &str, pool_size: Option<usize>) -> Result<Self, DatabaseError> {
        let connection_manager = match pool_size {
            Some(pool_size) => {
                Self::Pooled(PooledConnectionManager::new(address, pool_size).await?)
            }
            None => Self::Unpooled(UnpooledConnectionManager::new(address).await?),
        };
        Ok(connection_manager)
    }
    async fn get_connection(&self) -> Result<ConnectionHandle, DatabaseError> {
        match self {
            Self::Pooled(pooled) => pooled.clone().get_connection().await,
            Self::Unpooled(unpooled) => unpooled.get_connection(),
        }
    }
}

pub struct ConnectionHandle {
    connection_manager: Option<Arc<PooledConnectionManager>>,
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

pub(super) static CONNECTION_MANAGER: OnceCell<ConnectionManager> = OnceCell::const_new();

async fn create_connection_manager() -> ConnectionManager {
    let config = &get_config().database;
    ConnectionManager::new(&config.address, config.pool_size)
        .await
        .unwrap()
}

async fn get_connection_manager() -> &'static ConnectionManager {
    CONNECTION_MANAGER
        .get_or_init(create_connection_manager)
        .await
}

pub async fn get_connection() -> Result<ConnectionHandle, DatabaseError> {
    get_connection_manager().await.get_connection().await
}
