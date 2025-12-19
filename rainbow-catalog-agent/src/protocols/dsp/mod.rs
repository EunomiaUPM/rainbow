use crate::entities::catalogs::CatalogEntityTrait;
use crate::entities::data_services::DataServiceEntityTrait;
use crate::entities::datasets::DatasetEntityTrait;
use crate::entities::distributions::DistributionEntityTrait;
use crate::entities::odrl_policies::OdrlPolicyEntityTrait;
use crate::protocols::dsp::facades::FacadeService;
use crate::protocols::dsp::http::protocol::DspRouter;
use crate::protocols::dsp::orchestrator::orchestrator::OrchestratorService;
use crate::protocols::dsp::orchestrator::protocol::persistence::OrchestrationPersistenceForProtocol;
use crate::protocols::dsp::orchestrator::protocol::protocol::ProtocolOrchestratorService;
use crate::protocols::dsp::validator::validators::protocol::validation_dsp_steps::ValidationDspStepsService;
use crate::protocols::dsp::validator::validators::validate_payload::ValidatePayloadService;
use crate::protocols::dsp::validator::validators::validation_helpers::ValidationHelperService;
use crate::protocols::protocol::ProtocolPluginTrait;
use axum::Router;
use rainbow_common::config::services::CatalogConfig;
use rainbow_common::facades::ssi_auth_facade::MatesFacadeTrait;
use std::sync::Arc;

mod errors;
pub(crate) mod facades;
pub(crate) mod http;
pub(crate) mod orchestrator;
pub(crate) mod protocol_types;
pub(crate) mod validator;

pub struct CatalogDSP {
    pub catalog_entities_service: Arc<dyn CatalogEntityTrait>,
    pub data_service_entities_service: Arc<dyn DataServiceEntityTrait>,
    pub dataset_entities_service: Arc<dyn DatasetEntityTrait>,
    pub odrl_policies_service: Arc<dyn OdrlPolicyEntityTrait>,
    pub distributions_entity_service: Arc<dyn DistributionEntityTrait>,
    pub mates_facade: Arc<dyn MatesFacadeTrait>,
    _config: Arc<CatalogConfig>,
}

impl CatalogDSP {
    pub fn new(
        catalog_entities_service: Arc<dyn CatalogEntityTrait>,
        data_service_entities_service: Arc<dyn DataServiceEntityTrait>,
        dataset_entities_service: Arc<dyn DatasetEntityTrait>,
        odrl_policies_service: Arc<dyn OdrlPolicyEntityTrait>,
        distributions_entity_service: Arc<dyn DistributionEntityTrait>,
        mates_facade: Arc<dyn MatesFacadeTrait>,
        config: Arc<CatalogConfig>,
    ) -> Self {
        Self {
            catalog_entities_service,
            data_service_entities_service,
            dataset_entities_service,
            odrl_policies_service,
            distributions_entity_service,
            mates_facade,
            _config: config,
        }
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
        // Validator
        let validator_helper = Arc::new(ValidationHelperService::new());
        let validator_payload = Arc::new(ValidatePayloadService::new(validator_helper.clone()));
        let dsp_validator = Arc::new(ValidationDspStepsService::new(
            validator_payload.clone(),
            validator_helper.clone(),
        ));

        // facades
        let facades = Arc::new(FacadeService::new());

        // persistence
        let persistence = Arc::new(OrchestrationPersistenceForProtocol::new(
            self.catalog_entities_service.clone(),
            self.data_service_entities_service.clone(),
            self.dataset_entities_service.clone(),
            self.odrl_policies_service.clone(),
            self.distributions_entity_service.clone(),
            self.mates_facade.clone(),
        ));

        // orchestrators
        let http_orchestator = Arc::new(ProtocolOrchestratorService::new(
            dsp_validator.clone(),
            facades.clone(),
            persistence.clone(),
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
