use crate::transfer::consumer::data::schema::*;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Insertable)]
#[diesel(table_name=transfer_callbacks)]
#[diesel(primary_key(id))]
pub struct TransferCallbacksModel {
    pub id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
