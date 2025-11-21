use rainbow_common::protocol::context_field::ContextField;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::str::FromStr;
use urn::Urn;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferProcessMessageWrapper<T>
where
    T: TransferProcessMessageTrait,
{
    #[serde(rename = "@context")]
    pub context: ContextField,
    #[serde(rename = "@type")]
    pub _type: TransferProcessMessageType,
    #[serde(flatten)]
    pub dto: T,
}

pub trait TransferProcessMessageTrait: Debug + Send + Sync {
    fn get_consumer_pid(&self) -> Option<Urn>;
    fn get_provider_pid(&self) -> Option<Urn>;
    fn get_agreement_id(&self) -> Option<Urn>;
    fn get_format(&self) -> Option<String>;
    fn get_data_address(&self) -> Option<DataAddressDto>;
    fn get_callback_address(&self) -> Option<String>;
    fn get_error_code(&self) -> Option<String>;
    fn get_error_reason(&self) -> Option<Vec<String>>;
    fn get_message(&self) -> TransferProcessMessageType;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferRequestMessageDto {
    pub agreement_id: Urn,
    pub format: String,
    pub data_address: Option<DataAddressDto>,
    pub callback_address: String,
    pub consumer_pid: Urn,
}

impl TransferProcessMessageTrait for TransferRequestMessageDto {
    fn get_consumer_pid(&self) -> Option<Urn> {
        Some(self.consumer_pid.clone())
    }

    fn get_provider_pid(&self) -> Option<Urn> {
        None
    }

    fn get_agreement_id(&self) -> Option<Urn> {
        Some(self.agreement_id.clone())
    }

    fn get_format(&self) -> Option<String> {
        Some(self.format.clone())
    }

    fn get_data_address(&self) -> Option<DataAddressDto> {
        self.data_address.clone()
    }

    fn get_callback_address(&self) -> Option<String> {
        Some(self.callback_address.clone())
    }

    fn get_error_code(&self) -> Option<String> {
        None
    }

    fn get_error_reason(&self) -> Option<Vec<String>> {
        None
    }

    fn get_message(&self) -> TransferProcessMessageType {
        TransferProcessMessageType::TransferRequestMessage
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferStartMessageDto {
    pub provider_pid: Urn,
    pub consumer_pid: Urn,
    pub data_address: Option<DataAddressDto>,
}

impl TransferProcessMessageTrait for TransferStartMessageDto {
    fn get_consumer_pid(&self) -> Option<Urn> {
        Some(self.consumer_pid.clone())
    }

    fn get_provider_pid(&self) -> Option<Urn> {
        Some(self.provider_pid.clone())
    }

    fn get_agreement_id(&self) -> Option<Urn> {
        None
    }

    fn get_format(&self) -> Option<String> {
        None
    }

    fn get_data_address(&self) -> Option<DataAddressDto> {
        self.data_address.clone()
    }

    fn get_callback_address(&self) -> Option<String> {
        None
    }

    fn get_error_code(&self) -> Option<String> {
        None
    }

    fn get_error_reason(&self) -> Option<Vec<String>> {
        None
    }

    fn get_message(&self) -> TransferProcessMessageType {
        TransferProcessMessageType::TransferStartMessage
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferSuspensionMessageDto {
    pub provider_pid: Urn,
    pub consumer_pid: Urn,
    pub code: Option<String>,
    pub reason: Option<Vec<String>>,
}

impl TransferProcessMessageTrait for TransferSuspensionMessageDto {
    fn get_consumer_pid(&self) -> Option<Urn> {
        Some(self.consumer_pid.clone())
    }

    fn get_provider_pid(&self) -> Option<Urn> {
        Some(self.provider_pid.clone())
    }

    fn get_agreement_id(&self) -> Option<Urn> {
        None
    }

    fn get_format(&self) -> Option<String> {
        None
    }

    fn get_data_address(&self) -> Option<DataAddressDto> {
        None
    }

    fn get_callback_address(&self) -> Option<String> {
        None
    }

    fn get_error_code(&self) -> Option<String> {
        self.code.clone()
    }

    fn get_error_reason(&self) -> Option<Vec<String>> {
        self.reason.clone()
    }

    fn get_message(&self) -> TransferProcessMessageType {
        TransferProcessMessageType::TransferSuspensionMessage
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferCompletionMessageDto {
    pub provider_pid: Urn,
    pub consumer_pid: Urn,
}

impl TransferProcessMessageTrait for TransferCompletionMessageDto {
    fn get_consumer_pid(&self) -> Option<Urn> {
        Some(self.consumer_pid.clone())
    }

    fn get_provider_pid(&self) -> Option<Urn> {
        Some(self.provider_pid.clone())
    }

    fn get_agreement_id(&self) -> Option<Urn> {
        None
    }

    fn get_format(&self) -> Option<String> {
        None
    }

    fn get_data_address(&self) -> Option<DataAddressDto> {
        None
    }

    fn get_callback_address(&self) -> Option<String> {
        None
    }

    fn get_error_code(&self) -> Option<String> {
        None
    }

    fn get_error_reason(&self) -> Option<Vec<String>> {
        None
    }

    fn get_message(&self) -> TransferProcessMessageType {
        TransferProcessMessageType::TransferCompletionMessage
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferTerminationMessageDto {
    pub provider_pid: Urn,
    pub consumer_pid: Urn,
    pub code: Option<String>,
    pub reason: Option<Vec<String>>,
}

impl TransferProcessMessageTrait for TransferTerminationMessageDto {
    fn get_consumer_pid(&self) -> Option<Urn> {
        Some(self.consumer_pid.clone())
    }

    fn get_provider_pid(&self) -> Option<Urn> {
        Some(self.provider_pid.clone())
    }

    fn get_agreement_id(&self) -> Option<Urn> {
        None
    }

    fn get_format(&self) -> Option<String> {
        None
    }

    fn get_data_address(&self) -> Option<DataAddressDto> {
        None
    }

    fn get_callback_address(&self) -> Option<String> {
        None
    }

    fn get_error_code(&self) -> Option<String> {
        self.code.clone()
    }

    fn get_error_reason(&self) -> Option<Vec<String>> {
        self.reason.clone()
    }

    fn get_message(&self) -> TransferProcessMessageType {
        TransferProcessMessageType::TransferTerminationMessage
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataAddressDto {
    pub endpoint_type: String,
    pub endpoint: Option<String>,
    pub endpoint_properties: Option<Vec<EndpointPropertyDto>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPropertyDto {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferProcessAckDto {
    pub consumer_pid: Urn,
    pub provider_pid: Urn,
    pub state: TransferProcessState,
}

impl TransferProcessMessageTrait for TransferProcessAckDto {
    fn get_consumer_pid(&self) -> Option<Urn> {
        Some(self.consumer_pid.clone())
    }

    fn get_provider_pid(&self) -> Option<Urn> {
        Some(self.provider_pid.clone())
    }

    fn get_agreement_id(&self) -> Option<Urn> {
        None
    }

    fn get_format(&self) -> Option<String> {
        None
    }

    fn get_data_address(&self) -> Option<DataAddressDto> {
        None
    }

    fn get_callback_address(&self) -> Option<String> {
        None
    }

    fn get_error_code(&self) -> Option<String> {
        None
    }

    fn get_error_reason(&self) -> Option<Vec<String>> {
        None
    }

    fn get_message(&self) -> TransferProcessMessageType {
        TransferProcessMessageType::TransferProcess
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TransferErrorDto {
    pub consumer_pid: Option<Urn>,
    pub provider_pid: Option<Urn>,
    pub code: Option<String>,
    pub reason: Option<Vec<String>>,
}

impl TransferProcessMessageTrait for TransferErrorDto {
    fn get_consumer_pid(&self) -> Option<Urn> {
        self.consumer_pid.clone()
    }

    fn get_provider_pid(&self) -> Option<Urn> {
        self.provider_pid.clone()
    }

    fn get_agreement_id(&self) -> Option<Urn> {
        None
    }

    fn get_format(&self) -> Option<String> {
        None
    }

    fn get_data_address(&self) -> Option<DataAddressDto> {
        None
    }

    fn get_callback_address(&self) -> Option<String> {
        None
    }

    fn get_error_code(&self) -> Option<String> {
        self.code.clone()
    }

    fn get_error_reason(&self) -> Option<Vec<String>> {
        self.reason.clone()
    }

    fn get_message(&self) -> TransferProcessMessageType {
        TransferProcessMessageType::TransferError
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransferProcessState {
    Requested,
    Started,
    Completed,
    Suspended,
    Terminated,
}
impl Display for TransferProcessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TransferProcessState::Requested => "REQUESTED".to_string(),
            TransferProcessState::Started => "STARTED".to_string(),
            TransferProcessState::Completed => "COMPLETED".to_string(),
            TransferProcessState::Suspended => "SUSPENDED".to_string(),
            TransferProcessState::Terminated => "TERMINATED".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for TransferProcessState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "REQUESTED" => Ok(TransferProcessState::Requested),
            "STARTED" => Ok(TransferProcessState::Started),
            "COMPLETED" => Ok(TransferProcessState::Completed),
            "SUSPENDED" => Ok(TransferProcessState::Suspended),
            "TERMINATED" => Ok(TransferProcessState::Terminated),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TransferProcessMessageType {
    TransferRequestMessage,
    TransferStartMessage,
    TransferCompletionMessage,
    TransferSuspensionMessage,
    TransferTerminationMessage,
    TransferProcess,
    TransferError,
}
