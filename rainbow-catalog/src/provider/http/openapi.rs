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
use axum::{Json, Router};
use once_cell::sync::Lazy;
use serde_json::Value;

pub fn route_openapi() -> Router {
    Router::new().route("/api/v1/catalog/openapi.json", get(get_open_api))
}

static OPENAPI_JSON: Lazy<Value> = Lazy::new(|| {
    // let openapi_yaml = include_str!("../../../openapi/catalog.json");
    // let openapi = serde_json::from_str::<Value>(&openapi_yaml).unwrap();

    serde_json::json!({
        "testeo": "testeo"
    })
});

async fn get_open_api() -> impl IntoResponse {
    (StatusCode::OK, Json(OPENAPI_JSON.clone())).into_response()
}
