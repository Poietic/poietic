use std::time::SystemTime;

use chrono::DateTime;
use serde_json::Value as JsonValue;
use surrealdb::sql::{Id, Thing};

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

pub(super) async fn create_composition_on_connection(
    connection: ConnectionHandle,
    content: JsonValue,
) -> Result<Composition, DatabaseError> {
    let id = Thing::from(("composition", Id::ulid()));
    let current_time = DateTime::from(SystemTime::now());
    let composition = Composition {
        id,
        content,
        last_modified_at: current_time.into(),
    };
    connection
        .create::<Option<Composition>>(&composition.id)
        .content(&composition)
        .await?;
    Ok(composition)
}

pub async fn create_composition(content: JsonValue) -> Result<Composition, DatabaseError> {
    let connection = get_connection_manager().await.get_connection().await?;
    create_composition_on_connection(connection, content).await
}
