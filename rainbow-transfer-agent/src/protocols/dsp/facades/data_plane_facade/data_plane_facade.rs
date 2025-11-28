/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::protocols::dsp::facades::data_plane_facade::DataPlaneProviderFacadeTrait;
use crate::protocols::dsp::protocol_types::{DataAddressDto, EndpointPropertyDto};
use rainbow_common::adv_protocol::interplane::data_plane_provision::DataPlaneProvisionRequest;
use rainbow_common::adv_protocol::interplane::data_plane_start::DataPlaneStart;
use rainbow_common::adv_protocol::interplane::data_plane_status::DataPlaneStatusRequest;
use rainbow_common::adv_protocol::interplane::data_plane_stop::DataPlaneStop;
use rainbow_common::adv_protocol::interplane::{
    DataPlaneControllerMessages, DataPlaneControllerVersion, DataPlaneSDPConfigField, DataPlaneSDPConfigTypes,
    DataPlaneSDPFieldTypes, DataPlaneSDPRequestField,
};
use rainbow_common::dcat_formats::{DctFormats, FormatAction};
use rainbow_common::protocol::catalog::dataservice_definition::{DataService, DataServiceDcatDeclaration};
use rainbow_dataplane::coordinator::dataplane_access_controller::DataPlaneAccessControllerTrait;
use std::sync::Arc;
use url::Url;
use urn::Urn;

pub struct DataPlaneProviderFacade {
    dataplane_controller: Arc<dyn DataPlaneAccessControllerTrait>,
}

impl DataPlaneProviderFacade {
    pub fn new(dataplane_controller: Arc<dyn DataPlaneAccessControllerTrait>) -> Self {
        Self { dataplane_controller }
    }
}

#[async_trait::async_trait]
impl DataPlaneProviderFacadeTrait for DataPlaneProviderFacade {
    async fn get_dataplane_address(&self, session_id: &Urn) -> anyhow::Result<DataAddressDto> {
        let status = self
            .dataplane_controller
            .data_plane_get_status(&DataPlaneStatusRequest {
                _type: DataPlaneControllerMessages::DataPlaneStatusRequest,
                version: DataPlaneControllerVersion::Version10,
                session_id: session_id.clone(),
            })
            .await?;
        let scheme = status
            .sdp_response
            .iter()
            .find(|f| f._type == DataPlaneSDPFieldTypes::DataPlaneAddressScheme)
            .unwrap()
            .content
            .clone();
        let address = status
            .sdp_response
            .iter()
            .find(|f| f._type == DataPlaneSDPFieldTypes::DataPlaneAddress)
            .unwrap()
            .content
            .clone();
        let auth_type = status
            .sdp_response
            .iter()
            .find(|f| f._type == DataPlaneSDPFieldTypes::DataPlaneAddressAuthType)
            .unwrap()
            .content
            .clone();
        let auth_content = status
            .sdp_response
            .iter()
            .find(|f| f._type == DataPlaneSDPFieldTypes::DataPlaneAddressAuthToken)
            .unwrap()
            .content
            .clone();

        let data_address = DataAddressDto {
            endpoint_type: scheme,
            endpoint: Option::from(address),
            endpoint_properties: Option::from(vec![
                EndpointPropertyDto { name: "authType".to_string(), value: auth_type },
                EndpointPropertyDto { name: "authorization".to_string(), value: auth_content },
            ]),
        };
        Ok(data_address)
    }

    async fn on_transfer_request(
        &self,
        session_id: &Urn,
        data_service: &DataService,
        format: &DctFormats,
    ) -> anyhow::Result<()> {
        let DataService { dcat, .. } = data_service;
        let DataServiceDcatDeclaration { endpoint_url, .. } = dcat;
        let endpoint_url = Url::parse(endpoint_url.as_str())?;
        let endpoint_scheme = endpoint_url.scheme().to_string();
        let endpoint_address = endpoint_url.to_string();

        let _dataplane_response = match format.action {
            FormatAction::Push => {
                // TODO push case next_hop should point to consumer dataplane
                todo!()
            }
            FormatAction::Pull => {
                self.dataplane_controller
                    .data_plane_provision_request(&DataPlaneProvisionRequest {
                        _type: DataPlaneControllerMessages::DataPlaneProvisionRequest,
                        version: DataPlaneControllerVersion::Version10,
                        session_id: session_id.clone(),
                        sdp_request: vec![
                            DataPlaneSDPRequestField {
                                _type: DataPlaneSDPFieldTypes::DataPlaneAddressScheme,
                                format: "https://www.iana.org/assignments/uri-schemes/uri-schemes.xhtml".to_string(),
                            },
                            DataPlaneSDPRequestField {
                                _type: DataPlaneSDPFieldTypes::DataPlaneAddress,
                                format: "uri".to_string(),
                            },
                            DataPlaneSDPRequestField {
                                _type: DataPlaneSDPFieldTypes::DataPlaneAddressAuthType,
                                format: "https://www.iana.org/assignments/http-authschemes/http-authschemes.xhtml"
                                    .to_string(),
                            },
                            DataPlaneSDPRequestField {
                                _type: DataPlaneSDPFieldTypes::DataPlaneAddressAuthToken,
                                format: "jwt".to_string(),
                            },
                        ],
                        sdp_config: Some(vec![
                            DataPlaneSDPConfigField {
                                _type: DataPlaneSDPConfigTypes::NextHopAddressScheme,
                                format: Some(
                                    "https://www.iana.org/assignments/uri-schemes/uri-schemes.xhtml".to_string(),
                                ),
                                content: endpoint_scheme,
                            },
                            DataPlaneSDPConfigField {
                                _type: DataPlaneSDPConfigTypes::NextHopAddress,
                                format: Some("uri".to_string()),
                                content: endpoint_address,
                            },
                            DataPlaneSDPConfigField {
                                _type: DataPlaneSDPConfigTypes::Direction,
                                format: Some("dcterms:transferDirection".to_string()),
                                content: FormatAction::Pull.to_string(),
                            },
                        ]),
                    })
                    .await?
            }
        };

        Ok(())
    }

    async fn on_transfer_start(&self, session_id: &Urn) -> anyhow::Result<()> {
        let _ = self
            .dataplane_controller
            .data_plane_start(&DataPlaneStart {
                _type: DataPlaneControllerMessages::DataPlaneStart,
                version: DataPlaneControllerVersion::Version10,
                session_id: session_id.clone(),
            })
            .await?;
        Ok(())
    }

    async fn on_transfer_suspension(&self, session_id: &Urn) -> anyhow::Result<()> {
        let _ = self
            .dataplane_controller
            .data_plane_stop(&DataPlaneStop {
                _type: DataPlaneControllerMessages::DataPlaneStop,
                version: DataPlaneControllerVersion::Version10,
                session_id: session_id.clone(),
            })
            .await?;
        Ok(())
    }

    async fn on_transfer_completion(&self, session_id: &Urn) -> anyhow::Result<()> {
        let _ = self
            .dataplane_controller
            .data_plane_stop(&DataPlaneStop {
                _type: DataPlaneControllerMessages::DataPlaneStop,
                version: DataPlaneControllerVersion::Version10,
                session_id: session_id.clone(),
            })
            .await?;
        Ok(())
    }

    async fn on_transfer_termination(&self, session_id: &Urn) -> anyhow::Result<()> {
        let _ = self
            .dataplane_controller
            .data_plane_stop(&DataPlaneStop {
                _type: DataPlaneControllerMessages::DataPlaneStop,
                version: DataPlaneControllerVersion::Version10,
                session_id: session_id.clone(),
            })
            .await?;
        Ok(())
    }
}
