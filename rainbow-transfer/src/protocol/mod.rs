use crate::provider::data::entities::transfer_process;
use rainbow_common::protocol::transfer::{TransferMessageTypes, TransferProcessMessage, TransferState, TRANSFER_CONTEXT};
use rainbow_common::utils::convert_uuid_to_uri;

impl From<transfer_process::Model> for TransferProcessMessage {
    fn from(model: transfer_process::Model) -> Self {
        TransferProcessMessage {
            context: TRANSFER_CONTEXT.to_string(),
            _type: TransferMessageTypes::TransferProcessMessage.to_string(),
            provider_pid: convert_uuid_to_uri(&model.provider_pid).unwrap(),
            consumer_pid: convert_uuid_to_uri(&model.consumer_pid.unwrap()).unwrap(),
            state: TransferState::from(model.state),
        }
    }
}