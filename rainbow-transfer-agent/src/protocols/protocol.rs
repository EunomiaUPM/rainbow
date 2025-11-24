use crate::TransferDummyTrait;
use std::sync::Arc;

#[allow(unused)]
pub struct TransferSharedServices {
    // Repo
    pub transfer_process_repo: Arc<dyn TransferDummyTrait>,
    pub transfer_identifiers_repo: Arc<dyn TransferDummyTrait>,
    pub transfer_message_repo: Arc<dyn TransferDummyTrait>,

    pub auth_facade: Arc<dyn TransferDummyTrait>,
    pub dataplane_facade: Arc<dyn TransferDummyTrait>,
    pub notifier: Arc<dyn TransferDummyTrait>,
    pub agreement_facade: Arc<dyn TransferDummyTrait>,
    pub data_service_facade: Arc<dyn TransferDummyTrait>,
}

#[async_trait::async_trait]
#[allow(unused)]
pub trait ProtocolPluginTrait {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn short_name(&self) -> &'static str;
    fn build_router(&self) -> anyhow::Result<axum::Router>;
    fn build_grpc_router(&self) -> anyhow::Result<Option<axum::Router>>;
}
