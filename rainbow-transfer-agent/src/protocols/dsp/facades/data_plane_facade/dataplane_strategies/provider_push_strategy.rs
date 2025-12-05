use crate::protocols::dsp::facades::data_plane_facade::dataplane_strategies::DataPlaneStrategyTrait;
use crate::protocols::dsp::facades::data_plane_facade::DataPlaneFacadeTrait;
use crate::protocols::dsp::protocol_types::DataAddressDto;
use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::protocol::transfer::transfer_data_address::DataAddress;
use rainbow_dataplane::coordinator::dataplane_access_controller::DataPlaneAccessControllerTrait;
use std::sync::Arc;
use urn::Urn;

pub struct ProviderPushDataplaneStrategy {
    dataplane_controller_access: Arc<dyn DataPlaneAccessControllerTrait>,
}

impl ProviderPushDataplaneStrategy {
    pub fn new(dataplane_controller_access: Arc<dyn DataPlaneAccessControllerTrait>) -> Self {
        Self { dataplane_controller_access }
    }
}

#[async_trait::async_trait]
impl DataPlaneStrategyTrait for ProviderPushDataplaneStrategy {}

#[async_trait::async_trait]
impl DataPlaneFacadeTrait for ProviderPushDataplaneStrategy {
    async fn get_dataplane_address(&self, session_id: &Urn) -> anyhow::Result<DataAddress> {
        todo!()
    }

    async fn on_transfer_request_pre(
        &self,
        session_id: &Urn,
        format: &DctFormats,
        data_service: &Option<DataService>,
        data_address: &Option<DataAddressDto>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_request_post(
        &self,
        session_id: &Urn,
        format: &DctFormats,
        data_service: &Option<DataService>,
        data_address: &Option<DataAddressDto>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_start_pre(&self, session_id: &Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_start_post(&self, session_id: &Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_suspension_pre(&self, session_id: &Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_suspension_post(&self, session_id: &Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_completion_pre(&self, session_id: &Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_completion_post(&self, session_id: &Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_termination_pre(&self, session_id: &Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_termination_post(&self, session_id: &Urn) -> anyhow::Result<()> {
        todo!()
    }
}
