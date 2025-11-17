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

use crate::core::traits::CoreWalletTrait;
use crate::errors::CustomToResponse;
use crate::types::wallet::{DidsInfo, KeyDefinition};
use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use std::sync::Arc;

pub struct WalletRouter {
    holder: Arc<dyn CoreWalletTrait>,
}

impl WalletRouter {
    pub fn new(holder: Arc<dyn CoreWalletTrait>) -> Self {
        Self { holder }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/register", post(Self::register))
            .route("/login", post(Self::login))
            .route("/logout", post(Self::logout))
            .route("/onboard", post(Self::onboard))
            .route("/partial-onboard", post(Self::partial_onboard))
            .route("/key", post(Self::register_key))
            .route("/did", post(Self::register_did))
            .route("/key", delete(Self::delete_key))
            .route("/did", delete(Self::delete_did))
            .route("/did.json", get(Self::did_json))
            .with_state(self.holder)
    }

    async fn register(State(holder): State<Arc<dyn CoreWalletTrait>>) -> impl IntoResponse {
        match holder.register().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn login(State(holder): State<Arc<dyn CoreWalletTrait>>) -> impl IntoResponse {
        match holder.login().await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn logout(State(holder): State<Arc<dyn CoreWalletTrait>>) -> impl IntoResponse {
        match holder.logout().await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn onboard(State(holder): State<Arc<dyn CoreWalletTrait>>) -> impl IntoResponse {
        match holder.onboard().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn partial_onboard(State(holder): State<Arc<dyn CoreWalletTrait>>) -> impl IntoResponse {
        match holder.partial_onboard().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn register_key(State(holder): State<Arc<dyn CoreWalletTrait>>) -> impl IntoResponse {
        match holder.register_key().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn register_did(State(holder): State<Arc<dyn CoreWalletTrait>>) -> impl IntoResponse {
        match holder.register_did().await {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn delete_key(
        State(holder): State<Arc<dyn CoreWalletTrait>>,
        payload: Result<Json<KeyDefinition>, JsonRejection>,
    ) -> impl IntoResponse {
        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => return e.into_response(),
        };

        match holder.delete_key(payload).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn delete_did(
        State(holder): State<Arc<dyn CoreWalletTrait>>,
        payload: Result<Json<DidsInfo>, JsonRejection>,
    ) -> impl IntoResponse {
        let payload = match payload {
            Ok(Json(data)) => data,
            Err(e) => return e.into_response(),
        };

        match holder.delete_did(payload).await {
            Ok(_) => StatusCode::OK.into_response(),
            Err(e) => e.to_response(),
        }
    }

    async fn did_json(State(holder): State<Arc<dyn CoreWalletTrait>>) -> impl IntoResponse {
        match holder.get_did_doc().await {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.to_response(),
        }
    }
}
