use std::sync::Arc;

use surrealdb::{
    engine::any::{self, Any},
    Surreal,
};

use crate::database::{connection::ConnectionHandle, database_error::DatabaseError};

#[derive(Debug, Clone)]
pub struct UnpooledConnectionManager {
    connection: Arc<Surreal<Any>>,
}

impl UnpooledConnectionManager {
    pub async fn new(address: &str) -> Result<Self, DatabaseError> {
        let connection = any::connect(address).await?;
        connection.use_ns("poietic").use_db("poietic").await?;
        Ok(Self { connection: Arc::new(connection) })
    }
    pub fn get_connection(&self) -> Result<ConnectionHandle, DatabaseError> {
        Ok(ConnectionHandle::new(None, self.connection.clone()))
    }
}
