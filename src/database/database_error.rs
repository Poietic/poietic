use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum DatabaseError {
    RecordNotFound,
    SurrealDbError(Arc<surrealdb::Error>),
}

impl From<surrealdb::Error> for DatabaseError {
    fn from(value: surrealdb::Error) -> Self {
        DatabaseError::SurrealDbError(Arc::new(value))
    }
}
