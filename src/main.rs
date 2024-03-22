use server::{admin::start_admin_http_server, public::start_public_http_server};
use tokio::task::{JoinError, JoinSet};

mod component;
mod config;
mod database;
mod html;
mod server;

#[actix_web::main]
async fn main() -> Result<(), JoinError> {
    let mut http_server_set = JoinSet::new();
    http_server_set.spawn(start_public_http_server());
    http_server_set.spawn(start_admin_http_server());
    while let Some(res) = http_server_set.join_next().await {
        res?;
    }
    Ok(())
}
