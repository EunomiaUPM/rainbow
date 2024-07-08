use serde::{Deserialize, Serialize};
use serde_json::Value;
use garde::Validate;


const CONTEXT: &str = "https://w3id.org/dspace/2024/1/context.json";


#[derive(Debug, Serialize, Deserialize, Validate)]
#[garde(context(()))]
pub struct TransferRequestMessage {

    #[serde(rename = "@context")]
    #[garde(contains(CONTEXT))]
    pub context: String,

    #[serde(rename = "@type")]
    #[garde(skip)]
    pub _type: String,

    #[serde(rename = "dspace:consumerPid")]
    #[garde(contains("dspace:TransferRequestMessage"))]
    pub consumer_pid: String,

    #[serde(rename = "dspace:agreementId")]
    #[garde(skip)]
    pub agreement_id: String,

    #[serde(rename = "dct:format")]
    #[garde(skip)]
    pub format: String,

    #[serde(rename = "dspace:callbackAddress")]
    #[garde(skip)]
    pub callback_address: String,

    #[serde(rename = "dspace:dataAddress")]
    #[garde(dive)]
    pub data_address: Option<DataAddress>,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct TransferStartMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:dataAddress")]
    pub data_address: Option<DataAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSuspensionMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:code")]
    pub code: String,
    #[serde(rename = "dspace:reason")]
    pub reason: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCompletionMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferTerminationMessage {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "dspace:providerPid")]
    pub provider_pid: String,
    #[serde(rename = "dspace:consumerPid")]
    pub consumer_pid: String,
    #[serde(rename = "dspace:code")]
    pub code: String,
    #[serde(rename = "dspace:reason")]
    pub reason: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DataAddress {
    #[serde(rename = "@type")]
    #[garde(skip)]
    pub _type: String,

    #[serde(rename = "dspace:endpointType")]
    #[garde(skip)]
    pub endpoint_type: String,

    #[serde(rename = "dspace:endpoint")]
    #[garde(skip)]
    pub endpoint: String,

    #[serde(rename = "dspace:endpointProperties")]
    #[garde(dive)]
    pub endpoint_properties: Vec<EndpointProperty>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EndpointProperty {
    #[serde(rename = "@type")]
    #[garde(skip)]
    pub _type: String,

    #[serde(rename = "dspace:name")]
    #[garde(skip)]
    pub name: String,

    #[serde(rename = "dspace:value")]
    #[garde(skip)]
    pub value: String,
}



fn validate_uuid(value: &str) -> garde::Result {
    if value != "pass" {
        return Err(garde::Error::new("password is not strong enough"));
    }
    Ok(())
}
