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

use axum::{
    Json, Router,
    extract::{FromRef, Path, State, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Serialize;
use std::future::Future;
use std::sync::Arc;

use crate::protocols::dsp::errors::extract_payload_error;
use crate::protocols::dsp::orchestrator::OrchestratorTrait;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::errors::CommonErrors;
use rainbow_common::protocol::context_field::ContextField;

#[derive(Clone)]
pub struct DspRouter {
    orchestrator: Arc<dyn OrchestratorTrait>,
    config: Arc<ApplicationProviderConfig>,
}

impl FromRef<DspRouter> for Arc<dyn OrchestratorTrait> {
    fn from_ref(state: &DspRouter) -> Self {
        state.orchestrator.clone()
    }
}

impl FromRef<DspRouter> for Arc<ApplicationProviderConfig> {
    fn from_ref(state: &DspRouter) -> Self {
        state.config.clone()
    }
}

impl DspRouter {
    pub fn new(service: Arc<dyn OrchestratorTrait>, config: Arc<ApplicationProviderConfig>) -> Self {
        Self { orchestrator: service, config }
    }

    pub fn router(self) -> Router {
        Router::new().with_state(self)
    }
}
