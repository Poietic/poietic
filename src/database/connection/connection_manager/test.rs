// Copyright 2024 Lech Mazur
//
// This file is part of Poietic.
//
// Poietic is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License, version 2,
// as published by the Free Software Foundation.
//
// Poietic is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Poietic. If not, see <https://www.gnu.org/licenses/>.

use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::fs;

use super::ConnectionManager;

pub async fn create_test_rocksdb_database() -> ConnectionManager {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let database_dir_name = format!("poietic_test_{}", timestamp);
    let database_dir_path = env::temp_dir().join(database_dir_name);
    let address = format!("file://{}", database_dir_path.to_str().unwrap());
    fs::create_dir(database_dir_path).await.unwrap();
    ConnectionManager::new(&address, None).await.unwrap()
}

pub async fn create_test_mem_database() -> ConnectionManager {
    ConnectionManager::new("mem://", None).await.unwrap()
}

#[tokio::test]
async fn test_database_setup() {
    create_test_rocksdb_database().await;
    create_test_mem_database().await;
}
