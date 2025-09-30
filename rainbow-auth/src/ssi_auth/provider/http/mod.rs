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
mod openapi;

use crate::ssi_auth::common::errors::CustomToResponse;
use crate::ssi_auth::common::traits::RainbowSSIAuthWalletTrait;
use crate::ssi_auth::common::types::gnap::{AccessToken, GrantRequest, RefBody};
use crate::ssi_auth::common::types::ssi::{dids::DidsInfo, keys::KeyDefinition};
use crate::ssi_auth::common::utils::token::extract_gnap_token;
use crate::ssi_auth::provider::core::traits::provider_trait::RainbowSSIAuthProviderManagerTrait;
use crate::ssi_auth::provider::core::Manager;
use axum::extract::{Form, Path, State};
use axum::http::{HeaderMap, Method, Uri};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use rainbow_common::auth::business::RainbowBusinessLoginRequest;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::mates::mates::VerifyTokenRequest;
use rainbow_db::auth_provider::repo_factory::factory_trait::AuthRepoFactoryTrait;
use reqwest::StatusCode;
use serde::Deserialize;
use std::sync::Arc;
use tracing::{error, info};

pub struct RainbowAuthProviderRouter<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    pub manager: Arc<Manager<T>>,
}

impl<T> RainbowAuthProviderRouter<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + Clone + 'static,
{
    pub fn new(manager: Arc<Manager<T>>) -> Self {
        Self { manager }
    }
    pub fn router(self) -> Router {
        let router = Router::new()
            // WALLET
            .route("/api/v1/wallet/register", post(Self::wallet_register))
            .route("/api/v1/wallet/login", post(Self::wallet_login))
            .route("/api/v1/wallet/logout", post(Self::wallet_logout))
            .route("/api/v1/wallet/onboard", post(Self::wallet_onboard))
            .route(
                "/api/v1/wallet/partial-onboard",
                post(Self::partial_onboard),
            )
            .route("/api/v1/wallet/key", post(Self::register_key))
            .route("/api/v1/wallet/did", post(Self::register_did))
            .route("/api/v1/wallet/key", delete(Self::delete_key))
            .route("/api/v1/wallet/did", delete(Self::delete_did))
            .route("/api/v1/did.json", get(Self::didweb))
            // GNAP
            .route("/api/v1/access", post(Self::access_request))
            .route("/api/v1/continue/:id", post(Self::continue_request))
            // OIDC4VP
            .route("/api/v1/pd/:state", get(Self::pd))
            .route("/api/v1/verify/:state", post(Self::verify))
            // MATES
            .route("/api/v1/verify/mate/token", post(Self::verify_mate_token))
            .route(
                "/api/v1/retrieve/business/token",
                post(Self::retrieve_business_mate_token),
            )
            .route("/api/v1/business/login", post(Self::fast_login))
            .with_state(self.manager)
            .fallback(Self::fallback); // 2 routers cannot have 1 fallback each

        router.merge(openapi::route_openapi()) // OPENAPI STUFF
    }

    // WALLET ------------------------------------------------------------------------------------->

    async fn wallet_register(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("POST /wallet/register");

        match manager.register_wallet().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn wallet_login(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("POST /wallet/login");

        match manager.login_wallet().await {
            Ok(()) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn wallet_logout(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("POST /wallet/logout");

        match manager.logout_wallet().await {
            Ok(()) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn wallet_onboard(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("POST /wallet/onboard");

        match manager.onboard_wallet().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn partial_onboard(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("POST /wallet/partial-onboard");

        match manager.partial_onboard().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn register_key(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("POST /wallet/key");

        match manager.register_key().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn register_did(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("POST /wallet/did");

        match manager.register_did().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn delete_key(
        State(manager): State<Arc<Manager<T>>>,
        Json(payload): Json<KeyDefinition>,
    ) -> impl IntoResponse {
        info!("DELETE /wallet/key");

        match manager.delete_key(payload).await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn delete_did(State(manager): State<Arc<Manager<T>>>, Json(payload): Json<DidsInfo>) -> impl IntoResponse {
        info!("DELETE /wallet/did");

        match manager.delete_did(payload).await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn didweb(State(manager): State<Arc<Manager<T>>>) -> impl IntoResponse {
        info!("GET /did.json");
        match manager.get_did_doc().await {
            Ok(did) => Json(did).into_response(),
            Err(e) => e.to_response(),
        }
    }

    //GNAP

    async fn access_request(
        State(manager): State<Arc<Manager<T>>>,
        Json(payload): Json<GrantRequest>,
    ) -> impl IntoResponse {
        info!("POST /access");

        match manager.manage_access(payload).await {
            Ok(response) => (StatusCode::OK, Json(response)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn continue_request(
        State(manager): State<Arc<Manager<T>>>,
        headers: HeaderMap,
        Path(id): Path<String>,
        Json(payload): Json<RefBody>,
    ) -> impl IntoResponse {
        let log = format!("POST /continue/{}", id);
        info!("{}", log);

        let token = match extract_gnap_token(headers) {
            Some(token) => token,
            None => {
                let error = CommonErrors::unauthorized_new(Some("Missing token".to_string()));
                error!("{}", error.log());
                return error.into_response();
            }
        };

        let int_model = match manager.validate_continue_request(id, payload.interact_ref, token).await {
            Ok(model) => model,
            Err(e) => return e.to_response(),
        };

        let req_model = match manager.continue_req(int_model.clone()).await {
            Ok(model) => model,
            Err(e) => return e.to_response(),
        };

        let mate = match manager.retrieve_data(req_model.clone(), int_model).await {
            Ok(mate) => mate,
            Err(e) => return e.to_response(),
        };

        match manager.save_mate(mate).await {
            Ok(_) => (),
            Err(e) => return e.to_response(),
        }

        let res = AccessToken::default(req_model.token.unwrap());

        (StatusCode::OK, Json(res)).into_response()
    }

    // OIDC

    async fn pd(State(manager): State<Arc<Manager<T>>>, Path(state): Path<String>) -> impl IntoResponse {
        let log = format!("GET /pd/{}", state);
        info!("{}", log);

        // COMPLETAR CON REQUIREMENTS
        match manager.generate_vp_def(state).await {
            Ok(vpd) => Json(vpd).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn verify(
        State(manager): State<Arc<Manager<T>>>,
        Path(state): Path<String>,
        Form(payload): Form<VerifyPayload>,
    ) -> impl IntoResponse {
        let log = format!("POST /verify/{}", state);
        info!("{}", log);

        // {payload.vp_token,payload.presentation_submission}

        let id = match manager.verify_all(state, payload.vp_token).await {
            Ok(id) => id,
            Err(e) => return e.to_response(),
        };

        match manager.end_verification(id).await {
            Ok(Some(uri)) => (StatusCode::OK, uri).into_response(),
            Ok(None) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn verify_mate_token(
        State(manager): State<Arc<Manager<T>>>,
        Json(payload): Json<VerifyTokenRequest>,
    ) -> impl IntoResponse {
        info!("POST /verify/mate/token");

        let mate = match manager.verify_token(payload.token).await {
            Ok(model) => model,
            Err(e) => return e.to_response(),
        };
        (StatusCode::OK, Json(mate)).into_response()
    }

    async fn retrieve_business_mate_token(
        State(manager): State<Arc<Manager<T>>>,
        Json(payload): Json<RainbowBusinessLoginRequest>,
    ) -> impl IntoResponse {
        info!("POST /retrieve/business/token");

        let response = match manager.retrieve_business_token(payload.auth_request_id).await {
            Ok(res) => res,
            Err(e) => return e.to_response(),
        };

        (StatusCode::OK, Json(response)).into_response()
    }

    async fn fast_login(
        State(manager): State<Arc<Manager<T>>>,
        Json(payload): Json<RainbowBusinessLoginRequest>,
    ) -> impl IntoResponse {
        info!("POST /business/login");

        let uri = match manager.fast_login(payload.auth_request_id).await {
            Ok(uri) => uri,
            Err(e) => return e.to_response(),
        };

        uri.into_response()
    }

    async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
        let log = format!("{} {}", method, uri);
        error!("Unexpected route");
        error!("{}", log);
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
}

#[derive(Deserialize)]
struct VerifyPayload {
    vp_token: String,
    // presentation_submission: String,
}
