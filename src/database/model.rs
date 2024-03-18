use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  id: Thing,
  email: String,
  password_hash: String,
  password_salt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Composition {
  id: Thing,
  content: JsonValue,
  last_modified_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
  id: Thing,
  path: String,
  composition: Thing,
}
