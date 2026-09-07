/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::Arc;

use axum::extract::rejection::JsonRejection;
use axum::extract::{Extension, Path, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::get;
use axum::{Json, Router, middleware};
use common::auth::claims::Claims;
use common::auth::middleware::bearer;
use common::auth::rbac::Rbac;
use ymir::errors::AppResult;
use ymir::utils::extract_payload;

use crate::entities::commands::CreateClientCommand;
use crate::http::forms::ClientView;
use crate::services::client_service::ClientServiceTrait;
use crate::services::token_service::TokenServiceTrait;

#[derive(Clone)]
pub(crate) struct ClientsRouter {
    token_svc: Arc<dyn TokenServiceTrait>,
    client_svc: Arc<dyn ClientServiceTrait>,
}

impl ClientsRouter {
    pub(crate) fn new(
        token_svc: Arc<dyn TokenServiceTrait>,
        client_svc: Arc<dyn ClientServiceTrait>,
    ) -> Self {
        Self {
            token_svc,
            client_svc,
        }
    }

    pub(crate) fn router(self) -> Router {
        let protected = Router::new()
            .route("/", get(Self::handle_list).post(Self::handle_create))
            .route(
                "/{id}",
                get(Self::handle_get_one).delete(Self::handle_delete),
            )
            .route_layer(middleware::from_fn_with_state(
                self.clone(),
                Self::auth_middleware,
            ));

        Router::new().merge(protected).with_state(self)
    }

    async fn auth_middleware(
        State(s): State<Self>,
        mut req: Request,
        next: Next,
    ) -> AppResult<Response> {
        let token = bearer(req.headers())?.to_owned();
        let claims = s.token_svc.validate_token(&token).await?;
        req.extensions_mut().insert(claims);
        Ok(next.run(req).await)
    }

    async fn handle_list(
        State(s): State<Self>,
        Extension(claims): Extension<Claims>,
    ) -> AppResult<Json<Vec<ClientView>>> {
        Rbac::require_admin(&claims)?;
        Ok(Json(s.client_svc.list_clients().await?))
    }

    async fn handle_get_one(
        State(s): State<Self>,
        Extension(claims): Extension<Claims>,
        Path(id): Path<String>,
    ) -> AppResult<Json<ClientView>> {
        Rbac::require_read(&claims, &id)?;
        Ok(Json(s.client_svc.get_client(&id).await?))
    }

    async fn handle_create(
        State(s): State<Self>,
        Extension(claims): Extension<Claims>,
        payload: Result<Json<CreateClientCommand>, JsonRejection>,
    ) -> AppResult<(StatusCode, Json<ClientView>)> {
        Rbac::require_admin(&claims)?;
        let cmd = extract_payload(payload)?;
        Ok((
            StatusCode::CREATED,
            Json(s.client_svc.create_client(&cmd).await?),
        ))
    }

    async fn handle_delete(
        State(s): State<Self>,
        Extension(claims): Extension<Claims>,
        Path(id): Path<String>,
    ) -> AppResult<StatusCode> {
        Rbac::require_admin(&claims)?;
        s.client_svc.delete_client(&id).await?;
        Ok(StatusCode::NO_CONTENT)
    }
}
