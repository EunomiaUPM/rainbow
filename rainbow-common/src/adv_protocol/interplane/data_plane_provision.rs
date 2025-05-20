use crate::adv_protocol::interplane::{
    DataPlaneControllerMessages, DataPlaneControllerVersion, DataPlaneSDPConfigField, DataPlaneSDPRequestField,
    DataPlaneSDPResponseField,
};
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPlaneProvisionRequest {
    #[serde(rename = "@type")]
    pub _type: DataPlaneControllerMessages,
    #[serde(rename = "@version")]
    pub version: DataPlaneControllerVersion,
    #[serde(rename = "sessionId")]
    pub session_id: Urn,
    #[serde(rename = "sdpRequest")]
    pub sdp_request: Vec<DataPlaneSDPRequestField>,
    #[serde(rename = "sdpConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp_config: Option<Vec<DataPlaneSDPConfigField>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPlaneProvisionResponse {
    #[serde(rename = "@type")]
    pub _type: DataPlaneControllerMessages,
    #[serde(rename = "@version")]
    pub version: DataPlaneControllerVersion,
    #[serde(rename = "sessionId")]
    pub session_id: Urn,
    #[serde(rename = "sdpResponse")]
    pub sdp_response: Vec<DataPlaneSDPResponseField>,
    #[serde(rename = "sdpRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp_request: Option<Vec<DataPlaneSDPRequestField>>,
    #[serde(rename = "sdpConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdp_config: Option<Vec<DataPlaneSDPConfigField>>,
}
