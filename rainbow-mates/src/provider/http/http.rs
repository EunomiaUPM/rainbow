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

use axum::extract::{Path, Query, State};
use axum::http::{Method, Uri};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use rainbow_common::mates::Mates;
use rainbow_db::mates::repo::{MateRepoFactory, MateRepoTrait};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub struct RainbowMateProviderRouter<T>
where
    T: MateRepoTrait + Send + Sync + Clone + 'static,
{
    pub mate_repo: Arc<T>,
}

impl<T> RainbowMateProviderRouter<T>
where
    T: MateRepoTrait + Send + Sync + Clone + 'static,
{
    pub fn new(mate_repo: Arc<T>) -> Self {
        Self { mate_repo }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/mates", get(Self::get_mates))
            .route("/mates", post(Self::new_mate))
            .route("/mates/:id", get(Self::get_singular_mate))
            .route("/mates/:id", put(Self::edit_mate))
            .route("/mates/:id", delete(Self::delete_mate))
            .with_state(self.mate_repo)
            .fallback(Self::fallback)
    }

    async fn get_mates(State(mate_repo): State<Arc<T>>) -> impl IntoResponse {
        info!("GET /mates");

        let mates = match mate_repo.get_all_mates(None, None).await {
            Ok(mates) => mates,
            Err(e) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        // TODO HABLAR CON CARLOS
        StatusCode::OK.into_response()
    }

    async fn new_mate(State(mate_repo): State<Arc<T>>, Json(payload): Json<Mates>) -> impl IntoResponse {
        info!("GET /mates");

        let mates = match mate_repo.create_mate(payload).await {
            Ok(mates) => mates,
            Err(e) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        // TODO HABLAR CON CARLOS
        StatusCode::OK.into_response()
    }

    async fn get_singular_mate(State(mate_repo): State<Arc<T>>, Path(id): Path<String>) -> impl IntoResponse {
        let log = format!("GET /mates/{}", id);
        info!(log);

        let mate = match mate_repo.get_mate_by_id(id).await {
            Ok(mate) => mate,
            Err(e) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        // TODO HABLAR CON CARLOS
        StatusCode::OK.into_response()
    }

    async fn edit_mate(
        State(mate_repo): State<Arc<T>>,
        Path(id): Path<String>,
        Json(payload): Json<Mates>,
    ) -> impl IntoResponse {
        let log = format!("POST /mates/{}", id);
        info!(log);
        // TODO COMPLETAR PA EDITAR, COMO ME LLEGAN LOS DATOS A MI??

        let mate = match mate_repo.update_mate(payload).await {
            Ok(mate) => mate,
            Err(e) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        // TODO HABLAR CON CARLOS
        StatusCode::OK.into_response()
    }

    async fn delete_mate(State(mate_repo): State<Arc<T>>, Path(id): Path<String>) -> impl IntoResponse {
        let log = format!("DELETE /mates/{}", id);
        info!(log);

        match mate_repo.delete_mate(id).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
        let log = format!("{} {}", method, uri);
        info!("{}", log);
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
}
