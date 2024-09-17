use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Debug, Clone)]
#[diesel(table_name = crate::db::schema::transfer_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TransferSession {
    pub id: Uuid,
    pub provider_pid: Uuid,
    pub consumer_pid: Uuid,
    pub state: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::db::schema::transfer_sessions)]
pub struct CreateTransferSession {
    pub id: Uuid,
    pub provider_pid: Uuid,
    pub consumer_pid: Uuid,
    pub state: String,
    pub created_at: chrono::NaiveDateTime,
}
