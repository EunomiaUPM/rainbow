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

use crate::protocols::dsp::orchestrator::rpc::RPCOrchestratorTrait;
use crate::protocols::dsp::persistence::NegotiationPersistenceTrait;
use crate::protocols::dsp::validator::traits::validation_rpc_steps::ValidationRpcSteps;
use rainbow_common::config::provider_config::ApplicationProviderConfig;
use rainbow_common::http_client::HttpClient;
use std::sync::Arc;

#[allow(unused)]
pub struct RPCOrchestratorService {
    validator: Arc<dyn ValidationRpcSteps>,
    pub persistence_service: Arc<dyn NegotiationPersistenceTrait>,
    pub _config: Arc<ApplicationProviderConfig>,
    pub http_client: Arc<HttpClient>,
}

impl RPCOrchestratorService {
    pub fn new(
        validator: Arc<dyn ValidationRpcSteps>,
        persistence_service: Arc<dyn NegotiationPersistenceTrait>,
        _config: Arc<ApplicationProviderConfig>,
        http_client: Arc<HttpClient>,
    ) -> RPCOrchestratorService {
        RPCOrchestratorService { validator, persistence_service, _config, http_client }
    }
}

#[async_trait::async_trait]
impl RPCOrchestratorTrait for RPCOrchestratorService {}
