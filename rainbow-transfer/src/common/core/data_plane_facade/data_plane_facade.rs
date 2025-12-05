use crate::common::core::data_plane_facade::dataplane_strategy_factory::DataPlaneStrategyFactory;
use crate::common::core::data_plane_facade::DataPlaneFacadeTrait;
use crate::provider::core::rainbow_entities::RainbowTransferProviderServiceTrait;
use axum::async_trait;
use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::protocol::transfer::transfer_data_address::DataAddress;
use rainbow_common::protocol::transfer::TransferRoles;
use std::ops::Deref;
use std::sync::Arc;
use urn::Urn;

pub struct DataPlaneProviderFacadeForDSProtocol {
    dataplane_strategy_factory: Arc<DataPlaneStrategyFactory>,
    transfer_process_repo: Arc<dyn RainbowTransferProviderServiceTrait>
}
impl DataPlaneProviderFacadeForDSProtocol {
    pub fn new(
        dataplane_strategy_factory: Arc<DataPlaneStrategyFactory>,
        transfer_process_service: Arc<dyn RainbowTransferProviderServiceTrait>
    ) -> Self {
        Self { dataplane_strategy_factory, transfer_process_service }
    }
}

#[async_trait]
impl DataPlaneFacadeTrait for DataPlaneProviderFacadeForDSProtocol {
    async fn get_dataplane_address(&self, session_id: Urn) -> anyhow::Result<DataAddress> {
        todo!()
    }

    async fn on_transfer_request_pre(&self, session_id: Urn, format: DctFormats, data_service: DataService, data_address: Option<DataAddress>) -> anyhow::Result<()> {
        let strategy = self.dataplane_strategy_factory.get_strategy(&TransferRoles::Provider, &format);
        strategy.on_transfer_request_pre(session_id, format, data_service, data_address).await?;
        Ok(())
    }

    async fn on_transfer_request_post(&self, session_id: Urn, format: DctFormats, data_service: DataService, data_address: Option<DataAddress>) -> anyhow::Result<()> {
        let strategy = self.dataplane_strategy_factory.get_strategy(&TransferRoles::Provider, &format);
        strategy.on_transfer_request_post(session_id, format, data_service, data_address).await?;
        Ok(())
    }

    async fn on_transfer_start_pre(&self, session_id: Urn) -> anyhow::Result<()> {
        let process = self.transfer_process_service
            .get_transfer_by_id(session_id.clone())
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        let format = process.format;
        let strategy = self.dataplane_strategy_factory.get_strategy(&TransferRoles::Provider, &format);
        strategy.on_transfer_start_pre(session_id).await?;
        Ok(())
    }

    async fn on_transfer_start_post(&self, session_id: Urn) -> anyhow::Result<()> {
        let strategy = self.dataplane_strategy_factory.get_strategy(&TransferRoles::Provider, &format);
        strategy.deref().on_transfer_start_post(session_id).await?;
        Ok(())
    }

    async fn on_transfer_suspension_pre(&self, session_id: Urn) -> anyhow::Result<()> {
        let strategy = self.dataplane_strategy_factory.get_strategy(&TransferRoles::Provider, &format);
        strategy.deref().on_transfer_suspension_pre(session_id).await?;
        Ok(())
    }

    async fn on_transfer_suspension_post(&self, session_id: Urn) -> anyhow::Result<()> {
        let strategy = self.dataplane_strategy_factory.get_strategy(&TransferRoles::Provider, &format);
        strategy.deref().on_transfer_suspension_post(session_id).await?;
        Ok(())
    }

    async fn on_transfer_completion_pre(&self, session_id: Urn) -> anyhow::Result<()> {
        let strategy = self.dataplane_strategy_factory.get_strategy(&TransferRoles::Provider, &format);
        strategy.deref().on_transfer_completion_pre(session_id).await?;
        Ok(())
    }

    async fn on_transfer_completion_post(&self, session_id: Urn) -> anyhow::Result<()> {
        let strategy = self.dataplane_strategy_factory.get_strategy(&TransferRoles::Provider, &format);
        strategy.deref().on_transfer_completion_post(session_id).await?;
        Ok(())
    }

    async fn on_transfer_termination_pre(&self, session_id: Urn) -> anyhow::Result<()> {
        let strategy = self.dataplane_strategy_factory.get_strategy(&TransferRoles::Provider, &format);
        strategy.deref().on_transfer_termination_pre(session_id).await?;
        Ok(())
    }

    async fn on_transfer_termination_post(&self, session_id: Urn) -> anyhow::Result<()> {
        let strategy = self.dataplane_strategy_factory.get_strategy(&TransferRoles::Provider, &format);
        strategy.deref().on_transfer_termination_post(session_id).await?;
        Ok(())
    }
}
