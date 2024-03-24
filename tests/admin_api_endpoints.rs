use std::str::FromStr;

use actix_web::{
    test::{self, TestRequest},
    web::Data,
    App,
};
use poietic::{
    database::{connection::connection_manager::ConnectionManager, model::{Composition, Page}},
    server::admin::{
        api::{
            CreateCompositionRequestBody, CreateCompositionResponseBody, CreatePageRequestBody,
            CreatePageResponseBody,
        },
        configure_admin_app,
    },
};
use serde_json::Value;
use surrealdb::sql::Thing;

async fn create_test_database() -> ConnectionManager {
    ConnectionManager::new("mem://", None).await.unwrap()
}

#[actix_web::test]
async fn composition_creation() {
    let connection_manager = create_test_database().await;
    let admin_app = test::init_service(
        App::new()
            .app_data(Data::new(connection_manager.clone()))
            .configure(configure_admin_app),
    )
    .await;
    let composition_content = serde_json::from_str::<Value>(
        r#"{
            "component": "poitic:Paragraph",
            "params": {
                "content": "Lorem ipsum, dolor sit amet."
            }
        }"#,
    )
    .unwrap();
    let request = TestRequest::post()
        .uri("/api/poietic/create-composition")
        .set_json(CreateCompositionRequestBody {
            content: composition_content.clone(),
        })
        .to_request();
    let response: CreateCompositionResponseBody =
        test::call_and_read_body_json(&admin_app, request).await;
    let composition_id = Thing::from_str(&response.id).unwrap();
    let connection = connection_manager.get_connection().await.unwrap();
    let composition: Composition = connection.select(composition_id).await.unwrap().unwrap();
    assert_eq!(composition_content, composition.content);
}

#[actix_web::test]
async fn page_creation() {
    let connection_manager = create_test_database().await;
    let admin_app = test::init_service(
        App::new()
            .app_data(Data::new(connection_manager.clone()))
            .configure(configure_admin_app),
    )
    .await;
    let composition_content = serde_json::from_str::<Value>(
        r#"{
            "component": "poitic:Paragraph",
            "params": {
                "content": "Lorem ipsum, dolor sit amet."
            }
        }"#,
    )
    .unwrap();
    let composition_request = TestRequest::post()
        .uri("/api/poietic/create-composition")
        .set_json(CreateCompositionRequestBody {
            content: composition_content.clone(),
        })
        .to_request();
    let composition_response: CreateCompositionResponseBody =
        test::call_and_read_body_json(&admin_app, composition_request).await;
    let page_path = "/";
    let page_request = TestRequest::post()
        .uri("/api/poietic/create-page")
        .set_json(CreatePageRequestBody {
            path: page_path.to_string(),
            composition_id: composition_response.id.clone(),
        })
        .to_request();
    let page_response: CreatePageResponseBody =
        test::call_and_read_body_json(&admin_app, page_request).await;
    let page_id = Thing::from_str(&page_response.id).unwrap();
    let connection = connection_manager.get_connection().await.unwrap();
    let page: Page = connection.select(page_id).await.unwrap().unwrap();
    assert_eq!(composition_response.id, page.composition.to_string());
    assert_eq!(page_path, page.path);
}
