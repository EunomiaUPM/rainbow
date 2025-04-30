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

// use crate::ssi_auth::consumer::core::{consumer_vc_request, ConsumerSSIVCRequest};
// use anyhow::bail;
// use rainbow_common::err::transfer_err::TransferErrorType;

use crate::ssi_auth::consumer::manager::Manager;
use crate::ssi_auth::consumer::types::ReachProvider;
use axum::extract::{Path, Query, State};
use axum::http::{Method, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use once_cell::sync::Lazy;
use rainbow_db::auth_consumer::repo::AuthConsumerRepoTrait;
use reqwest::StatusCode;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use url::Url;
use urlencoding::decode;

pub struct RainbowAuthConsumerRouter<T>
where
    T: AuthConsumerRepoTrait + Send + Sync + Clone + 'static,
{
    pub manager: Arc<Mutex<Manager<T>>>,
}

impl<T> RainbowAuthConsumerRouter<T>
where
    T: AuthConsumerRepoTrait + Send + Sync + Clone + 'static,
{
    pub fn new(auth_repo: Arc<T>) -> Self {
        let manager = Arc::new(Mutex::new(Manager::new(auth_repo)));
        Self { manager }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/wallet/register", post(Self::wallet_register))
            .route("/wallet/login", post(Self::wallet_login))
            .route("/wallet/logout", post(Self::wallet_logout))
            .route("/wallet/onboard", post(Self::wallet_oboard))
            .route("/auth/ssi", post(Self::auth_ssi))
            .route("/callback/:id", get(Self::callback))
            .with_state(self.manager)
            .fallback(Self::fallback)
    }

    async fn wallet_register(State(manager): State<Arc<Mutex<Manager<T>>>>) -> impl IntoResponse {
        info!("POST /wallet/register");

        let mut manager = manager.lock().await;
        match manager.register_wallet().await {
            Ok(()) => StatusCode::CREATED,
            Err(e) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    async fn wallet_login(State(manager): State<Arc<Mutex<Manager<T>>>>) -> impl IntoResponse {
        info!("POST /wallet/login");

        let mut manager = manager.lock().await;
        match manager.login_wallet().await {
            Ok(()) => StatusCode::OK,
            Err(e) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    async fn wallet_logout(State(manager): State<Arc<Mutex<Manager<T>>>>) -> impl IntoResponse {
        info!("POST /wallet/logout");

        let mut manager = manager.lock().await;
        match manager.logout_wallet().await {
            Ok(()) => StatusCode::OK,
            Err(e) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    async fn wallet_oboard(State(manager): State<Arc<Mutex<Manager<T>>>>) -> impl IntoResponse {
        info!("POST /wallet/onboard");

        let mut manager = manager.lock().await;
        match manager.onboard().await {
            Ok(()) => StatusCode::CREATED,
            Err(e) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    async fn auth_ssi(
        State(manager): State<Arc<Mutex<Manager<T>>>>,
        Json(payload): Json<ReachProvider>,
    ) -> impl IntoResponse {
        info!("POST /auth/ssi");

        let mut manager = manager.lock().await;

        match manager.onboard().await {
            Ok(()) => {}
            Err(e) => {
                return {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Wallet Onboarding Failed"})),
                    )
                        .into_response()
                }
            }
        }

        let mut auth_ver;
        match manager.request_access(payload.url, payload.id, payload.actions).await {
            Ok(auth_ver_model) => auth_ver = auth_ver_model,
            Err(e) => {
                return {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Error contacting the provider"})),
                    )
                        .into_response()
                }
            }
        }

        let mut vpd_as_string;
        match manager.join_exchange(auth_ver.uri).await {
            Ok(texto) => vpd_as_string = texto,
            Err(e) => {
                return {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Retrieving the Presentation Definition"})),
                    )
                        .into_response()
                }
            }
        }

        let vpd = match manager.parse_vpd(vpd_as_string.clone()).await {
            Ok(json) => json,
            Err(e) => {
                return {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Parsing the Presentation Definition"})),
                    )
                        .into_response()
                }
            }
        };

        let vcs = match manager.match_vc4vp(vpd).await {
            Ok(vcs) => vcs,
            Err(e) => {
                return {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Error retrieving credentials matching the vp"})),
                    )
                        .into_response()
                }
            }
        };

        let mut creds = Vec::new();
        for vc in vcs {
            creds.push(vc.id);
        }

        let res = match manager.present_vp(vpd_as_string, creds).await {
            Ok(vcs) => (
                StatusCode::OK,
                Json(json!({
                    "TODO CORRECTO": "ASI ES"
                })),
            )
                .into_response(),
            Err(e) => {
                return {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Error presenting credentials"})),
                    )
                        .into_response()
                }
            }
        };
        res
    }

    async fn callback(
        State(manager): State<Arc<Mutex<Manager<T>>>>,
        Path(state): Path<String>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        info!("GET /callback");

        let nonce = match params.get("nonce") {
            Some(nonce) => nonce,
            None => return StatusCode::BAD_REQUEST.into_response(),
        };



        StatusCode::OK.into_response()
    }

    async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
        let log = format!("{} {}", method, uri);
        info!("{}", log);
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
}

// ---------------------------------------------------------------------

// async fn login() -> impl IntoResponse {
//     let mut session = SESSION_MANAGER.lock().await;
//
//     match session.access().await {
//         Ok(()) => {
//             info!("Sesion creada con éxito");
//             Json("Login successful")
//         }
//         Err(err) => {
//             info!("FRACASO");
//             Json("FRACASO")
//         }
//     }
// }
//
// async fn url(Json(payload): Json<MessageRequest>) -> impl IntoResponse {
//     let mut session = SESSION_MANAGER.lock().await;
//
//     match session.access().await {
//         Ok(()) => {
//             info!("Sesion creada con éxito");
//         }
//         Err(err) => {
//             info!("FRACASO");
//         }
//     }
//
//     match session.joinexchange(payload.message).await {
//         Ok(string) => {
//             println!();
//             println!("UURRLL: {}", string);
//             println!()
//         }
//         Err(err) => println!("errror: {}", err),
//     }
//
//     StatusCode::OK
// }
//
// async fn matcheo(Json(payload): Json<MessageRequest>) -> impl IntoResponse {
//     let mut session = SESSION_MANAGER.lock().await;
//
//     match session.access().await {
//         Ok(()) => {
//             info!("Sesion creada con éxito");
//         }
//         Err(err) => {
//             info!("FRACASO");
//         }
//     }
//
//     let res =
//         session.joinexchange(payload.message).await.unwrap_or_else(|err| String::from("ERROR"));
//     let url = Url::parse(decode(&res).unwrap().as_ref()).unwrap();
//
//     if let Some((_, vpd)) = url.query_pairs().find(|(key, _)| key == "presentation_definition") {
//         println!("vp: {}", vpd);
//
//         match serde_json::from_str::<Value>(&vpd) {
//             Ok(json) => {
//                 println!("json: {:?}", json);
//             }
//             Err(err) => {
//                 println!("ERROR")
//             }
//         }
//     } else {
//         println!("ERROR");
//     }
//
//     StatusCode::OK
// }
//
// async fn exchange(Json(payload): Json<MessageRequest>) -> impl IntoResponse {
//     let mut session = SESSION_MANAGER.lock().await;
//
//     match session.access().await {
//         Ok(()) => {
//             info!("Sesion creada con éxito");
//         }
//         Err(err) => {
//             info!("FRACASO");
//         }
//     }
//
//     let res =
//         session.joinexchange(payload.message).await.unwrap_or_else(|err| String::from("ERROR"));
//     println!();
//     println!("UURRLL1: {}", res);
//     println!();
//
//     let url = Url::parse(decode(&res).unwrap().as_ref()).unwrap();
//
//     println!();
//     println!("UURRLL2: {}", url);
//     println!();
//
//     if let Some((_, vpd)) = url.query_pairs().find(|(key, _)| key == "presentation_definition") {
//         let vpd = serde_json::from_str::<Value>(&vpd).unwrap();
//
//         let vcs = session.match_vc4vp(vpd).await.unwrap();
//
//         let mut creds = Vec::new();
//         for vc in vcs {
//             creds.push(vc.id);
//         }
//
//         let kk = session.present_vp(res, creds).await.unwrap();
//     } else {
//         println!("ERROR");
//     }
//
//     StatusCode::OK
// }
//
//
// use serde::{Deserialize, Serialize};
//
// #[derive(Deserialize, Serialize)]
// struct MessageRequest {
//     message: String,
// }
