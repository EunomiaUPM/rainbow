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

use crate::core::traits::{AuthorityTrait, RainbowSSIAuthWalletTrait};
use crate::core::Authority;
use crate::data::repo_factory::factory_trait::AuthRepoFactoryTrait;
use crate::errors::helpers::BadFormat;
use crate::errors::{CustomToResponse, ErrorLog, Errors};
use crate::types::gnap::{GrantRequest, RefBody};
use crate::types::manager::VcManager;
use crate::types::oidc::VerifyPayload;
use crate::types::wallet::{DidsInfo, KeyDefinition};
use crate::utils::extract_gnap_token;
use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, Method, StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Form, Json, Router};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info};

pub struct RainbowAuthorityRouter<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + 'static,
{
    pub authority: Arc<Authority<T>>,
}

impl<T> RainbowAuthorityRouter<T>
where
    T: AuthRepoFactoryTrait + Send + Sync + 'static,
{
    pub fn new(authority: Arc<Authority<T>>) -> Self {
        Self { authority }
    }

    pub fn router(self) -> Router {
        Router::new()
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
            .route("/api/v1/request/credential", post(Self::access_request))
            .route("/api/v1/continue/:id", post(Self::continue_request))
            // OIDC4VCI
            .route("/api/v1/credentialOffer", get(Self::cred_offer))
            // ISSUER DATA
            .route(
                "/api/v1/.well-known/openid-credential-issuer",
                get(Self::issuer),
            )
            .route(
                "/api/v1/.well-known/oauth-authorization-server",
                get(Self::issuer_auth_server),
            )
            .route("/api/v1/jwks", get(Self::issuer_jwks))
            .route("/api/v1/token", post(Self::issuer_token))
            .route("/api/v1/credential", post(Self::issuer_credential))
            // OIDC4VP
            .route("/api/v1/pd/:state", get(Self::pd))
            .route("/api/v1/verify/:state", post(Self::verify))
            // VC REQUESTS
            .route("/api/v1/request/all", get(Self::get_all_requests))
            .route("/api/v1/request/:id", get(Self::get_one))
            .route("/api/v1/request/:id", post(Self::manage_request))
            // OTHER
            .route("/api/v1/callback/:id", post(Self::callback))
            .with_state(self.authority)
            .fallback(Self::fallback)
            .merge(openapi::route_openapi())
    }

    // WALLET ------------------------------------------------------------------------------------->

    async fn wallet_register(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/register");
        info!("POST /wallet/register");

        match manager.register_wallet().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn wallet_login(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/login");

        match manager.login_wallet().await {
            Ok(()) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn wallet_logout(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/logout");

        match manager.logout_wallet().await {
            Ok(()) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn wallet_onboard(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/onboard");

        match manager.onboard_wallet().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn partial_onboard(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/partial-onboard");

        match manager.partial_onboard().await {
            Ok(()) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn register_key(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/key");

        match manager.register_key().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn register_did(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("POST /wallet/did");

        match manager.register_did().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn delete_key(
        State(manager): State<Arc<Authority<T>>>,
        Json(payload): Json<KeyDefinition>,
    ) -> impl IntoResponse {
        info!("DELETE /wallet/key");

        match manager.delete_key(payload).await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn delete_did(State(manager): State<Arc<Authority<T>>>, Json(payload): Json<DidsInfo>) -> impl IntoResponse {
        info!("DELETE /wallet/did");

        match manager.delete_did(payload).await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn didweb(State(manager): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("GET /did.json");
        match manager.get_did_doc().await {
            Ok(did) => Json(did).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn access_request(
        State(authority): State<Arc<Authority<T>>>,
        Json(payload): Json<GrantRequest>,
    ) -> impl IntoResponse {
        info!("POST /access");

        match authority.manage_access(payload).await {
            Ok(response) => (StatusCode::OK, Json(response)).into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn continue_request(
        State(authority): State<Arc<Authority<T>>>,
        headers: HeaderMap,
        Path(id): Path<String>,
        Json(payload): Json<RefBody>,
    ) -> impl IntoResponse {
        let log = format!("POST /continue/{}", id);
        info!("{}", log);

        let token = match extract_gnap_token(headers) {
            Some(token) => token,
            None => {
                let error = Errors::unauthorized_new(Some("Missing token".to_string()));
                error!("{}", error.log());
                return error.into_response();
            }
        };

        let int_model = match authority.validate_continue_request(id, payload.interact_ref, token).await {
            Ok(model) => model,
            Err(e) => return e.to_response(),
        };

        let req_model = match authority.continue_req(int_model.clone()).await {
            Ok(model) => model,
            Err(e) => return e.to_response(),
        };

        let uri = req_model.vc_uri.unwrap(); // EXPECTED ALWAYS
        uri.into_response()
    }
    async fn cred_offer(
        State(authority): State<Arc<Authority<T>>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let id = match params.get("id") {
            Some(hash) => hash.clone(),
            None => {
                info!("GET /credentialOffer");
                let error = Errors::format_new(
                    BadFormat::Received,
                    Some("Unable to retrieve hash from callback".to_string()),
                );
                error!("{}", error.log());
                return error.into_response();
            }
        };
        let log = format!("GET /credentialOffer/{}", id);
        info!("{}", log);

        match authority.get_cred_offer_data(id).await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn issuer(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("GET /issuer");

        match authority.get_issuer().await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn issuer_auth_server(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("GET /auth-server");

        match authority.get_auth_server().await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn issuer_jwks(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("GET /issuer_jwks");

        match authority.get_parsed_keys().await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn issuer_token(State(authority): State<Arc<Authority<T>>>, body: Bytes) -> impl IntoResponse {
        info!("POST /token");
        // Body recibido (UTF-8): grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Apre-authorized_code&pre-authorized_code=TFWik9CMkjtCNeHjlAUeJ5RMdhmqSzQeljQqFThbGEY
        let body_str = String::from_utf8_lossy(&body);
        println!("Body recibido: {}", body_str); // Para ver qué llega

        // Generar respuesta aleatoria
        let resp = serde_json::json!({
            "access_token": "MOCK_TOKEN_123",
            "token_type": "Bearer",
            "expires_in": 3600
        }
        );

        (StatusCode::OK, Json(resp)).into_response()
    }

    async fn issuer_credential(
        State(authority): State<Arc<Authority<T>>>,
        headers: HeaderMap,
        body: Bytes,
    ) -> impl IntoResponse {
        info!("POST /credential");
        // Body recibido: grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Apre-authorized_code&pre-authorized_code=6Z37cZDpio6pAgAdkjbeiuBDfy_j4mEPmixmXXmYGzg
        // 2025-10-23T14:45:42.729208Z  INFO rainbow_authority::http: 303: POST /credential
        // Body recibido (UTF-8): {"format":"jwt_vc_json","proof":{"proof_type":"jwt","jwt":"eyJraWQiOiJkaWQ6andrOmV5SnJkSGtpT2lKU1UwRWlMQ0psSWpvaVFWRkJRaUlzSW00aU9pSjRUMEYzV2xwa1EzQXdRMGRDUVY5dE1WVkhSMVZYY0VNNVRISmxjbUpqYkRNeGJsSm9aVGh6TnpkNlIyMDRhWEEzWVVndFVXNU5OVU5MYkVvMVlYbHZVRGhLVFY5ckxXaFZRMFYyUTA5cU5XdzVWbU5oTjBSZk5UZFJNalJKVUV0SE4wZFFTR2RuWXpaNWJHeHdaRmxuU2tWbFkyeFVjWEpHY2pkR1VGQnhjVGhEUXpZMVdWcGZTMWhHWTNCeU9ISkdSVkZFVUd0a2MzZ3piVTFxVGtZelFVcE5OekJwYUVJdGRHdE9PV3h4Y0haTGIwaDZNekpSYUhSeWJWaEZZMnQxWDFOdFZGbEZUVTlHVlUweGQwSTFURTR4Tm1ONFp6SXhaWFl3TW5aR2RsVkNZbWRXWkVOVE5IWm9jak5ITmxOTU0yZFBaWFZ6ZDNCWmFUWnRUVFo2Y2xsb1kzbGthREphUmpaQ1VFWlZNUzFOTms5dGNFTkdSbmQ2U0ZWV05UZHFNSEpNTFVSME9VMUphMUZVV214WGFtNWtjVlZPWkcxRWNubHlkMEUxT1U4elMxbFdZV1k1Y21Vd2RXOVFTM1pHVW1SbVkzY2lmUSMwIiwidHlwIjoib3BlbmlkNHZjaS1wcm9vZitqd3QiLCJhbGciOiJSUzI1NiJ9.eyJpc3MiOiJkaWQ6andrOmV5SnJkSGtpT2lKU1UwRWlMQ0psSWpvaVFWRkJRaUlzSW00aU9pSjRUMEYzV2xwa1EzQXdRMGRDUVY5dE1WVkhSMVZYY0VNNVRISmxjbUpqYkRNeGJsSm9aVGh6TnpkNlIyMDRhWEEzWVVndFVXNU5OVU5MYkVvMVlYbHZVRGhLVFY5ckxXaFZRMFYyUTA5cU5XdzVWbU5oTjBSZk5UZFJNalJKVUV0SE4wZFFTR2RuWXpaNWJHeHdaRmxuU2tWbFkyeFVjWEpHY2pkR1VGQnhjVGhEUXpZMVdWcGZTMWhHWTNCeU9ISkdSVkZFVUd0a2MzZ3piVTFxVGtZelFVcE5OekJwYUVJdGRHdE9PV3h4Y0haTGIwaDZNekpSYUhSeWJWaEZZMnQxWDFOdFZGbEZUVTlHVlUweGQwSTFURTR4Tm1ONFp6SXhaWFl3TW5aR2RsVkNZbWRXWkVOVE5IWm9jak5ITmxOTU0yZFBaWFZ6ZDNCWmFUWnRUVFo2Y2xsb1kzbGthREphUmpaQ1VFWlZNUzFOTms5dGNFTkdSbmQ2U0ZWV05UZHFNSEpNTFVSME9VMUphMUZVV214WGFtNWtjVlZPWkcxRWNubHlkMEUxT1U4elMxbFdZV1k1Y21Vd2RXOVFTM1pHVW1SbVkzY2lmUSIsInN1YiI6ImRpZDpqd2s6ZXlKcmRIa2lPaUpTVTBFaUxDSmxJam9pUVZGQlFpSXNJbTRpT2lKNFQwRjNXbHBrUTNBd1EwZENRVjl0TVZWSFIxVlhjRU01VEhKbGNtSmpiRE14YmxKb1pUaHpOemQ2UjIwNGFYQTNZVWd0VVc1Tk5VTkxiRW8xWVhsdlVEaEtUVjlyTFdoVlEwVjJRMDlxTld3NVZtTmhOMFJmTlRkUk1qUkpVRXRITjBkUVNHZG5Zelo1Ykd4d1pGbG5Ta1ZsWTJ4VWNYSkdjamRHVUZCeGNUaERRelkxV1ZwZlMxaEdZM0J5T0hKR1JWRkVVR3RrYzNnemJVMXFUa1l6UVVwTk56QnBhRUl0ZEd0T09XeHhjSFpMYjBoNk16SlJhSFJ5YlZoRlkydDFYMU50VkZsRlRVOUdWVTB4ZDBJMVRFNHhObU40WnpJeFpYWXdNblpHZGxWQ1ltZFdaRU5UTkhab2NqTkhObE5NTTJkUFpYVnpkM0JaYVRadFRUWjZjbGxvWTNsa2FESmFSalpDVUVaVk1TMU5Oazl0Y0VOR1JuZDZTRlZXTlRkcU1ISk1MVVIwT1UxSmExRlVXbXhYYW01a2NWVk9aRzFFY25seWQwRTFPVTh6UzFsV1lXWTVjbVV3ZFc5UVMzWkdVbVJtWTNjaWZRIiwianRpIjoiNzcyZWIyNzAtZTM0NS00N2UyLTljNjMtMjEwNTViYjAyN2IyIiwiYXVkIjoiaHR0cDovL2hvc3QuZG9ja2VyLmludGVybmFsOjE1MDAvYXBpL3YxIiwiaWF0IjoxNzYxMjMwNzQyLCJleHAiOjE3NjEyMzEwNDJ9.IzO729kQK0nV1JbDLcge6hYaan60bBqtooYLz5XPThEvRe4eGPWTJZ3_gzGvOCHgQTCPGJz6qu6IBzxu-IProflKcMArrfumP078GYDSRWbluIdfdrYyRFXGN6doXR4oo6Rb0SDN1S7M7bYxCNkwsNfRQnWIn9Rq2tZS4G8qKCHrI5aThgwGa9nGR7y-xAm6LKTpAiRzGDs90BoJFcpFi_J5CIi4FqtOC6FZPTIIc0Yp_-7hmEo_JuHUv_CRhUDPy3dQBClexxFCbPYUpCBhdF5ziGE1q99Yh7AIG_OZtocZ7iE8OOBR8-Ph9eDYGp1jpMmbqgB-b8D7CL2GHF1V0w"},"credential_definition":{"type":["VerifiableCredential","IdentityCredential"]}}
        // Headers recibidos:
        //     authorization: "Bearer MOCK_TOKEN_123"
        // accept: "application/json"
        // accept-charset: "UTF-8"
        // user-agent: "ktor-client"
        // content-type: "application/json"
        // content-length: "2777"
        // host: "host.docker.internal:1500"
        // connection: "Keep-Alive"
        // accept-encoding: "gzip"

        let body_str = String::from_utf8_lossy(&body);
        println!("Body recibido (UTF-8): {}", body_str);
        println!("Headers recibidos:");
        for (name, value) in headers.iter() {
            println!("{}: {:?}", name, value);
        }

        let vc_jwt = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJodHRwOi8vbG9jYWxob3N0OjcwMDIvZHJhZnQxMyIsInN1YiI6ImRpZDpleGFtcGxlOmFiY2RlZjEyMzQ1NiIsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIl0sImlkIjoidXJuOnV1aWQ6MTIzNDU2NzgtMTIzNC0xMjM0LTEyMzQtMTIzNDU2Nzg5YWJjIiwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIklkZW50aXR5Q3JlZGVudGlhbCJdLCJpc3VlciI6Imh0dHA6Ly9sb2NhbGhvc3Q6NzAwMi9kcmFmdDEzIiwiaXNzdWFuY2VEYXRlIjoiMjAyNS0xMC0yM1QxNDowMDowMFoiLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpleGFtcGxlOmFiY2RlZjEyMzQ1NiIsIm5hbWUiOiJKb2huIERvZSJ9fX0.MOCK_SIGNATURE";
        let response = serde_json::json!({
        "format": "jwt_vc_json",
        "credential": vc_jwt,
    });

        (StatusCode::OK, Json(response)).into_response()
    }

    async fn pd(State(authority): State<Arc<Authority<T>>>, Path(state): Path<String>) -> impl IntoResponse {
        let log = format!("GET /pd/{}", state);
        info!("{}", log);

        match authority.generate_vp_def(state).await {
            Ok(vpd) => Json(vpd).into_response(),
            Err(e) => e.to_response(),
        }
    }
    async fn verify(
        State(authority): State<Arc<Authority<T>>>,
        Path(state): Path<String>,
        Form(payload): Form<VerifyPayload>,
    ) -> impl IntoResponse {
        let log = format!("POST /verify/{}", state);
        info!("{}", log);

        // {payload.vp_token,payload.presentation_submission}

        let id = match authority.verify_all(state, payload.vp_token).await {
            Ok(id) => id,
            Err(e) => return e.to_response(),
        };

        match authority.end_verification(id).await {
            Ok(Some(uri)) => (StatusCode::OK, uri).into_response(),
            Ok(None) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn get_all_requests(State(authority): State<Arc<Authority<T>>>) -> impl IntoResponse {
        info!("GET /request/all");

        match authority.repo.request().get_all(None, None).await {
            Ok(data) => {
                let res = serde_json::to_value(data).unwrap(); // EXPECTED ALWAYS
                (StatusCode::OK, Json(res)).into_response()
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                error.into_response()
            }
        }
    }

    async fn get_one(State(authority): State<Arc<Authority<T>>>, Path(id): Path<String>) -> impl IntoResponse {
        let log = format!("GET /request/{}", &id);
        info!("{}", log);

        match authority.repo.request().get_by_id(id.as_str()).await {
            Ok(Some(data)) => {
                let res = serde_json::to_value(data).unwrap();
                (StatusCode::OK, Json(res)).into_response()
            }
            Ok(None) => {
                let error = Errors::missing_resource_new(id.clone(), Some(format!("Missing request with id: {}", id)));
                error!("{}", error.log());
                error.into_response()
            }
            Err(e) => {
                let error = Errors::database_new(Some(e.to_string()));
                error!("{}", error.log());
                error.into_response()
            }
        }
    }

    async fn manage_request(
        State(authority): State<Arc<Authority<T>>>,
        Path(id): Path<String>,
        Json(payload): Json<VcManager>,
    ) -> impl IntoResponse {
        let log = format!("POST /request/{}", &id);
        info!("{}", log);

        match authority.manage_vc_request(id, payload).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn callback(
        State(authority): State<Arc<Authority<T>>>,
        Path(id): Path<String>,
        Json(payload): Json<Value>,
    ) -> impl IntoResponse {
        let log = format!("POST /callback/{}", &id);
        info!("{}", log);

        match authority.manage_callback(id, payload).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn fallback(method: Method, uri: Uri) -> (StatusCode, String) {
        let log = format!("{} {}", method, uri);
        error!("Unexpected route");
        error!("{}", log);
        (StatusCode::NOT_FOUND, format!("No route for {uri}"))
    }
}
