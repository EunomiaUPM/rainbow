use crate::coordinator::controller::DataPlaneControllerTrait;
use crate::coordinator::dataplane_process::dataplane_process::DataPlaneProcess;
use crate::coordinator::dataplane_process::{DataPlaneDefaultBehaviour, DataPlaneProcessAddress, DataPlaneProcessRequest, DataPlaneProcessTrait};
use axum::async_trait;
use rainbow_common::adv_protocol::interplane::data_plane_provision::{DataPlaneProvisionRequest, DataPlaneProvisionResponse};
use rainbow_common::adv_protocol::interplane::data_plane_start::{DataPlaneStart, DataPlaneStartAck};
use rainbow_common::adv_protocol::interplane::data_plane_status::{DataPlaneStatusRequest, DataPlaneStatusResponse};
use rainbow_common::adv_protocol::interplane::data_plane_stop::{DataPlaneStop, DataPlaneStopAck};
use rainbow_common::adv_protocol::interplane::{DataPlaneControllerMessages, DataPlaneControllerVersion, DataPlaneProcessDirection, DataPlaneProcessState, DataPlaneSDPConfigTypes, DataPlaneSDPFieldTypes, DataPlaneSDPResponseField};
use rainbow_common::config::global_config::ApplicationGlobalConfig;
use rainbow_common::dcat_formats::FormatAction;
use rainbow_db::dataplane::repo::{DataPlaneFieldRepo, DataPlaneProcessRepo};
use std::sync::Arc;
use tracing::debug;

pub struct DataPlaneControllerService<T>
where
    T: DataPlaneProcessTrait + Send + Sync,
{
    config: Arc<ApplicationGlobalConfig>,
    dataplane_process_service: Arc<T>,
}

impl<T> DataPlaneControllerService<T>
where
    T: DataPlaneProcessTrait + Send + Sync,
{
    pub fn new(config: Arc<ApplicationGlobalConfig>, dataplane_process_service: Arc<T>) -> Self {
        Self {
            config,
            dataplane_process_service,
        }
    }
}

#[async_trait]
impl<T> DataPlaneControllerTrait for DataPlaneControllerService<T>
where
    T: DataPlaneProcessTrait + Send + Sync,
{
    async fn data_plane_provision_request(&self, input: DataPlaneProvisionRequest) -> anyhow::Result<DataPlaneProvisionResponse> {
        debug!("DataPlaneControllerService -> data_plane_provision_request");
        let process_address = self.config.transfer_process_host.clone().unwrap();
        let sdp_config = input.sdp_config.unwrap();;
        let next_hop_protocol = sdp_config.iter()
            .find(|s| s._type == DataPlaneSDPConfigTypes::NextHopAddressScheme)
            .expect("DataPlaneSDPConfigTypes::NextHopAddressScheme must be defined");
        let next_hop_address = sdp_config.iter()
            .find(|s| s._type == DataPlaneSDPConfigTypes::NextHopAddress)
            .expect("DataPlaneSDPConfigTypes::NextHopAddress must be defined");
        let next_hop_direction = sdp_config.iter()
            .find(|s| s._type == DataPlaneSDPConfigTypes::Direction)
            .expect("DataPlaneSDPConfigTypes::Direction must be defined");
        let next_hop_direction_as = next_hop_direction.content.parse::<FormatAction>()?;

        let data_plane_url = format!(
            "{}://{}:{}/data/{}",
            process_address.protocol,
            process_address.url,
            process_address.port,
            input.session_id.clone()
        );
        let data_plane_process = DataPlaneProcess::create_dataplane_process(DataPlaneProcessRequest {
            session_id: input.session_id.clone(),
            process_address: DataPlaneProcessAddress {
                protocol: process_address.protocol.clone(),
                url: data_plane_url,
                auth_type: "".to_string(),
                auth_content: "".to_string(),
            },
            downstream_hop: DataPlaneProcessAddress {
                protocol: next_hop_protocol.content.clone(),
                url: next_hop_address.content.clone(),
                auth_type: "".to_string(),
                auth_content: "".to_string(),
            },
            process_direction: DataPlaneProcessDirection::from(next_hop_direction_as),
        }).await?;

        let data_plane_process = self.dataplane_process_service.create_dataplane_process(data_plane_process).await?;

        Ok(DataPlaneProvisionResponse {
            _type: DataPlaneControllerMessages::DataPlaneProvisionResponse,
            version: DataPlaneControllerVersion::Version10,
            session_id: input.session_id,
            sdp_response: vec![
                DataPlaneSDPResponseField {
                    _type: DataPlaneSDPFieldTypes::DataPlaneAddressScheme,
                    format: "https://www.iana.org/assignments/uri-schemes/uri-schemes.xhtml".to_string(),
                    content: data_plane_process.process_address.protocol,
                },
                DataPlaneSDPResponseField {
                    _type: DataPlaneSDPFieldTypes::DataPlaneAddress,
                    format: "uri".to_string(),
                    content: data_plane_process.process_address.url,
                },
                DataPlaneSDPResponseField {
                    _type: DataPlaneSDPFieldTypes::DataPlaneAddressAuthType,
                    format: "https://www.iana.org/assignments/http-authschemes/http-authschemes.xhtml".to_string(),
                    content: data_plane_process.process_address.auth_type,
                },
                DataPlaneSDPResponseField {
                    _type: DataPlaneSDPFieldTypes::DataPlaneAddressAuthToken,
                    format: "jwt".to_string(),
                    content: data_plane_process.process_address.auth_content,
                }
            ],
            sdp_request: None,
            sdp_config: None,
        })
    }

    async fn data_plane_start(&self, input: DataPlaneStart) -> anyhow::Result<DataPlaneStartAck> {
        self.dataplane_process_service.set_dataplane_process_status(input.session_id.clone(), DataPlaneProcessState::STARTED).await?;
        let ack = DataPlaneStartAck {
            _type: DataPlaneControllerMessages::DataPlaneStartAck,
            version: DataPlaneControllerVersion::Version10,
            session_id: input.session_id,
        };
        Ok(ack)
    }

    async fn data_plane_stop(&self, input: DataPlaneStop) -> anyhow::Result<DataPlaneStopAck> {
        // Ignore error
        match self.dataplane_process_service.set_dataplane_process_status(input.session_id.clone(), DataPlaneProcessState::STOPPED).await {
            Ok(_) => {}
            Err(_) => {}
        };
        debug!("hola");
        let ack = DataPlaneStopAck {
            _type: DataPlaneControllerMessages::DataPlaneStopAck,
            version: DataPlaneControllerVersion::Version10,
            session_id: input.session_id,
        };
        Ok(ack)
    }

    async fn data_plane_get_status(&self, input: DataPlaneStatusRequest) -> anyhow::Result<DataPlaneStatusResponse> {
        let data_plane_process = self.dataplane_process_service.get_dataplane_process_by_id(input.session_id).await?;
        Ok(DataPlaneStatusResponse {
            _type: DataPlaneControllerMessages::DataPlaneStatusResponse,
            version: DataPlaneControllerVersion::Version10,
            session_id: data_plane_process.id,
            sdp_response: vec![
                DataPlaneSDPResponseField {
                    _type: DataPlaneSDPFieldTypes::DataPlaneAddressScheme,
                    format: "https://www.iana.org/assignments/uri-schemes/uri-schemes.xhtml".to_string(),
                    content: data_plane_process.process_address.protocol,
                },
                DataPlaneSDPResponseField {
                    _type: DataPlaneSDPFieldTypes::DataPlaneAddress,
                    format: "uri".to_string(),
                    content: data_plane_process.process_address.url,
                },
                DataPlaneSDPResponseField {
                    _type: DataPlaneSDPFieldTypes::DataPlaneAddressAuthType,
                    format: "https://www.iana.org/assignments/http-authschemes/http-authschemes.xhtml".to_string(),
                    content: data_plane_process.process_address.auth_type,
                },
                DataPlaneSDPResponseField {
                    _type: DataPlaneSDPFieldTypes::DataPlaneAddressAuthToken,
                    format: "jwt".to_string(),
                    content: data_plane_process.process_address.auth_content,
                }
            ],
        })
    }
}