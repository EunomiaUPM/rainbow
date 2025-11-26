/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use once_cell::sync::Lazy;
use rainbow_common::openapi::swagger_ui_html;

pub fn route_openapi() -> Router {
    let openapi_spec = "/api/v1/transfer/openapi.json";
    Router::new().route(openapi_spec, get(get_open_api)).route(
        "/api/v1/transfer/openapi",
        get(|| swagger_ui_html(openapi_spec)),
    )
}

static OPENAPI_JSON: Lazy<&'static str> =
    Lazy::new(|| include_str!("./../../../../static/specs/openapi/transfer/transfer_consumer.json"));

async fn get_open_api() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "application/json")],
        OPENAPI_JSON.as_bytes(),
    )
        .into_response()
}
