use std::{ops::Deref, sync::Arc};

use surrealdb::{
    engine::any::{self, Any},
    Surreal,
};
use tokio::sync::{Mutex, OnceCell};

use crate::config::get_config;

pub struct ConnectionManager {
    connections: Mutex<Vec<Surreal<Any>>>,
    address: String,
}

impl ConnectionManager {
    async fn new(address: &str, pool_size: usize) -> Arc<Self> {
        let mut connections = Vec::<Surreal<Any>>::with_capacity(pool_size);
        for _ in 0..pool_size {
            let connection = any::connect(address).await.unwrap();
            connections.push(connection)
        }
        Arc::new(Self {
            connections: Mutex::new(connections),
            address: address.to_string(),
        })
    }
    async fn get_connection(self: Arc<Self>) -> ConnectionHandle {
        match self.connections.lock().await.pop() {
            Some(connection) => ConnectionHandle {
                connection,
                connection_manager: Some(self.clone()),
            },
            None => ConnectionHandle {
                connection: any::connect(&self.address).await.unwrap(),
                connection_manager: None,
            },
        }
    }
    fn put_connection(self: Arc<Self>, connection: Surreal<Any>) {
        let self_clone = self.clone();
        tokio::spawn(async move { self_clone.connections.lock().await.push(connection) });
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
            Some(connection_manager) => connection_manager.put_connection(self.connection.clone()),
            None => {}
        }
    }
}

static CONNECTION_MANAGER: OnceCell<Arc<ConnectionManager>> = OnceCell::const_new();

async fn create_connection_manager() -> Arc<ConnectionManager> {
    ConnectionManager::new(&get_config().database.address, 4).await
}

async fn get_connection_manager() -> Arc<ConnectionManager> {
    CONNECTION_MANAGER
        .get_or_init(create_connection_manager)
        .await
        .clone()
}

pub async fn get_connection() -> ConnectionHandle {
    get_connection_manager().await.get_connection().await
}
