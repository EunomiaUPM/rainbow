use crate::db::get_db_connection;
use crate::transfer::consumer::data::models::{TransferCallbacksModel, TransferCallbacksModelNewState};
use crate::transfer::consumer::data::repo::TransferConsumerDataRepo;
use crate::transfer::consumer::data::schema::transfer_callbacks::dsl::transfer_callbacks;
use crate::transfer::consumer::data::schema::transfer_callbacks::{
    consumer_pid, data_address, id, provider_pid, updated_at,
};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{QueryDsl, SelectableHelper};
use uuid::Uuid;

pub struct TransferConsumerDataRepoPostgres {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl TransferConsumerDataRepoPostgres {
    pub fn new() -> Self {
        let connection_pool = get_db_connection();
        Self { connection_pool }
    }
}
impl TransferConsumerDataRepo for TransferConsumerDataRepoPostgres {
    fn get_all_callbacks(&self, limit: Option<i64>) -> anyhow::Result<Vec<TransferCallbacksModel>> {
        let connection = &mut self.connection_pool.get()?;
        let transaction = transfer_callbacks
            .limit(limit.unwrap_or(50))
            .select(TransferCallbacksModel::as_select())
            .load(connection)?;

        Ok(transaction)
    }
    fn get_callback_by_id(
        &self,
        callback_id: Uuid,
    ) -> anyhow::Result<Option<TransferCallbacksModel>> {
        let connection = &mut self.connection_pool.get()?;
        let transaction = transfer_callbacks
            .filter(id.eq(callback_id))
            .select(TransferCallbacksModel::as_select())
            .first(connection)
            .optional()?;

        Ok(transaction)
    }

    fn get_callback_by_consumer_id(
        &self,
        consumer_pid_in: Uuid,
    ) -> anyhow::Result<Option<TransferCallbacksModel>> {
        let connection = &mut self.connection_pool.get()?;
        let transaction = transfer_callbacks
            .filter(consumer_pid.eq(consumer_pid_in))
            .select(TransferCallbacksModel::as_select())
            .first(connection)
            .optional()?;

        Ok(transaction)
    }

    fn create_callback(&self) -> anyhow::Result<TransferCallbacksModel> {
        let connection = &mut self.connection_pool.get()?;
        let transaction = diesel::insert_into(transfer_callbacks)
            .values(TransferCallbacksModel {
                id: Uuid::new_v4(),
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: None,
                provider_pid: None,
                consumer_pid: None,
                data_address: None,
            })
            .returning(TransferCallbacksModel::as_select())
            .get_result(connection)?;

        Ok(transaction)
    }
    fn update_callback(
        &self,
        callback_id: Uuid,
        new_state: TransferCallbacksModelNewState,
    ) -> anyhow::Result<Option<TransferCallbacksModel>> {
        let connection = &mut self.connection_pool.get()?;
        let values = (
            consumer_pid.eq(new_state.consumer_pid),
            provider_pid.eq(new_state.provider_pid),
            data_address.eq(new_state.data_address),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        );
        let transaction = diesel::update(transfer_callbacks.filter(id.eq(callback_id)))
            .set(values)
            .returning(TransferCallbacksModel::as_select())
            .get_result(connection)
            .optional()?;

        Ok(transaction)
    }

    fn delete_callback(&self, callback_id: Uuid) -> anyhow::Result<()> {
        let connection = &mut self.connection_pool.get()?;
        let _ =
            diesel::delete(transfer_callbacks.filter(id.eq(callback_id))).execute(connection)?;

        Ok(())
    }
}
