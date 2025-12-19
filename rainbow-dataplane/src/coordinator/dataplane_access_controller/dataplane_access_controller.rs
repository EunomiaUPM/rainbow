use crate::coordinator::data_source_connector::DataSourceConnectorTrait;
use crate::coordinator::dataplane_access_controller::DataPlaneAccessControllerTrait;
use crate::entities::data_plane_process::{
    DataPlaneProcessEntitiesTrait, EditDataPlaneProcessDto, NewDataPlaneProcessDto,
};
use rainbow_common::adv_protocol::interplane::data_plane_provision::{
    DataPlaneProvisionRequest, DataPlaneProvisionResponse,
};
use rainbow_common::adv_protocol::interplane::data_plane_start::{DataPlaneStart, DataPlaneStartAck};
use rainbow_common::adv_protocol::interplane::data_plane_status::{DataPlaneStatusRequest, DataPlaneStatusResponse};
use rainbow_common::adv_protocol::interplane::data_plane_stop::{DataPlaneStop, DataPlaneStopAck};
use rainbow_common::adv_protocol::interplane::{
    DataPlaneControllerMessages, DataPlaneControllerVersion, DataPlaneProcessDirection, DataPlaneProcessState,
    DataPlaneSDPConfigTypes, DataPlaneSDPFieldTypes, DataPlaneSDPResponseField,
};
use rainbow_common::config::services::TransferConfig;
use rainbow_common::config::traits::HostConfigTrait;
use rainbow_common::config::types::HostType;
use rainbow_common::dcat_formats::FormatAction;
use std::collections::HashMap;
use std::sync::Arc;

pub struct DataPlaneAccessControllerService {
    data_source_connector_service: Arc<dyn DataSourceConnectorTrait>,
    dataplane_process_entity: Arc<dyn DataPlaneProcessEntitiesTrait>,
    config: Arc<TransferConfig>,
}

impl DataPlaneAccessControllerService {
    pub fn new(
        data_source_connector_service: Arc<dyn DataSourceConnectorTrait>,
        dataplane_process_entity: Arc<dyn DataPlaneProcessEntitiesTrait>,
        config: Arc<TransferConfig>,
    ) -> Self {
        Self { data_source_connector_service, dataplane_process_entity, config }
    }
}

#[async_trait::async_trait]
impl DataPlaneAccessControllerTrait for DataPlaneAccessControllerService {
    async fn data_plane_provision_request(
        &self,
        input: &DataPlaneProvisionRequest,
    ) -> anyhow::Result<DataPlaneProvisionResponse> {
        let process_address = self.config.get_host(HostType::Http);
        let sdp_config = input.sdp_config.as_ref().unwrap();
        let next_hop_protocol = sdp_config
            .iter()
            .find(|s| s._type == DataPlaneSDPConfigTypes::NextHopAddressScheme)
            .expect("DataPlaneSDPConfigTypes::NextHopAddressScheme must be defined");
        let next_hop_address = sdp_config
            .iter()
            .find(|s| s._type == DataPlaneSDPConfigTypes::NextHopAddress)
            .expect("DataPlaneSDPConfigTypes::NextHopAddress must be defined");
        let next_hop_direction = sdp_config
            .iter()
            .find(|s| s._type == DataPlaneSDPConfigTypes::Direction)
            .expect("DataPlaneSDPConfigTypes::Direction must be defined");
        let next_hop_direction_as = next_hop_direction.content.parse::<FormatAction>()?;

        let data_plane_url = format!("{}/data/{}", process_address, input.session_id.clone());

        let mut dataplane_fields: HashMap<String, String> = HashMap::new();
        dataplane_fields.insert(String::from("ProcessAddressProtocol"), "".to_string());
        dataplane_fields.insert(String::from("ProcessAddressUrl"), data_plane_url);
        dataplane_fields.insert(String::from("ProcessAddressAuth"), "".to_string());
        dataplane_fields.insert(String::from("ProcessAddressAuthContent"), "".to_string());
        dataplane_fields.insert(
            String::from("DownstreamHopAddressProtocol"),
            next_hop_protocol.content.to_string(),
        );
        dataplane_fields.insert(
            String::from("DownstreamHopAddressUrl"),
            next_hop_address.content.to_string(),
        );
        dataplane_fields.insert(String::from("DownstreamHopAddressAuth"), "".to_string());
        dataplane_fields.insert(
            String::from("DownstreamHopAddressAuthContent"),
            "".to_string(),
        );
        dataplane_fields.insert(String::from("UpstreamHopAddressProtocol"), "".to_string());
        dataplane_fields.insert(String::from("UpstreamHopAddressUrl"), "".to_string());
        dataplane_fields.insert(String::from("UpstreamHopAddressAuth"), "".to_string());
        dataplane_fields.insert(
            String::from("UpstreamHopAddressAuthContent"),
            "".to_string(),
        );
        let dataplane_response = self
            .dataplane_process_entity
            .create_data_plane_process(&NewDataPlaneProcessDto {
                id: input.session_id.clone(),
                direction: next_hop_direction_as.to_string(),
                state: DataPlaneProcessState::REQUESTED.to_string(),
                fields: Some(dataplane_fields),
            })
            .await?;

        Ok(DataPlaneProvisionResponse {
            _type: DataPlaneControllerMessages::DataPlaneProvisionResponse,
            version: DataPlaneControllerVersion::Version10,
            session_id: input.session_id.clone(),
            sdp_response: vec![
                DataPlaneSDPResponseField {
                    _type: DataPlaneSDPFieldTypes::DataPlaneAddressScheme,
                    format: "https://www.iana.org/assignments/uri-schemes/uri-schemes.xhtml".to_string(),
                    content: dataplane_response.data_plane_fields.get("ProcessAddressProtocol").unwrap().to_string(),
                },
                DataPlaneSDPResponseField {
                    _type: DataPlaneSDPFieldTypes::DataPlaneAddress,
                    format: "uri".to_string(),
                    content: dataplane_response.data_plane_fields.get("ProcessAddressUrl").unwrap().to_string(),
                },
                DataPlaneSDPResponseField {
                    _type: DataPlaneSDPFieldTypes::DataPlaneAddressAuthType,
                    format: "https://www.iana.org/assignments/http-authschemes/http-authschemes.xhtml".to_string(),
                    content: dataplane_response.data_plane_fields.get("ProcessAddressAuth").unwrap().to_string(),
                },
                DataPlaneSDPResponseField {
                    _type: DataPlaneSDPFieldTypes::DataPlaneAddressAuthToken,
                    format: "jwt".to_string(),
                    content: dataplane_response.data_plane_fields.get("ProcessAddressAuthContent").unwrap().to_string(),
                },
            ],
            sdp_request: None,
            sdp_config: None,
        })
    }

    async fn data_plane_start(&self, input: &DataPlaneStart) -> anyhow::Result<DataPlaneStartAck> {
        let dp_process = self
            .dataplane_process_entity
            .put_data_plane_process(
                &input.session_id,
                &EditDataPlaneProcessDto { state: Some(DataPlaneProcessState::STARTED.to_string()), fields: None },
            )
            .await?;
        Ok(DataPlaneStartAck {
            _type: DataPlaneControllerMessages::DataPlaneStartAck,
            version: DataPlaneControllerVersion::Version10,
            session_id: input.session_id.clone(),
        })
    }

    async fn data_plane_stop(&self, input: &DataPlaneStop) -> anyhow::Result<DataPlaneStopAck> {
        let dp_process = self
            .dataplane_process_entity
            .put_data_plane_process(
                &input.session_id,
                &EditDataPlaneProcessDto { state: Some(DataPlaneProcessState::STOPPED.to_string()), fields: None },
            )
            .await?;
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
