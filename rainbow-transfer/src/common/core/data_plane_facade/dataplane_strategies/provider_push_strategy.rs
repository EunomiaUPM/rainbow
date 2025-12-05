use crate::common::core::data_plane_facade::dataplane_strategies::DataPlaneStrategyTrait;
use crate::common::core::data_plane_facade::DataPlaneFacadeTrait;
use axum::async_trait;
use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::protocol::transfer::transfer_data_address::DataAddress;
use urn::Urn;

pub struct ProviderPushDataplaneStrategy;

#[async_trait]
impl DataPlaneStrategyTrait for ProviderPushDataplaneStrategy {}

#[async_trait]
impl DataPlaneFacadeTrait for ProviderPushDataplaneStrategy {
    async fn get_dataplane_address(&self, session_id: Urn) -> anyhow::Result<DataAddress> {
        todo!()
    }

    async fn on_transfer_request_pre(
        &self,
        session_id: Urn,
        format: DctFormats,
        data_service: DataService,
        data_address: Option<DataAddress>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_request_post(
        &self,
        session_id: Urn,
        format: DctFormats,
        data_service: DataService,
        data_address: Option<DataAddress>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_start_pre(&self, session_id: Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_start_post(&self, session_id: Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_suspension_pre(&self, session_id: Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_suspension_post(&self, session_id: Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_completion_pre(&self, session_id: Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_completion_post(&self, session_id: Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_termination_pre(&self, session_id: Urn) -> anyhow::Result<()> {
        todo!()
    }

    async fn on_transfer_termination_post(&self, session_id: Urn) -> anyhow::Result<()> {
        todo!()
    }
}
