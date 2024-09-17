use crate::db::get_db_connection;
use crate::db::models::{CreateTransferSession, TransferSession};
use crate::transfer::persistence::Persistence;
use crate::transfer::protocol::messages::TransferState;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::uuid;

pub struct SQLPersistence;

impl Persistence for SQLPersistence {
    fn persist_transfer_request(request: CreateTransferSession) -> anyhow::Result<TransferSession> {
        use crate::db::schema::transfer_sessions::dsl::transfer_sessions;
        let connection = &mut get_db_connection().get()?;
        let transaction = diesel::insert_into(transfer_sessions)
            .values(&request)
            .returning(TransferSession::as_returning())
            .get_result(connection)?;

        Ok(transaction)
    }

    fn persist_transfer_start() -> anyhow::Result<TransferSession> {
        use crate::db::schema::transfer_sessions::dsl::{state, transfer_sessions, updated_at};
        let connection = &mut get_db_connection().get()?;
        // TODO Uuid should be the right one...
        let uuid = uuid!("0f4bb619-0a1f-4187-b7f3-fd34c592fba1");
        let values = (
            state.eq(TransferState::STARTED.to_string()),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        );

        let transaction = diesel::update(transfer_sessions.find(uuid))
            .set(values)
            .returning(TransferSession::as_returning())
            .get_result(connection)?;

        Ok(transaction)
    }

    fn persist_transfer_suspension() -> anyhow::Result<TransferSession> {
        use crate::db::schema::transfer_sessions::dsl::{state, transfer_sessions, updated_at};
        let connection = &mut get_db_connection().get()?;
        // TODO Uuid should be the right one...
        let uuid = uuid!("0f4bb619-0a1f-4187-b7f3-fd34c592fba1");
        let values = (
            state.eq(TransferState::SUSPENDED.to_string()),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        );

        let transaction = diesel::update(transfer_sessions.find(uuid))
            .set(values)
            .returning(TransferSession::as_returning())
            .get_result(connection)?;

        Ok(transaction)
    }

    fn persist_transfer_completion() -> anyhow::Result<TransferSession> {
        use crate::db::schema::transfer_sessions::dsl::{state, transfer_sessions, updated_at};
        let connection = &mut get_db_connection().get()?;
        // TODO Uuid should be the right one...
        let uuid = uuid!("0f4bb619-0a1f-4187-b7f3-fd34c592fba1");
        let values = (
            state.eq(TransferState::COMPLETED.to_string()),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        );

        let transaction = diesel::update(transfer_sessions.find(uuid))
            .set(values)
            .returning(TransferSession::as_returning())
            .get_result(connection)?;

        Ok(transaction)
    }

    fn persist_transfer_termination() -> anyhow::Result<TransferSession> {
        use crate::db::schema::transfer_sessions::dsl::{state, transfer_sessions, updated_at};
        let connection = &mut get_db_connection().get()?;
        // TODO Uuid should be the right one...
        let uuid = uuid!("0f4bb619-0a1f-4187-b7f3-fd34c592fba1");
        let values = (
            state.eq(TransferState::TERMINATED.to_string()),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        );

        let transaction = diesel::update(transfer_sessions.find(uuid))
            .set(values)
            .returning(TransferSession::as_returning())
            .get_result(connection)?;

        Ok(transaction)
    }
}
