use super::entities::transfer_process;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;

impl From<transfer_process::Model> for TransferProcessMessage {
    fn from(model: transfer_process::Model) -> Self {
        TransferProcessMessage {
            provider_pid: model.provider_pid,
            consumer_pid: model.consumer_pid.unwrap(),
            state: model.state.parse().unwrap(),
            ..Default::default()
        }
    }
}