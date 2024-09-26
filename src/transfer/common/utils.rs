use crate::transfer::protocol::formats::{DctFormats, FormatAction};
use crate::transfer::protocol::messages::DataAddress;
use crate::transfer::provider::err::TransferErrorType;
use tracing::debug;
use uuid::Uuid;

pub fn is_consumer_pid_valid(consumer_pid: &String) -> anyhow::Result<bool> {
    debug!("Validating consumer_pid: {}", consumer_pid);
    Ok(true)
}

pub fn is_agreement_valid(agreement_id: &String) -> anyhow::Result<bool> {
    debug!("Validating agreement_id: {}", agreement_id);
    Ok(true)
}

pub fn is_provider_valid(provider_pid: &String) -> anyhow::Result<bool> {
    debug!("Validating provider_pid: {}", provider_pid);
    Ok(true)
}

pub fn has_data_address_in_push(
    data_address: &Option<DataAddress>,
    format: &DctFormats,
) -> anyhow::Result<bool> {
    let format_action = &format.action;
    match format_action {
        FormatAction::Push => Ok(data_address.is_some()),
        FormatAction::Pull => Ok(data_address.is_none()),
    }
}

pub fn does_callback_exist(callback_id: Uuid) -> anyhow::Result<bool> {
    debug!("{}", callback_id.to_string());
    Ok(true)
}

pub fn convert_uuid_to_uri(uuid: &Uuid) -> anyhow::Result<String> {
    Ok(format!("urn:uuid:{}", uuid.to_string()))
}

pub fn convert_uri_to_uuid(string: &String) -> anyhow::Result<Uuid> {
    let string = string.replace("urn:uuid:", "");
    let uuid = Uuid::parse_str(&string)?;
    Ok(uuid)
}
