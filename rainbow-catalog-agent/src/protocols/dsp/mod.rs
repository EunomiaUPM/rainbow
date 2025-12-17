use crate::protocols::dsp::facades::FacadeService;
use crate::protocols::dsp::http::protocol::DspRouter;
use crate::protocols::dsp::orchestrator::orchestrator::OrchestratorService;
use crate::protocols::dsp::orchestrator::protocol::protocol::ProtocolOrchestratorService;
use crate::protocols::dsp::validator::validators::protocol::validation_dsp_steps::ValidationDspStepsService;
use crate::protocols::dsp::validator::validators::validate_payload::ValidatePayloadService;
use crate::protocols::dsp::validator::validators::validation_helpers::ValidationHelperService;
use crate::protocols::protocol::ProtocolPluginTrait;
use axum::Router;
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use rainbow_common::http_client::HttpClient;
use std::sync::Arc;

mod errors;
pub(crate) mod facades;
pub(crate) mod http;
pub(crate) mod orchestrator;
pub(crate) mod protocol_types;
pub(crate) mod validator;

pub struct CatalogDSP {
    config: Arc<ApplicationGlobalConfig>,
}

impl CatalogDSP {
    pub fn new(config: Arc<ApplicationGlobalConfig>) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl ProtocolPluginTrait for CatalogDSP {
    fn name(&self) -> &'static str {
        "Dataspace Protocol"
    }

    fn version(&self) -> &'static str {
        "1.0"
    }

    fn short_name(&self) -> &'static str {
        "DSP"
    }

    async fn build_router(&self) -> anyhow::Result<Router> {
        let http_client = Arc::new(HttpClient::new(10, 10));

        // Validator
        let validator_helper = Arc::new(ValidationHelperService::new());
        let validator_payload = Arc::new(ValidatePayloadService::new(validator_helper.clone()));
        let dsp_validator = Arc::new(ValidationDspStepsService::new(
            validator_payload.clone(),
            validator_helper.clone(),
        ));

        // persistence

        // facades
        let facades = Arc::new(FacadeService::new());

        // orchestrators
        let http_orchestator = Arc::new(ProtocolOrchestratorService::new(
            dsp_validator.clone(),
            facades.clone(),
        ));

        let orchestrator_service = Arc::new(OrchestratorService::new(http_orchestator.clone()));

        // router
        let dsp_router = DspRouter::new(orchestrator_service.clone());

        Ok(Router::new().merge(dsp_router.router()))
    }

    fn build_grpc_router(&self) -> anyhow::Result<Option<Router>> {
        todo!()
    }
}
