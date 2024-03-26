// Copyright 2024 Jakub Duda, Lech Mazur
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

use actix_web::{
    web::{Data, ServiceConfig},
    App, HttpServer,
};

use crate::{
    config::http_server_config::HttpServerConfig,
    database::connection::connection_manager::ConnectionManager,
};

pub mod admin;
pub mod public;

pub async fn start_http_server(
    connection_manager: ConnectionManager,
    server_config: &HttpServerConfig,
    configure_actix: fn(&mut ServiceConfig),
) {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(connection_manager.clone()))
            .configure(configure_actix)
    })
    .bind((server_config.address, server_config.port))
    .unwrap()
    .run()
    .await
    .unwrap();
}
