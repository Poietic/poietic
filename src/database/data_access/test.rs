use std::time::SystemTime;

use chrono::DateTime;

use crate::database::{
    connection::{get_database_connection, setup_test_database},
    model::{Composition, Page},
};

async fn create_example_composition(id: &str, content: &str) {
    let connection = get_database_connection().await.unwrap();
    connection
        .create::<Vec<Composition>>("composition")
        .content(Composition {
            id: ("composition", id).into(),
            content: content.into(),
            last_modified_at: DateTime::from(SystemTime::now()).into(),
        })
        .await
        .unwrap();
}

async fn create_example_page(id: &str, path: &str, composition_id: &str) {
    let connection = get_database_connection().await.unwrap();
    connection
        .create::<Vec<Page>>("page")
        .content(Page {
            id: ("page", id).into(),
            path: path.into(),
            composition: ("composition", composition_id).into(),
        })
        .await
        .unwrap();
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
