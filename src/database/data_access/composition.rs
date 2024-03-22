use crate::database::{
    connection::get_connection,
    database_error::DatabaseError,
    model::{Composition, Page},
};

pub async fn get_composition_from_page(page: &Page) -> Result<Composition, DatabaseError> {
    let connection = get_connection().await?;
    let page: Option<Composition> = connection.select(&page.composition).await?;
    match page {
        Some(page) => Ok(page),
        None => Err(DatabaseError::RecordNotFound),
    }
}
