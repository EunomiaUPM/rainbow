use rainbow_common::utils::get_urn_from_string;
use rainbow_db::events::entities::notification;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use urn::Urn;

#[derive(Serialize, Deserialize)]
pub struct RainbowEventsNotificationResponse {
    #[serde(rename = "notificationId")]
    pub id: Urn,
    pub timestamp: chrono::NaiveDateTime,
    pub category: String,
    #[serde(rename = "messageType")]
    pub message_type: String,
    #[serde(rename = "messageContent")]
    pub message_content: serde_json::Value,
    #[serde(rename = "subscriptionId")]
    pub subscription_id: Urn,
}

impl TryFrom<notification::Model> for RainbowEventsNotificationResponse {
    type Error = anyhow::Error;

    fn try_from(value: notification::Model) -> anyhow::Result<Self> {
        Ok(Self {
            id: get_urn_from_string(&value.id)?,
            timestamp: value.timestamp,
            category: value.category,
            message_type: value.message_type,
            message_content: value.message_content,
            subscription_id: get_urn_from_string(&value.subscription_id)?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub enum RainbowEventsNotificationMessageTypes {
    RPCMessage,
    DSProtocolMessage,
    RainbowEntitiesMessage,
}

impl Display for RainbowEventsNotificationMessageTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RainbowEventsNotificationMessageTypes::RPCMessage => Ok(f.write_str("RPCMessage")?),
            RainbowEventsNotificationMessageTypes::DSProtocolMessage => Ok(f.write_str("DSProtocolMessage")?),
            RainbowEventsNotificationMessageTypes::RainbowEntitiesMessage => Ok(f.write_str("RainbowEntitiesMessage")?),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum RainbowEventsNotificationMessageCategory {
    TransferProcess,
    Catalog,
    ContractNegotiation,
    DataPlane,
}

impl Display for RainbowEventsNotificationMessageCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RainbowEventsNotificationMessageCategory::TransferProcess => Ok(f.write_str("TransferProcess")?),
            RainbowEventsNotificationMessageCategory::Catalog => Ok(f.write_str("Catalog")?),
            RainbowEventsNotificationMessageCategory::ContractNegotiation => Ok(f.write_str("ContractNegotiation")?),
            RainbowEventsNotificationMessageCategory::DataPlane => Ok(f.write_str("DataPlane")?),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum RainbowEventsNotificationStatus {
    Pending,
    Ok,
}

impl Display for RainbowEventsNotificationStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RainbowEventsNotificationStatus::Pending => Ok(f.write_str("Pending")?),
            RainbowEventsNotificationStatus::Ok => Ok(f.write_str("Ok")?),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RainbowEventsNotificationCreationRequest {
    pub category: RainbowEventsNotificationMessageCategory,
    pub message_type: RainbowEventsNotificationMessageTypes,
    pub message_content: serde_json::Value,
    pub status: RainbowEventsNotificationStatus,
}
