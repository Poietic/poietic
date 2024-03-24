use poietic::{
    config::get_config,
    database::connection::connection_manager::create_connection_manager,
    server::{admin::configure_admin_app, public::configure_public_app, start_http_server},
};
use tokio::task::{JoinError, JoinSet};

#[actix_web::main]
async fn main() -> Result<(), JoinError> {
    let config = get_config();
    let connection_manager = create_connection_manager().await;
    let mut http_server_set = JoinSet::new();
    http_server_set.spawn(start_http_server(
        connection_manager.clone(),
        &config.admin,
        configure_admin_app,
    ));
    http_server_set.spawn(start_http_server(
        connection_manager,
        &config.public,
        configure_public_app,
    ));
    while let Some(res) = http_server_set.join_next().await {
        res?;
    }
    Ok(())
}
