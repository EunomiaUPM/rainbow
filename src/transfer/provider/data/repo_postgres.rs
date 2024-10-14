use crate::db::get_db_connection;
use crate::transfer::protocol::messages::{TransferMessageTypes, TransferState};
use crate::transfer::provider::data::models::{
    DataPlaneProcessModel, TransferMessageModel, TransferProcessModel,
};
use crate::transfer::provider::data::schema::transfer_messages::dsl::{
    id as message_id, message_type, transfer_messages,
};
use crate::transfer::provider::data::schema::transfer_processes::dsl::transfer_processes;
use crate::transfer::provider::data::schema::transfer_processes::dsl::*;

use crate::transfer::provider::data::repo::TransferProviderDataRepo;
use crate::transfer::provider::data::schema::data_plane_processes;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

pub struct TransferProviderDataRepoPostgres {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl TransferProviderDataRepoPostgres {
    pub fn new() -> Self {
        let connection_pool = get_db_connection();
        Self { connection_pool }
    }
}

impl TransferProviderDataRepo for TransferProviderDataRepoPostgres {
    fn get_all_transfer_processes(
        &self,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<TransferProcessModel>> {
        // let connection = &mut get_db_connection().get()?;
        let connection = &mut self.connection_pool.get()?;
        let transaction = transfer_processes
            .limit(limit.unwrap_or(20))
            .select(TransferProcessModel::as_select())
            .load(connection)?;

        Ok(transaction)
    }

    fn get_transfer_process_by_consumer_pid(
        &self,
        consumer_pid_in: Uuid,
    ) -> anyhow::Result<Option<TransferProcessModel>> {
        let connection = &mut self.connection_pool.get()?;
        let transaction = transfer_processes
            .filter(consumer_pid.eq(consumer_pid_in))
            .select(TransferProcessModel::as_select())
            .first(connection)
            .optional()?;

        Ok(transaction)
    }

    fn get_transfer_process_by_provider_pid(
        &self,
        provider_pid_in: Uuid,
    ) -> anyhow::Result<Option<TransferProcessModel>> {
        let connection = &mut self.connection_pool.get()?;
        let transaction = transfer_processes
            .filter(provider_pid.eq(provider_pid_in))
            .select(TransferProcessModel::as_select())
            .first(connection)
            .optional()?;

        Ok(transaction)
    }

    fn create_transfer_process(
        &self,
        transfer_process: TransferProcessModel,
    ) -> anyhow::Result<()> {
        let connection = &mut self.connection_pool.get()?;
        let _ = diesel::insert_into(transfer_processes)
            .values(&transfer_process)
            .execute(connection)?;

        Ok(())
    }

    fn update_transfer_process_by_provider_pid(
        &self,
        provider_pid_in: &Uuid,
        new_state: TransferState,
    ) -> anyhow::Result<Option<TransferProcessModel>> {
        let connection = &mut self.connection_pool.get()?;
        let find = transfer_processes.find(provider_pid_in);
        let values = (
            state.eq(new_state.to_string()),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        );
        let transaction = diesel::update(find)
            .set(values)
            .returning(TransferProcessModel::as_returning())
            .get_result(connection)
            .optional()?;

        Ok(transaction)
    }

    fn get_all_transfer_messages(
        &self,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<TransferMessageModel>> {
        // TODO create joins and return format ok
        let connection = &mut self.connection_pool.get()?;
        let transaction = transfer_messages
            .limit(limit.unwrap_or(20))
            .select(TransferMessageModel::as_select())
            .load(connection)?;

        Ok(transaction)
    }

    fn get_all_transfer_messages_by_type(
        &self,
        message_type_in: TransferMessageTypes,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<TransferMessageModel>> {
        let connection = &mut self.connection_pool.get()?;
        let transaction = transfer_messages
            .filter(message_type.eq(message_type_in.to_string()))
            .limit(limit.unwrap_or(20))
            .select(TransferMessageModel::as_select())
            .load(connection)?;

        Ok(transaction)
    }

    fn get_transfer_message_by_id(
        &self,
        message_id_in: Uuid,
    ) -> anyhow::Result<Option<TransferMessageModel>> {
        let connection = &mut self.connection_pool.get()?;
        let transaction = transfer_messages
            .filter(message_id.eq(message_id_in))
            .select(TransferMessageModel::as_select())
            .first(connection)
            .optional()?;

        Ok(transaction)
    }

    fn create_transfer_message(&self, message: TransferMessageModel) -> anyhow::Result<()> {
        let connection = &mut self.connection_pool.get()?;
        let _ = diesel::insert_into(transfer_messages)
            .values(&message)
            .execute(connection)?;

        Ok(())
    }

    fn create_data_plane_process(
        &self,
        data_plane_process: DataPlaneProcessModel,
    ) -> anyhow::Result<DataPlaneProcessModel> {
        let connection = &mut self.connection_pool.get()?;
        let transaction = diesel::insert_into(data_plane_processes::table)
            .values(data_plane_process)
            .get_result(connection)?;

        Ok(transaction)
    }

    fn update_data_plane_process(
        &self,
        data_plane_process_id: Uuid,
        new_state: bool,
    ) -> anyhow::Result<Option<DataPlaneProcessModel>> {
        let connection = &mut self.connection_pool.get()?;
        let find = data_plane_processes::table.find(data_plane_process_id);
        let values = (
            data_plane_processes::dsl::updated_at.eq(chrono::Utc::now().naive_utc()),
            data_plane_processes::dsl::state.eq(new_state),
        );
        let transaction = diesel::update(find)
            .set(values)
            .returning(DataPlaneProcessModel::as_returning())
            .get_result(connection)
            .optional()?;

        Ok(transaction)
    }

    fn get_data_plane_process_by_id(
        &self,
        data_plane_id: Uuid,
    ) -> anyhow::Result<Option<DataPlaneProcessModel>> {
        let connection = &mut self.connection_pool.get()?;
        let transaction = data_plane_processes::table
            .find(data_plane_id)
            .select(DataPlaneProcessModel::as_select())
            .first(connection)
            .optional()?;

        Ok(transaction)
    }

    fn get_data_plane_process_by_transfer_process_id(
        &self,
        transfer_process_id: Uuid,
    ) -> anyhow::Result<Option<DataPlaneProcessModel>> {
        let connection = &mut self.connection_pool.get()?;
        let transaction = data_plane_processes::table
            .filter(data_plane_processes::transfer_process_id.eq(transfer_process_id))
            .select(DataPlaneProcessModel::as_select())
            .first(connection)
            .optional()?;

        Ok(transaction)
    }

    fn delete_data_plane_process_by_id(&self, data_plane_id: Uuid) -> anyhow::Result<()> {
        let connection = &mut self.connection_pool.get()?;
        let find = data_plane_processes::table.find(data_plane_id);
        let _ = diesel::delete(find).execute(connection)?;

        Ok(())
    }
}
