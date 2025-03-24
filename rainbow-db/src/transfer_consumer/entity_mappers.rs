use crate::transfer_consumer::entities::transfer_callback;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use rainbow_common::protocol::transfer::TransferState;

impl From<transfer_callback::Model> for TransferProcessMessage {
    fn from(model: transfer_callback::Model) -> Self {
        TransferProcessMessage {
            provider_pid: model.provider_pid.unwrap_or("".to_string()),
            consumer_pid: model.consumer_pid,
            state: TransferState::REQUESTED,
            ..Default::default()
        }
    }
}