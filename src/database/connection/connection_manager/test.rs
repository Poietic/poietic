use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::fs;

use super::{ConnectionManager, CONNECTION_MANAGER};

pub async fn setup_test_database() {
    CONNECTION_MANAGER
        .get_or_init(|| async {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let database_dir_name = format!("poietic_test_{}", timestamp);
            let database_dir_path = env::temp_dir().join(database_dir_name);
            let address = format!("file://{}", database_dir_path.to_str().unwrap());
            fs::create_dir(database_dir_path).await.unwrap();
            ConnectionManager::new(&address, None).await.unwrap()
        })
        .await;
}

#[tokio::test]
async fn test_database_setup() {
    setup_test_database().await;
}
