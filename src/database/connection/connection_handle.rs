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
