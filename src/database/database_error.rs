#[derive(Debug)]
pub enum DatabaseError {
    RecordNotFound,
    SurrealDbError(surrealdb::Error),
}

impl From<surrealdb::Error> for DatabaseError {
    fn from(value: surrealdb::Error) -> Self {
        DatabaseError::SurrealDbError(value)
    }
}
