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

use std::time::SystemTime;

use actix_web::{
    test::{self, TestRequest},
    web::Data,
    App,
};
use chrono::DateTime;
use poietic::{
    database::{
        connection::connection_manager::ConnectionManager,
        model::{Composition, Page},
    },
    server::public::configure_public_app,
};

async fn create_test_database() -> ConnectionManager {
    ConnectionManager::new("mem://", None).await.unwrap()
}

const LOREM_PARAGRAPH: &str = r#"{
    "component": "poietic:Paragraph",
    "params": {
        "content": "Lorem ipsum, dolor sit amet."
    }
}"#;

const LOREM_PARAGRAPH_RENDERED: &str = "<p>Lorem ipsum, dolor sit amet.</p>";

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

#[actix_web::test]
async fn page_route_handling() {
    let connection_manager = create_test_database().await;
    create_example_composition(&connection_manager, "index", LOREM_PARAGRAPH).await;
    create_example_page(&connection_manager, "index", "", "index").await;

    let public_app = test::init_service(
        App::new()
            .app_data(Data::new(connection_manager.clone()))
            .configure(configure_public_app),
    )
    .await;
    let request = TestRequest::get().uri("/").to_request();
    let response = String::from_utf8(
        test::call_and_read_body(&public_app, request)
            .await
            .to_vec(),
    )
    .unwrap();
    assert_eq!(LOREM_PARAGRAPH_RENDERED, response);
}
