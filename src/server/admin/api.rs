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
    post,
    web::{route, scope, Path},
    HttpResponse, Responder, Route, Scope,
};

pub mod create_composition;
pub mod create_page;

#[post("/{namespace}/{api_function}")]
pub async fn api_route_service(path: Path<(String, String)>) -> impl Responder {
    let (namespace, api_function) = path.into_inner();
    todo!("Handle API routes");
    ""
}

fn create_404_handler() -> Route {
    route().to(HttpResponse::NotFound)
}

pub fn create_api_scope() -> Scope {
    scope("/api")
        .service(create_composition::create_composition)
        .service(create_page::create_page)
        .service(api_route_service)
        .default_service(create_404_handler())
}
