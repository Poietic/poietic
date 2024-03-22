use std::sync::Arc;

use tokio::sync::OnceCell;

use crate::{config::get_config, database::database_error::DatabaseError};

use self::{pooled::PooledConnectionManager, unpooled::UnpooledConnectionManager};

use super::ConnectionHandle;

pub mod pooled;
pub mod unpooled;
#[cfg(test)]
pub mod test;

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
    pub async fn get_connection(&self) -> Result<ConnectionHandle, DatabaseError> {
        match self {
            Self::Pooled(pooled) => pooled.clone().get_connection().await,
            Self::Unpooled(unpooled) => unpooled.get_connection(),
        }
    }
}

pub static CONNECTION_MANAGER: OnceCell<ConnectionManager> = OnceCell::const_new();

pub async fn create_connection_manager() -> ConnectionManager {
    let config = &get_config().database;
    ConnectionManager::new(&config.address, config.pool_size)
        .await
        .unwrap()
}

pub async fn get_connection_manager() -> &'static ConnectionManager {
    CONNECTION_MANAGER
        .get_or_init(create_connection_manager)
        .await
}
