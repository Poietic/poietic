use surrealdb::{
    engine::any::{self, Any},
    Surreal,
};

use crate::database::{connection::ConnectionHandle, database_error::DatabaseError};

#[derive(Debug, Clone)]
pub struct UnpooledConnectionManager {
    connection: Surreal<Any>,
}

impl UnpooledConnectionManager {
    pub async fn new(address: &str) -> Result<Self, DatabaseError> {
        Ok(Self {
            connection: any::connect(address).await?,
        })
    }
    pub fn get_connection(&self) -> Result<ConnectionHandle, DatabaseError> {
        Ok(ConnectionHandle::new(None, self.connection.clone()))
    }
}
