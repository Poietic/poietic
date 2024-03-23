use crate::database::{
    connection::{connection_handle::ConnectionHandle, connection_manager::get_connection_manager}, database_error::DatabaseError, model::Page
};

pub(super) async fn get_page_at_path_on_connection(
    connection: ConnectionHandle,
    path: &str,
) -> Result<Page, DatabaseError> {
    let page: Option<Page> = connection
        .query("SELECT * FROM page WHERE path = $path")
        .bind(("path", path))
        .await?
        .take(0)?;
    match page {
        Some(page) => Ok(page),
        None => Err(DatabaseError::RecordNotFound),
    }
}

pub async fn get_page_at_path(path: &str) -> Result<Page, DatabaseError> {
    let connection = get_connection_manager().await.get_connection().await?;
    get_page_at_path_on_connection(connection, path).await
}
