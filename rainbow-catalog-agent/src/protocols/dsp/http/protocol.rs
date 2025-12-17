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

use crate::protocols::dsp::orchestrator::OrchestratorTrait;
use crate::protocols::dsp::protocol_types::{CatalogMessageWrapper, CatalogRequestMessageDto, DatasetRequestMessage};
use axum::{
    extract::{rejection::JsonRejection, FromRef, Path, State},
    response::IntoResponse,
    routing::get,
    routing::post,
    Json, Router,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct DspRouter {
    orchestrator: Arc<dyn OrchestratorTrait>,
}

impl FromRef<DspRouter> for Arc<dyn OrchestratorTrait> {
    fn from_ref(state: &DspRouter) -> Self {
        state.orchestrator.clone()
    }
}

impl DspRouter {
    pub fn new(service: Arc<dyn OrchestratorTrait>) -> Self {
        Self { orchestrator: service }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/request", post(Self::handle_catalog_request))
            .route("/datasets/:id", get(Self::handle_dataset_request))
            .with_state(self)
    }

    async fn handle_catalog_request(
        State(state): State<DspRouter>,
        input: Result<Json<CatalogMessageWrapper<CatalogRequestMessageDto>>, JsonRejection>,
    ) -> impl IntoResponse {
        "ok"
    }

    async fn handle_dataset_request(
        State(state): State<DspRouter>,
        Path(id): Path<String>,
        input: Result<Json<CatalogMessageWrapper<DatasetRequestMessage>>, JsonRejection>,
    ) -> impl IntoResponse {
        "ok"
    }
}
