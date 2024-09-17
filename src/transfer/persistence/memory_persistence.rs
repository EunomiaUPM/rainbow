use crate::db::models::{CreateTransferSession, TransferSession};
use crate::transfer::persistence::Persistence;
use crate::transfer::protocol::messages::TransferState;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::OnceCell;
use tracing::debug;
use uuid::{uuid, Uuid};

pub struct MemoryPersistence;

static MEMORY_TRANSFER_SESSIONS: Lazy<Mutex<HashMap<Uuid, TransferSession>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

impl Persistence for MemoryPersistence {
    fn persist_transfer_request(request: CreateTransferSession) -> anyhow::Result<TransferSession> {
        let mut db = MEMORY_TRANSFER_SESSIONS.lock().unwrap();
        let transfer_session = TransferSession {
            id: request.id,
            provider_pid: request.provider_pid,
            consumer_pid: request.consumer_pid,
            state: request.state,
            created_at: request.created_at,
            updated_at: None,
        };
        let ts_clone = transfer_session.clone();
        db.insert(request.id, transfer_session);

        debug!("{:?}", db);
        Ok(ts_clone)
    }

    fn persist_transfer_start() -> anyhow::Result<TransferSession> {
        let mut db = MEMORY_TRANSFER_SESSIONS.lock().unwrap();
        let uuid = uuid!("0f4bb619-0a1f-4187-b7f3-fd34c592fba1");
        if let Some(transfer_session) = db.get_mut(&uuid) {
            transfer_session.state = TransferState::STARTED.to_string();
            let ts_clone = transfer_session.clone();
            debug!("{:?}", db);
            Ok(ts_clone)
        } else {
            Err(anyhow::anyhow!("Transfer session not found"))
        }
    }

    fn persist_transfer_suspension() -> anyhow::Result<TransferSession> {
        let mut db = MEMORY_TRANSFER_SESSIONS.lock().unwrap();
        let uuid = uuid!("0f4bb619-0a1f-4187-b7f3-fd34c592fba1");
        if let Some(transfer_session) = db.get_mut(&uuid) {
            transfer_session.state = TransferState::SUSPENDED.to_string();
            let ts_clone = transfer_session.clone();
            debug!("{:?}", db);
            Ok(ts_clone)
        } else {
            Err(anyhow::anyhow!("Transfer session not found"))
        }
    }

    fn persist_transfer_completion() -> anyhow::Result<TransferSession> {
        let mut db = MEMORY_TRANSFER_SESSIONS.lock().unwrap();
        let uuid = uuid!("0f4bb619-0a1f-4187-b7f3-fd34c592fba1");
        if let Some(transfer_session) = db.get_mut(&uuid) {
            transfer_session.state = TransferState::COMPLETED.to_string();
            let ts_clone = transfer_session.clone();
            debug!("{:?}", db);
            Ok(ts_clone)
        } else {
            Err(anyhow::anyhow!("Transfer session not found"))
        }
    }

    fn persist_transfer_termination() -> anyhow::Result<TransferSession> {
        let mut db = MEMORY_TRANSFER_SESSIONS.lock().unwrap();
        let uuid = uuid!("0f4bb619-0a1f-4187-b7f3-fd34c592fba1");
        if let Some(transfer_session) = db.get_mut(&uuid) {
            transfer_session.state = TransferState::TERMINATED.to_string();
            let ts_clone = transfer_session.clone();
            debug!("{:?}", db);
            Ok(ts_clone)
        } else {
            Err(anyhow::anyhow!("Transfer session not found"))
        }
    }
}

impl MemoryPersistence {
    fn get_transfers() -> anyhow::Result<Vec<TransferSession>> {
        let mut db = MEMORY_TRANSFER_SESSIONS.lock().unwrap();
        Ok(db.iter().map(|(_, v)| v.clone()).collect())
    }

    fn get_transfer_by_uuid(uuid: Uuid) -> anyhow::Result<Option<TransferSession>> {
        let mut db = MEMORY_TRANSFER_SESSIONS.lock().unwrap();
        let transfer_session = db.get(&uuid).map(|t| t.clone());
        Ok(transfer_session)
    }
}