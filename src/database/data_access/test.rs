// Copyright 2024 Lech Mazur, Adam Wasiak
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

use std::time::SystemTime;

use chrono::DateTime;
use serde_json::Value;
use surrealdb::sql::Thing;

use crate::database::{
    connection::connection_manager::{test::create_test_mem_database, ConnectionManager},
    data_access::composition::CompositionRepository,
    data_access::page::PageRepository,
    model::{Composition, Page},
};

async fn create_example_composition(
    connection_manager: &ConnectionManager,
    id: &str,
    content: &str,
) {
    let connection = connection_manager.get_connection().await.unwrap();
    let result = connection
        .create::<Vec<Composition>>("composition")
        .content(Composition {
            id: ("composition", id).into(),
            content: serde_json::from_str(content).unwrap(),
            last_modified_at: DateTime::from(SystemTime::now()).into(),
        })
        .await;
    match result {
        Err(surrealdb::Error::Db(surrealdb::error::Db::RecordExists { thing: _ })) => {}
        Err(other) => Err(other).unwrap(),
        _ => {}
    }
}

async fn create_example_page(
    connection_manager: &ConnectionManager,
    id: &str,
    path: &str,
    composition_id: &str,
) {
    let connection = connection_manager.get_connection().await.unwrap();
    let result = connection
        .create::<Vec<Page>>("page")
        .content(Page {
            id: ("page", id).into(),
            path: path.into(),
            composition: ("composition", composition_id).into(),
        })
        .await;
    match result {
        Err(surrealdb::Error::Db(surrealdb::error::Db::RecordExists { thing: _ })) => {}
        Err(other) => Err(other).unwrap(),
        _ => {}
    }
}

const LOREM_IPSUM_PARAGRAH: &str = r#"{
    "component": "poieitc:Paragraph",
    "params": {
        "content": "Lorem ipsum"
    }
}"#;

pub async fn prepare_example_database() -> ConnectionManager {
    let connection_manager = create_test_mem_database().await;
    for id in 1..5 {
        let id = id.to_string();
        create_example_composition(&connection_manager, &id, LOREM_IPSUM_PARAGRAH).await;
        create_example_page(&connection_manager, &id, &format!("lorem/{}", id), &id).await;
    }
    create_example_page(&connection_manager, "index", "", "1").await;
    connection_manager
}

#[tokio::test]
async fn example_database() {
    prepare_example_database().await;
}

#[tokio::test]
async fn page_fetching_by_path() {
    let connection_manager = prepare_example_database().await;
    let index_page = connection_manager.get_page_at_path("").await.unwrap();
    assert_eq!("page:index", &index_page.id.to_string());
    assert_eq!(Thing::from(("composition", "1")), index_page.composition);
}

#[tokio::test]
async fn composition_fetching_by_page() {
    let connection_manager = prepare_example_database().await;
    let index_page = Page {
        id: Thing::from(("page", "index")),
        composition: Thing::from(("composition", "1")),
        path: "".to_string(),
    };
    let index_composition = connection_manager
        .get_composition_from_page(&index_page)
        .await
        .unwrap();
    assert_eq!(Thing::from(("composition", "1")), index_composition.id);
    assert_eq!(
        serde_json::from_str::<Value>(LOREM_IPSUM_PARAGRAH).unwrap(),
        index_composition.content
    );
}
