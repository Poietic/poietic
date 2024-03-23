use crate::database::{
    connection::{connection_handle::ConnectionHandle, connection_manager::get_connection_manager},
    database_error::DatabaseError,
    model::{Composition, Page},
};

pub(super) async fn get_composition_from_page_on_connection(
    connection: ConnectionHandle,
    page: &Page,
) -> Result<Composition, DatabaseError> {
    let page: Option<Composition> = connection.select(&page.composition).await?;
    match page {
        Some(page) => Ok(page),
        None => Err(DatabaseError::RecordNotFound),
    }
}

pub async fn get_composition_from_page(page: &Page) -> Result<Composition, DatabaseError> {
    let connection = get_connection_manager().await.get_connection().await?;
    get_composition_from_page_on_connection(connection, page).await
}
