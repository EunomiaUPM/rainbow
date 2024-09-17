use crate::db::models::{CreateTransferSession, TransferSession};
use crate::transfer::persistence::Persistence;

pub struct MongoPersistence;

impl Persistence for MongoPersistence {
    fn persist_transfer_request(request: CreateTransferSession) -> anyhow::Result<TransferSession> {
        todo!()
    }

    fn persist_transfer_start() -> anyhow::Result<TransferSession> {
        todo!()
    }

    fn persist_transfer_suspension() -> anyhow::Result<TransferSession> {
        todo!()
    }

    fn persist_transfer_completion() -> anyhow::Result<TransferSession> {
        todo!()
    }

    fn persist_transfer_termination() -> anyhow::Result<TransferSession> {
        todo!()
    }
}