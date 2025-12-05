use crate::coordinator::data_source_connector::DataSourceConnectorTrait;
use std::sync::Arc;
use rainbow_common::adv_protocol::interplane::data_plane_provision::{DataPlaneProvisionRequest, DataPlaneProvisionResponse};
use rainbow_common::adv_protocol::interplane::data_plane_start::{DataPlaneStart, DataPlaneStartAck};
use rainbow_common::adv_protocol::interplane::data_plane_status::{DataPlaneStatusRequest, DataPlaneStatusResponse};
use rainbow_common::adv_protocol::interplane::data_plane_stop::{DataPlaneStop, DataPlaneStopAck};
use rainbow_common::adv_protocol::interplane::{DataPlaneControllerMessages, DataPlaneControllerVersion, DataPlaneProcessDirection, DataPlaneProcessState};
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use crate::coordinator::dataplane_access_controller::DataPlaneAccessControllerTrait;
use crate::entities::data_plane_process::{DataPlaneProcessEntitiesTrait, EditDataPlaneProcessDto, NewDataPlaneProcessDto};

pub struct DataPlaneAccessControllerService {
    data_source_connector_service: Arc<dyn DataSourceConnectorTrait>,
    dataplane_process_entity: Arc<dyn DataPlaneProcessEntitiesTrait>,
    config: Arc<ApplicationGlobalConfig>
}

impl DataPlaneAccessControllerService {
    pub fn new(
        data_source_connector_service: Arc<dyn DataSourceConnectorTrait>,
        dataplane_process_entity: Arc<dyn DataPlaneProcessEntitiesTrait>,
        config: Arc<ApplicationGlobalConfig>
    ) -> Self {
        Self { data_source_connector_service, dataplane_process_entity, config }
    }
}

#[async_trait::async_trait]
impl DataPlaneAccessControllerTrait for DataPlaneAccessControllerService {
    async fn data_plane_provision_request(&self, input: &DataPlaneProvisionRequest) -> anyhow::Result<DataPlaneProvisionResponse> {
        let dp_process = self.dataplane_process_entity.create_data_plane_process(&NewDataPlaneProcessDto {
            id: input.session_id.clone(),
            direction: DataPlaneProcessDirection::PULL.to_string(),
            state: DataPlaneProcessState::REQUESTED.to_string(),
            fields: None,
        }).await?;
        Ok(DataPlaneProvisionResponse {
            _type: DataPlaneControllerMessages::DataPlaneProvisionResponse,
            version: DataPlaneControllerVersion::Version10,
            session_id: input.session_id.clone(),
            sdp_response: vec![],
            sdp_request: None,
            sdp_config: None,
        })
    }

    async fn data_plane_start(&self, input: &DataPlaneStart) -> anyhow::Result<DataPlaneStartAck> {
        let dp_process = self.dataplane_process_entity
            .put_data_plane_process(&input.session_id, &EditDataPlaneProcessDto {
                state: Some(DataPlaneProcessState::STARTED.to_string()),
                fields: None
            }).await?;
        Ok(DataPlaneStartAck {
            _type: DataPlaneControllerMessages::DataPlaneStartAck,
            version: DataPlaneControllerVersion::Version10,
            session_id: input.session_id.clone(),
        })
    }

    async fn data_plane_stop(&self, input: &DataPlaneStop) -> anyhow::Result<DataPlaneStopAck> {
        let dp_process = self.dataplane_process_entity
            .put_data_plane_process(&input.session_id, &EditDataPlaneProcessDto {
                state: Some(DataPlaneProcessState::STOPPED.to_string()),
                fields: None
            }).await?;
        Ok(DataPlaneStopAck {
            _type: DataPlaneControllerMessages::DataPlaneStopAck,
            version: DataPlaneControllerVersion::Version10,
            session_id: input.session_id.clone(),
        })
    }

    async fn data_plane_get_status(&self, input: &DataPlaneStatusRequest) -> anyhow::Result<DataPlaneStatusResponse> {
        Ok(DataPlaneStatusResponse {
            _type: DataPlaneControllerMessages::DataPlaneStatusResponse,
            version: DataPlaneControllerVersion::Version10,
            session_id: input.session_id.clone(),
            sdp_response: vec![],
        })
    }
}
