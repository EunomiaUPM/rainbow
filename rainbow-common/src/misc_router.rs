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

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tracing::info;

pub fn router() -> Router {
    Router::new()
        .route("/version", get(get_version))
        .route("/.well-known/version", get(get_version))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionResponse {
    #[serde(rename = "@context")]
    context: String,
    protocol_versions: Vec<ProtocolVersionsResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProtocolVersionsResponse {
    version: String,
    path: String,
}

async fn get_version() -> impl IntoResponse {
    info!("GET /version");
    let response = VersionResponse {
        context: "https://w3id.org/dspace/2024/1/context.json".to_string(),
        protocol_versions: vec![ProtocolVersionsResponse {
            version: "1.0".to_string(),
            path: "/some/path/v1".to_string(),
        }],
    };
    (StatusCode::OK, Json(response))
}
