use crate::database::{connection::get_connection, database_error::DatabaseError, model::Page};

pub async fn get_page_at_path(
    path: &str,
) -> Result<Page, DatabaseError> {
    let connection = get_connection().await?;
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
