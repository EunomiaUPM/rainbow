use crate::adv_protocol::interplane::{DataPlaneControllerMessages, DataPlaneControllerVersion};
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPlaneStart {
    #[serde(rename = "@type")]
    pub _type: DataPlaneControllerMessages,
    #[serde(rename = "@version")]
    pub version: DataPlaneControllerVersion,
    #[serde(rename = "sessionId")]
    pub session_id: Urn,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPlaneStartAck {
    #[serde(rename = "@type")]
    pub _type: DataPlaneControllerMessages,
    #[serde(rename = "@version")]
    pub version: DataPlaneControllerVersion,
    #[serde(rename = "sessionId")]
    pub session_id: Urn,
}