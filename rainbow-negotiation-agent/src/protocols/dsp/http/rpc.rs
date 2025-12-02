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

use axum::{Router, extract::FromRef};
use std::sync::Arc;

use crate::protocols::dsp::orchestrator::OrchestratorTrait;
use rainbow_common::config::provider_config::ApplicationProviderConfig;

#[derive(Clone)]
pub struct RpcRouter {
    orchestrator: Arc<dyn OrchestratorTrait>,
    config: Arc<ApplicationProviderConfig>,
}

impl FromRef<RpcRouter> for Arc<dyn OrchestratorTrait> {
    fn from_ref(state: &RpcRouter) -> Self {
        state.orchestrator.clone()
    }
}

impl FromRef<RpcRouter> for Arc<ApplicationProviderConfig> {
    fn from_ref(state: &RpcRouter) -> Self {
        state.config.clone()
    }
}

impl RpcRouter {
    pub fn new(service: Arc<dyn OrchestratorTrait>, config: Arc<ApplicationProviderConfig>) -> Self {
        Self { orchestrator: service, config }
    }

    pub fn router(self) -> Router {
        Router::new().with_state(self)
    }
}
