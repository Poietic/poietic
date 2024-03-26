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
