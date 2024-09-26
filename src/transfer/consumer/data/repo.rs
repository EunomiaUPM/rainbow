use crate::db::get_db_connection;
use crate::transfer::consumer::data::models::TransferCallbacksModel;
use crate::transfer::consumer::data::schema::transfer_callbacks::dsl::transfer_callbacks;
use crate::transfer::consumer::data::schema::transfer_callbacks::{id, updated_at};
use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use uuid::Uuid;

pub fn get_all_callbacks(limit: Option<i64>) -> anyhow::Result<Vec<TransferCallbacksModel>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_callbacks
        .limit(limit.unwrap_or(20))
        .select(TransferCallbacksModel::as_select())
        .load(connection)?;

    Ok(transaction)
}
pub fn get_callback_by_id(callback_id: Uuid) -> anyhow::Result<Option<TransferCallbacksModel>> {
    let connection = &mut get_db_connection().get()?;
    let transaction = transfer_callbacks
        .filter(id.eq(callback_id))
        .select(TransferCallbacksModel::as_select())
        .first(connection)
        .optional()?;

    Ok(transaction)
}
pub fn create_callback() -> anyhow::Result<TransferCallbacksModel> {
    let connection = &mut get_db_connection().get()?;
    let transaction = diesel::insert_into(transfer_callbacks)
        .values(TransferCallbacksModel {
            id: Uuid::new_v4(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
        })
        .returning(TransferCallbacksModel::as_select())
        .get_result(connection)?;

    Ok(transaction)
}
pub fn update_callback(callback_id: Uuid) -> anyhow::Result<()> {
    let connection = &mut get_db_connection().get()?;
    let values = (
        id.eq(callback_id),
        updated_at.eq(chrono::Utc::now().naive_utc()),
    );
    let _ = diesel::update(transfer_callbacks.filter(id.eq(callback_id)))
        .set(values)
        .execute(connection)?;

    Ok(())
}

pub fn delete_callback(callback_id: Uuid) -> anyhow::Result<()> {
    let connection = &mut get_db_connection().get()?;
    let _ = diesel::delete(transfer_callbacks.filter(id.eq(callback_id))).execute(connection)?;

    Ok(())
}
