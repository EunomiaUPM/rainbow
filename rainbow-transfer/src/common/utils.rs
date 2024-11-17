use crate::protocol::messages::DataAddress;
use rainbow_common::formats::{DctFormats, FormatAction};
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


