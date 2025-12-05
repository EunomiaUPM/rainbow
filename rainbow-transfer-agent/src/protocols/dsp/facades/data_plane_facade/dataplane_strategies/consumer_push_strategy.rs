use crate::protocols::dsp::facades::data_plane_facade::dataplane_strategies::consumer_pull_strategy::ConsumerPullDataplaneStrategy;
use crate::protocols::dsp::facades::data_plane_facade::dataplane_strategies::DataPlaneStrategyTrait;
use crate::protocols::dsp::facades::data_plane_facade::DataPlaneFacadeTrait;
use crate::protocols::dsp::protocol_types::DataAddressDto;
use rainbow_common::adv_protocol::interplane::data_plane_provision::DataPlaneProvisionRequest;
use rainbow_common::adv_protocol::interplane::data_plane_start::DataPlaneStart;
use rainbow_common::adv_protocol::interplane::data_plane_stop::DataPlaneStop;
use rainbow_common::adv_protocol::interplane::{DataPlaneControllerMessages, DataPlaneControllerVersion};
use rainbow_common::dcat_formats::DctFormats;
use rainbow_common::protocol::catalog::dataservice_definition::DataService;
use rainbow_common::protocol::transfer::transfer_data_address::DataAddress;
use rainbow_dataplane::coordinator::dataplane_access_controller::DataPlaneAccessControllerTrait;
use std::sync::Arc;
use urn::Urn;

pub struct ConsumerPushDataplaneStrategy {
    dataplane_controller_access: Arc<dyn DataPlaneAccessControllerTrait>,
}

impl ConsumerPushDataplaneStrategy {
    pub fn new(dataplane_controller_access: Arc<dyn DataPlaneAccessControllerTrait>) -> Self {
        Self { dataplane_controller_access }
    }
}

#[async_trait::async_trait]
impl DataPlaneStrategyTrait for ConsumerPushDataplaneStrategy {}

#[async_trait::async_trait]
impl DataPlaneFacadeTrait for ConsumerPushDataplaneStrategy {
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
        Ok(())
    }

    async fn on_transfer_request_post(
        &self,
        session_id: &Urn,
        format: &DctFormats,
        data_service: &Option<DataService>,
        data_address: &Option<DataAddressDto>,
    ) -> anyhow::Result<()> {
        let provision_request = self
            .dataplane_controller_access
            .data_plane_provision_request(&DataPlaneProvisionRequest {
                _type: DataPlaneControllerMessages::DataPlaneProvisionRequest,
                version: DataPlaneControllerVersion::Version10,
                session_id: session_id.clone(),
                sdp_request: vec![],
                sdp_config: None,
            })
            .await?;
        Ok(())
    }

    async fn on_transfer_start_pre(&self, session_id: &Urn) -> anyhow::Result<()> {
        self.dataplane_controller_access
            .data_plane_start(&DataPlaneStart {
                _type: DataPlaneControllerMessages::DataPlaneStart,
                version: DataPlaneControllerVersion::Version10,
                session_id: session_id.clone(),
            })
            .await?;
        Ok(())
    }

    async fn on_transfer_start_post(&self, session_id: &Urn) -> anyhow::Result<()> {
        Ok(())
    }

    async fn on_transfer_suspension_pre(&self, session_id: &Urn) -> anyhow::Result<()> {
        self.dataplane_controller_access
            .data_plane_stop(&DataPlaneStop {
                _type: DataPlaneControllerMessages::DataPlaneStop,
                version: DataPlaneControllerVersion::Version10,
                session_id: session_id.clone(),
            })
            .await?;
        Ok(())
    }

    async fn on_transfer_suspension_post(&self, session_id: &Urn) -> anyhow::Result<()> {
        Ok(())
    }

    async fn on_transfer_completion_pre(&self, session_id: &Urn) -> anyhow::Result<()> {
        self.dataplane_controller_access
            .data_plane_stop(&DataPlaneStop {
                _type: DataPlaneControllerMessages::DataPlaneStop,
                version: DataPlaneControllerVersion::Version10,
                session_id: session_id.clone(),
            })
            .await?;
        Ok(())
    }

    async fn on_transfer_completion_post(&self, session_id: &Urn) -> anyhow::Result<()> {
        Ok(())
    }

    async fn on_transfer_termination_pre(&self, session_id: &Urn) -> anyhow::Result<()> {
        self.dataplane_controller_access
            .data_plane_stop(&DataPlaneStop {
                _type: DataPlaneControllerMessages::DataPlaneStop,
                version: DataPlaneControllerVersion::Version10,
                session_id: session_id.clone(),
            })
            .await?;
        Ok(())
    }

    async fn on_transfer_termination_post(&self, session_id: &Urn) -> anyhow::Result<()> {
        Ok(())
    }
}
