use crate::database::connection::{get_database_connection, setup_test_database};

#[tokio::test]
async fn database_connection() {
    setup_test_database().await;
    get_database_connection().await.unwrap();
}
