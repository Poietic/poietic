use actix_web::{
    web::{route, ServiceConfig},
    HttpResponse, Route,
};

pub mod api;
use self::api::create_api_scope;

fn create_404_handler() -> Route {
    route().to(HttpResponse::NotFound)
}

pub fn configure_admin_app(config: &mut ServiceConfig) {
    config
        .service(create_api_scope())
        .default_service(create_404_handler());
}
