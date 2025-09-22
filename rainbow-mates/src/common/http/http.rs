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

use crate::common::core::mates_types::BootstrapMateRequest;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, State};
use axum::http::{Method, Uri};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use rainbow_common::auth::business::RainbowBusinessLoginRequest;
use rainbow_common::mates::mates::VerifyTokenRequest;
use rainbow_common::mates::{BusMates, Mates};
use rainbow_db::mates::repo::{BusmateRepoTrait, MateRepoTrait};
use reqwest::{Client, StatusCode};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, info};

pub struct RainbowMatesRouter<T>
where
    T: MateRepoTrait + BusmateRepoTrait + Send + Sync + Clone + 'static,
{
    pub mate_repo: Arc<T>,
}

impl<T> RainbowMatesRouter<T>
where
    T: MateRepoTrait + BusmateRepoTrait + Send + Sync + Clone + 'static,
{
    pub fn new(mate_repo: Arc<T>) -> Self {
        Self { mate_repo }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/api/v1/mates", get(Self::get_mates))
            .route("/api/v1/mates", post(Self::new_mate))
            .route(
                "/api/v1/mates/verify",
                post(Self::verify_singular_mate_by_token),
            )
            .route("/api/v1/mates/me", get(Self::get_me_mate))
            .route("/api/v1/mates/me", post(Self::bootstrap_mate))
            .route("/api/v1/mates/:id", get(Self::get_singular_mate))
            .route("/api/v1/mates/:id", put(Self::edit_mate))
            .route("/api/v1/mates/:id", delete(Self::delete_mate))
            .route(
                "/api/v1/mates/bypass/:id_participant",
                get(Self::bypass_mates),
            )
            .route(
                "/api/v1/mates/bypass/:id_participant/:id",
                get(Self::bypass_mates_by_id),
            )
            .route("/api/v1/busmates", get(Self::get_busmates))
            .route("/api/v1/busmates", post(Self::new_busmate))
            // .route("/api/v1/busmates/me", get(Self::get_me_busmate))
            // .route("/api/v1/busmates/me", post(Self::bootstrap_busmate))
            .route("/api/v1/busmates/:id", get(Self::get_singular_busmate))
            .route("/api/v1/busmates/:id", put(Self::edit_busmate))
            .route("/api/v1/busmates/:id", delete(Self::delete_busmate))
            .route("/api/v1/busmates/token", post(Self::give_token))
            .with_state(self.mate_repo)
            .fallback(Self::fallback)
    }

    async fn get_mates(State(mate_repo): State<Arc<T>>) -> impl IntoResponse {
        info!("GET /api/v1/mates");

        match mate_repo.get_all_mates(None, None).await {
            Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    async fn new_mate(State(mate_repo): State<Arc<T>>, input: Result<Json<Mates>, JsonRejection>) -> impl IntoResponse {
        info!("POST /api/v1/mates");

        let input = match input {
            Ok(input) => input.0,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        };
        match mate_repo.create_mate(input).await {
            Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    async fn bootstrap_mate(
        State(mate_repo): State<Arc<T>>,
        input: Result<Json<BootstrapMateRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/mates/me");

        let input = match input {
            Ok(input) => input.0,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        };
        match mate_repo.create_mate(input.into()).await {
            Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    async fn get_singular_mate(State(mate_repo): State<Arc<T>>, Path(id): Path<String>) -> impl IntoResponse {
        info!("GET /mates/{}", id);

        match mate_repo.get_mate_by_id(id).await {
            Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
    async fn verify_singular_mate_by_token(
        State(mate_repo): State<Arc<T>>,
        input: Result<Json<VerifyTokenRequest>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /mates/verify");
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        };
        match mate_repo.get_mate_by_token(input).await {
            Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    async fn get_me_mate(State(mate_repo): State<Arc<T>>) -> impl IntoResponse {
        info!("GET /mates/me");

        match mate_repo.get_mate_me().await {
            Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    async fn edit_mate(
        State(mate_repo): State<Arc<T>>,
        Path(id): Path<String>,
        input: Result<Json<Mates>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("PUT /mates/{}", id);

        let input = match input {
            Ok(input) => input.0,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        };
        match mate_repo.update_mate(input).await {
            Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    async fn delete_mate(State(mate_repo): State<Arc<T>>, Path(id): Path<String>) -> impl IntoResponse {
        info!("DELETE /mates/{}", id);

        match mate_repo.delete_mate(id).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    async fn get_busmates(State(mate_repo): State<Arc<T>>) -> impl IntoResponse {
        info!("GET /api/v1/busmates");

        match mate_repo.get_all_busmates(None, None).await {
            Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    async fn new_busmate(
        State(mate_repo): State<Arc<T>>,
        input: Result<Json<BusMates>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("POST /api/v1/busmates");

        let input = match input {
            Ok(input) => input.0,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        };
        match mate_repo.create_busmate(input).await {
            Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    // async fn bootstrap_busmate(State(mate_repo): State<Arc<T>>, input: Result<Json<BootstrapMateRequest>, JsonRejection>) -> impl IntoResponse {
    //     info!("POST /api/v1/busmates/me");
    //
    //     let input = match input {
    //         Ok(input) => input.0,
    //         Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    //     };
    //     match mate_repo.create_busmate(input.into(), true).await {
    //         Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
    //         Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    //     }
    // }

    async fn get_singular_busmate(State(mate_repo): State<Arc<T>>, Path(id): Path<String>) -> impl IntoResponse {
        info!("GET /busmates/{}", id);

        match mate_repo.get_busmate_by_id(id).await {
            Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    // async fn get_me_busmate(State(mate_repo): State<Arc<T>>) -> impl IntoResponse {
    //     info!("GET /busmates/me");
    //
    //     match mate_repo.get_busmate_me().await {
    //         Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
    //         Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    //     }
    // }

    async fn edit_busmate(
        State(mate_repo): State<Arc<T>>,
        Path(id): Path<String>,
        input: Result<Json<BusMates>, JsonRejection>,
    ) -> impl IntoResponse {
        info!("PUT /busmates/{}", id);

        let input = match input {
            Ok(input) => input.0,
            Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        };
        match mate_repo.update_busmate(input).await {
            Ok(mates) => (StatusCode::OK, Json(mates)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    async fn delete_busmate(State(mate_repo): State<Arc<T>>, Path(id): Path<String>) -> impl IntoResponse {
        info!("DELETE /busmates/{}", id);

        match mate_repo.delete_busmate(id).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }

    async fn bypass_mates(
        State(mate_repo): State<Arc<T>>,
        Path(bypassing_participant_id): Path<String>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/mates/bypass/{}", bypassing_participant_id);
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");

        let base_url = match mate_repo.get_mate_by_id(bypassing_participant_id).await {
            Ok(mate) => mate.base_url.unwrap_or_default(),
            Err(e) => return (StatusCode::NOT_FOUND, e.to_string()).into_response(),
        };
        let url = format!("{}/api/v1/mates", base_url);
        match client.get(url).send().await {
            Ok(res) => {
                if res.status().is_success() == false {
                    return (res.status(), res.text().await.unwrap()).into_response();
                }
                let mates = res.json::<Vec<Mates>>().await.unwrap();
                (StatusCode::OK, Json(mates)).into_response()
            }
            Err(e) => (StatusCode::BAD_GATEWAY, e.to_string()).into_response(),
        }
    }

    async fn bypass_mates_by_id(
        State(mate_repo): State<Arc<T>>,
        Path((bypassing_participant_id, participant_id)): Path<(String, String)>,
    ) -> impl IntoResponse {
        info!(
            "GET /api/v1/mates/bypass/{}/{}",
            bypassing_participant_id, participant_id
        );
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");

        let base_url = match mate_repo.get_mate_by_id(bypassing_participant_id).await {
            Ok(mate) => mate.base_url.unwrap_or_default(),
            Err(e) => return (StatusCode::NOT_FOUND, e.to_string()).into_response(),
        };
        let url = format!("{}/api/v1/mates/{}", base_url, participant_id);
        match client.get(url).send().await {
            Ok(res) => {
                if res.status().is_success() == false {
                    return (res.status(), res.text().await.unwrap()).into_response();
                }
                let mates = res.json::<Mates>().await.unwrap();
                (StatusCode::OK, Json(mates)).into_response()
            }
            Err(e) => (StatusCode::BAD_GATEWAY, e.to_string()).into_response(),
        }
    }

    async fn give_token(
        State(mate_repo): State<Arc<T>>,
        Json(payload): Json<RainbowBusinessLoginRequest>,
    ) -> impl IntoResponse {
        info!("GET /api/v1/busmates/token");

        debug!("{:?}", payload);


        let model = match mate_repo.get_busmate_by_id(payload.auth_request_id).await {
            Ok(model) => model,
            Err(e) => return (StatusCode::NOT_FOUND, e.to_string()).into_response(),
        };

        let mate = match mate_repo.get_mate_by_id(model.participant_id).await {
            Ok(mate) => mate,
            Err(_) => {
                return (
                    StatusCode::NOT_FOUND,
                    "You need to onboard_wallet on the Provider first",
                )
                    .into_response()
            }
        };


        let token = match model.token {
            Some(token) => token,
            None => return StatusCode::PROCESSING.into_response(),
        };

        let answer = json!({
            "token": token,
            "mate": mate

        });
        (StatusCode::OK, Json(answer)).into_response()
    }

    async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
        let log = format!("{} {}", method, uri);
        info!("{}", log);
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
}
