use std::time::SystemTime;

use chrono::DateTime;
use serde_json::Value;
use surrealdb::sql::Thing;

use crate::database::{
    connection::{get_database_connection, setup_test_database},
    model::{Composition, Page},
};

use super::{composition::get_composition_from_page, page::get_page_at_path};

async fn create_example_composition(id: &str, content: &str) {
    let connection = get_database_connection().await.unwrap();
    let result = connection
        .create::<Vec<Composition>>("composition")
        .content(Composition {
            id: ("composition", id).into(),
            content: content.into(),
            last_modified_at: DateTime::from(SystemTime::now()).into(),
        })
        .await;
    match result {
        Err(surrealdb::Error::Db(surrealdb::error::Db::RecordExists { thing: _ })) => {}
        Err(other) => Err(other).unwrap(),
        _ => {}
    }
}

async fn create_example_page(id: &str, path: &str, composition_id: &str) {
    let connection = get_database_connection().await.unwrap();
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

pub async fn prepare_example_database() {
    setup_test_database().await;
    for id in 1..5 {
        let id = id.to_string();
        create_example_composition(&id, LOREM_IPSUM_PARAGRAH).await;
        create_example_page(&id, &format!("lorem/{}", id), &id).await;
    }
    create_example_page("index", "", "1").await;
}

#[tokio::test]
async fn example_database() {
    prepare_example_database().await;
}

#[tokio::test]
async fn page_fetching_by_path() {
    prepare_example_database().await;
    let index_page = get_page_at_path("").await.unwrap();
    assert_eq!("page:index", &index_page.id.to_string());
    assert_eq!(Thing::from(("composition", "1")), index_page.composition);
}

#[tokio::test]
async fn composition_fetching_by_page() {
    prepare_example_database().await;
    let index_page = Page {
        id: Thing::from(("page", "index")),
        composition: Thing::from(("composition", "1")),
        path: "".to_string(),
    };
    let index_composition = get_composition_from_page(&index_page).await.unwrap();
    assert_eq!(Thing::from(("composition", "1")), index_composition.id);
    assert_eq!(Value::from(LOREM_IPSUM_PARAGRAH), index_composition.content);
}
