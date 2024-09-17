use crate::db::models::{CreateTransferSession, TransferSession};


pub mod sql_persistence;
pub mod memory_persistence;
pub mod mongo_persistence;


pub trait Persistence {
    fn persist_transfer_request(request: CreateTransferSession) -> anyhow::Result<TransferSession>;
    fn persist_transfer_start() -> anyhow::Result<TransferSession>;
    fn persist_transfer_suspension() -> anyhow::Result<TransferSession>;
    fn persist_transfer_completion() -> anyhow::Result<TransferSession>;
    fn persist_transfer_termination() -> anyhow::Result<TransferSession>;
}

