use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TransferState {
    Requested,
    Started,
    Suspended,
    Completed,
    Terminated,
}

pub enum Response {
    Ack,
    Error,
}
