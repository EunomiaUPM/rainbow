use crate::adv_protocol::interplane::{
    DataPlaneControllerMessages, DataPlaneControllerVersion, DataPlaneSDPResponseField,
};
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPlaneStatusRequest {
    #[serde(rename = "@type")]
    pub _type: DataPlaneControllerMessages,
    #[serde(rename = "@version")]
    pub version: DataPlaneControllerVersion,
    #[serde(rename = "sessionId")]
    pub session_id: Urn,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPlaneStatusResponse {
    #[serde(rename = "@type")]
    pub _type: DataPlaneControllerMessages,
    #[serde(rename = "@version")]
    pub version: DataPlaneControllerVersion,
    #[serde(rename = "sessionId")]
    pub session_id: Urn,
    #[serde(rename = "sdpResponse")]
    pub sdp_response: Vec<DataPlaneSDPResponseField>,
}
