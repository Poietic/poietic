use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Thing,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Composition {
    pub id: Thing,
    pub content: JsonValue,
    pub last_modified_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub id: Thing,
    pub path: String,
    pub composition: Thing,
}
