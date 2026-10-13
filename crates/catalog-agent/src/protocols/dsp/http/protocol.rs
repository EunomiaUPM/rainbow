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

use crate::protocols::dsp::orchestrator::OrchestratorTrait;
use crate::protocols::dsp::protocol_types::{
    CatalogMessageType, CatalogMessageWrapper, CatalogRequestMessageDto, DatasetRequestMessage,
};
use axum::http::StatusCode;
use axum::{
    extract::{rejection::JsonRejection, FromRef, Path, State},
    middleware,
    response::IntoResponse,
    routing::get,
    routing::post,
    Extension, Json, Router,
};
use common::oauth::UserInfo;
use common::dsp_common::context_field::ContextField;
use common::dsp_common::normalizer::dsp_namespace_normalizer;
use common::dsp_common::PeerAuth;
use common::facades::grants_facade::VerifiedPeer;
use std::str::FromStr;
use std::sync::Arc;
use urn::Urn;

#[derive(Clone)]
pub struct DspRouter {
    orchestrator: Arc<dyn OrchestratorTrait>,
    peer_auth: PeerAuth,
}

impl FromRef<DspRouter> for Arc<dyn OrchestratorTrait> {
    fn from_ref(state: &DspRouter) -> Self {
        state.orchestrator.clone()
    }
}

impl DspRouter {
    pub fn new(service: Arc<dyn OrchestratorTrait>, peer_auth: PeerAuth) -> Self {
        Self {
            orchestrator: service,
            peer_auth,
        }
    }

    /// The authenticated peer as an actor: its DID under the role of its grant, so it reads
    /// the catalogs that role reaches (all of them with the default `/admin`).
    fn peer_user(peer: &VerifiedPeer) -> UserInfo {
        peer.to_user()
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/request", post(Self::handle_catalog_request))
            .route("/datasets/{id}", get(Self::handle_dataset_request))
            .layer(middleware::from_fn_with_state(
                self.peer_auth.clone(),
                PeerAuth::run,
            ))
            .layer(middleware::from_fn(dsp_namespace_normalizer))
            .with_state(self)
    }

    async fn handle_catalog_request(
        State(state): State<DspRouter>,
        Extension(peer): Extension<VerifiedPeer>,
        input: Result<Json<CatalogMessageWrapper<CatalogRequestMessageDto>>, JsonRejection>,
    ) -> impl IntoResponse {
        let input = match input {
            Ok(input) => input.0,
            Err(e) => return (StatusCode::BAD_REQUEST, e.body_text()).into_response(),
        };
        let user = Self::peer_user(&peer);
        match state
            .orchestrator
            .get_protocol_service()
            .on_catalog_request(&user, &input)
            .await
        {
            Ok(catalog) => (StatusCode::OK, Json(catalog)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }

    async fn handle_dataset_request(
        State(state): State<DspRouter>,
        Path(id): Path<String>,
        Extension(peer): Extension<VerifiedPeer>,
    ) -> impl IntoResponse {
        let user = Self::peer_user(&peer);
        let dataset_id = match Urn::from_str(&id) {
            Ok(urn) => urn,
            Err(_) => {
                return (StatusCode::BAD_REQUEST, format!("Invalid dataset ID: {id}"))
                    .into_response()
            }
        };
        let request_msg = CatalogMessageWrapper {
            context: ContextField::default(),
            _type: CatalogMessageType::DatasetRequestMessage,
            dto: DatasetRequestMessage {
                dataset: dataset_id,
            },
        };
        match state
            .orchestrator
            .get_protocol_service()
            .on_dataset_request(&user, &request_msg)
            .await
        {
            Ok(dataset) => (StatusCode::OK, Json(dataset)).into_response(),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
        }
    }
}
