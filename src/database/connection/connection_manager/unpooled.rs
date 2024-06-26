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
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex, TryLockError},
    task::Poll,
};

use surrealdb::{
    engine::any::{self, Any},
    Surreal,
};

use crate::database::{
    connection::connection_handle::ConnectionHandle, database_error::DatabaseError,
};

#[derive(Debug, Clone)]
pub struct UnpooledConnectionManager {
    connection: Arc<Mutex<Option<Surreal<Any>>>>,
    address: Arc<str>,
}

impl UnpooledConnectionManager {
    pub async fn new(address: &str) -> Result<Self, DatabaseError> {
        let connection = any::connect(address).await?;
        connection.use_ns("poietic").use_db("poietic").await?;
        let connection_manager = Self {
            connection: Arc::new(Mutex::new(Some(connection))),
            address: Arc::from(address),
        };
        Ok(connection_manager)
    }
    pub async fn get_connection(&self) -> Result<ConnectionHandle, DatabaseError> {
        let mut connection = AcquireConnection::new(self.clone()).await;
        if connection.health().await.is_err() {
            connection = any::connect(self.address.as_ref()).await?;
        }
        Ok(ConnectionHandle::new_unpooled(self.clone(), connection))
    }

    pub(in crate::database::connection) fn release_connection(&self, connection: Surreal<Any>) {
        tokio::spawn(self.clone().release_connection_async(connection));
    }
    pub(in crate::database::connection) async fn release_connection_async(
        self,
        connection: Surreal<Any>,
    ) {
        ReleaseConnection::new(self.clone(), connection).await;
    }
}

struct AcquireConnection {
    connection_manager: UnpooledConnectionManager,
}

impl AcquireConnection {
    fn new(connection_manager: UnpooledConnectionManager) -> Self {
        Self { connection_manager }
    }
}

impl Future for AcquireConnection {
    type Output = Surreal<Any>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        match self.connection_manager.connection.try_lock() {
            Ok(mut lock) => match lock.take() {
                Some(connection) => Poll::Ready(connection),
                None => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            },
            Err(TryLockError::WouldBlock) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(TryLockError::Poisoned(error)) => panic!("{:?}", error),
        }
    }
}

struct ReleaseConnection {
    connection_manager: UnpooledConnectionManager,
    connection: Surreal<Any>,
}

impl ReleaseConnection {
    fn new(connection_manager: UnpooledConnectionManager, connection: Surreal<Any>) -> Self {
        Self {
            connection_manager,
            connection,
        }
    }
}

impl Future for ReleaseConnection {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        match self.connection_manager.connection.try_lock() {
            Ok(mut lock) => {
                let _ = lock.insert(self.connection.clone());
                Poll::Ready(())
            }
            Err(TryLockError::WouldBlock) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(TryLockError::Poisoned(error)) => panic!("{:?}", error),
        }
    }
}
