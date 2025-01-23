/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::common::misc_router;
use crate::consumer::http::openapi::open_api_setup;
use crate::consumer::http::{hl_api, protocol_api};
use axum::http::Method;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub async fn create_consumer_router() -> Router {
    let cors = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any);

    let open_api = open_api_setup().unwrap();

    // create routing system
    let server = Router::new()
        .merge(misc_router::router())
        .merge(protocol_api::router())
        .merge(hl_api::router())
        .merge(open_api)
        .layer(TraceLayer::new_for_http());
    server
}
