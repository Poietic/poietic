mod connection_handle;
mod connection_manager;
#[cfg(test)]
mod test;

pub use self::connection_handle::ConnectionHandle;
use self::connection_manager::get_connection_manager;

use super::database_error::DatabaseError;

#[cfg(test)]
pub use self::connection_manager::test::setup_test_database;

pub async fn get_database_connection() -> Result<ConnectionHandle, DatabaseError> {
    get_connection_manager().await.get_connection().await
}
