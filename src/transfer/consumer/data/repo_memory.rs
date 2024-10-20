use crate::db::get_db_memory_connection_r2d2;
use crate::transfer::consumer::data::models::{TransferCallbacksModel, TransferCallbacksModelNewState};
use crate::transfer::consumer::data::repo::TransferConsumerDataRepo;
use chrono::DateTime;
use diesel::r2d2::Pool;
use duckdb::types::Value::Null;
use duckdb::{params, DuckdbConnectionManager};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct TransferConsumerDataRepoMemory {
    client: Pool<DuckdbConnectionManager>,
}
impl TransferConsumerDataRepoMemory {
    pub fn new() -> Self {
        let client = get_db_memory_connection_r2d2();
        Self { client }
    }
}
impl TransferConsumerDataRepo for TransferConsumerDataRepoMemory {
    fn get_all_callbacks(&self, limit: Option<i64>) -> anyhow::Result<Vec<TransferCallbacksModel>> {
        let conn = self.client.get()?;
        let mut transaction = conn.prepare(
            r#"
            SELECT * FROM consumer.transfer_callbacks
            LIMIT ?
        "#,
        )?;
        let results = transaction.query_map(params![limit.unwrap_or(50)], |row| {
            Ok(TransferCallbacksModel {
                id: row.get(0)?,
                created_at: DateTime::from_timestamp_micros(row.get(1)?)
                    .unwrap()
                    .naive_utc(),
                updated_at: match row.get(2)? {
                    Null => None,
                    _ => Option::from(
                        DateTime::from_timestamp_micros(row.get(2)?)
                            .unwrap()
                            .naive_utc(),
                    ),
                },
                provider_pid: row.get(3)?,
                consumer_pid: row.get(4)?,
                data_address: row.get(5)?,
            })
        })?;
        let mut callbacks: Vec<TransferCallbacksModel> = Vec::new();
        for result in results {
            callbacks.push(result?);
        }
        Ok(callbacks)
    }
    fn get_callback_by_id(
        &self,
        callback_id: Uuid,
    ) -> anyhow::Result<Option<TransferCallbacksModel>> {
        let conn = self.client.get()?;
        let mut transaction = conn.prepare(
            r#"
            SELECT * FROM consumer.transfer_callbacks
            WHERE id = ?
        "#,
        )?;
        let results = transaction.query_row(params![callback_id], |row| {
            Ok(TransferCallbacksModel {
                id: row.get(0)?,
                created_at: DateTime::from_timestamp_micros(row.get(1)?)
                    .unwrap()
                    .naive_utc(),
                updated_at: match row.get(2)? {
                    Null => None,
                    _ => Option::from(
                        DateTime::from_timestamp_micros(row.get(2)?)
                            .unwrap()
                            .naive_utc(),
                    ),
                },
                provider_pid: row.get(3)?,
                consumer_pid: row.get(4)?,
                data_address: row.get(5)?,
            })
        });
        if results.is_err() {
            return Ok(None);
        }
        Ok(Some(results?))
    }

    fn get_callback_by_consumer_id(
        &self,
        consumer_pid_in: Uuid,
    ) -> anyhow::Result<Option<TransferCallbacksModel>> {
        let conn = self.client.get()?;
        let mut transaction = conn.prepare(
            r#"
            SELECT * FROM consumer.transfer_callbacks
            WHERE consumer_pid = ?
        "#,
        )?;
        let results = transaction.query_row(params![consumer_pid_in], |row| {
            Ok(TransferCallbacksModel {
                id: row.get(0)?,
                created_at: DateTime::from_timestamp_micros(row.get(1)?)
                    .unwrap()
                    .naive_utc(),
                updated_at: match row.get(2)? {
                    Null => None,
                    _ => Option::from(
                        DateTime::from_timestamp_micros(row.get(2)?)
                            .unwrap()
                            .naive_utc(),
                    ),
                },
                provider_pid: row.get(3)?,
                consumer_pid: row.get(4)?,
                data_address: row.get(5)?,
            })
        });
        if results.is_err() {
            return Ok(None);
        }
        Ok(Some(results?))
    }

    fn create_callback(&self) -> anyhow::Result<TransferCallbacksModel> {
        let conn = self.client.get()?;
        let id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();

        #[derive(Serialize, Deserialize)]
        struct Test {
            a: i32,
        }
        let transaction = conn.execute(
            r#"
            INSERT INTO consumer.transfer_callbacks
            VALUES (?, ?, null, null, null, ?)
        "#,
            params![
                id.to_string(),
                now.to_string(),
                serde_json::to_string(&Test { a: 0 })?
            ],
        )?;
        let mut transaction = conn.prepare(
            r#"
            SELECT * FROM consumer.transfer_callbacks
            WHERE id = ?
            LIMIT 1
        "#,
        )?;
        let result = transaction.query_row(params![id.to_string()], |row| {
            Ok(TransferCallbacksModel {
                id: row.get(0)?,
                created_at: DateTime::from_timestamp_micros(row.get(1)?)
                    .unwrap()
                    .naive_utc(),
                updated_at: match row.get(2)? {
                    Null => None,
                    _ => Option::from(
                        DateTime::from_timestamp_micros(row.get(2)?)
                            .unwrap()
                            .naive_utc(),
                    ),
                },
                provider_pid: row.get(3)?,
                consumer_pid: row.get(4)?,
                data_address: row.get(5)?,
            })
        });
        Ok(result?)
    }
    fn update_callback(
        &self,
        callback_id: Uuid,
        new_state: TransferCallbacksModelNewState,
    ) -> anyhow::Result<Option<TransferCallbacksModel>> {
        let conn = self.client.get()?;
        let id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();

        #[derive(Serialize, Deserialize)]
        struct Test {
            a: i32,
        }
        let transaction = conn.execute(
            r#"
            UPDATE consumer.transfer_callbacks
            SET updated_at = ?,
                provider_pid = ?,
                consumer_pid = ?,
                data_address = ?
            WHERE id = ?
        "#,
            params![
                now.to_string(),
                new_state.provider_pid,
                new_state.consumer_pid,
                serde_json::to_string(&new_state.consumer_pid)?,
                id.to_string(),
            ],
        )?;
        let mut transaction = conn.prepare(
            r#"
            SELECT * FROM consumer.transfer_callbacks
            WHERE id = ?
            LIMIT 1
        "#,
        )?;
        let result = transaction.query_row(params![id.to_string()], |row| {
            Ok(TransferCallbacksModel {
                id: row.get(0)?,
                created_at: DateTime::from_timestamp_micros(row.get(1)?)
                    .unwrap()
                    .naive_utc(),
                updated_at: match row.get(2)? {
                    Null => None,
                    _ => Option::from(
                        DateTime::from_timestamp_micros(row.get(2)?)
                            .unwrap()
                            .naive_utc(),
                    ),
                },
                provider_pid: row.get(3)?,
                consumer_pid: row.get(4)?,
                data_address: row.get(5)?,
            })
        })?;
        Ok(Some(result))
    }

    fn delete_callback(&self, callback_id: Uuid) -> anyhow::Result<()> {
        let conn = self.client.get()?;
        let id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();

        #[derive(Serialize, Deserialize)]
        struct Test {
            a: i32,
        }
        let transaction = conn.execute(
            r#"
            INSERT INTO consumer.transfer_callbacks
            VALUES (?, ?, null, null, null, ?)
        "#,
            params![
                id.to_string(),
                now.to_string(),
                serde_json::to_string(&Test { a: 0 })?
            ],
        )?;
        let mut transaction = conn.prepare(
            r#"
            SELECT * FROM consumer.transfer_callbacks
            WHERE id = ?
            LIMIT 1
        "#,
        )?;
        let result = transaction.query_row(params![id.to_string()], |row| {
            Ok(TransferCallbacksModel {
                id: row.get(0)?,
                created_at: DateTime::from_timestamp_micros(row.get(1)?)
                    .unwrap()
                    .naive_utc(),
                updated_at: match row.get(2)? {
                    Null => None,
                    _ => Option::from(
                        DateTime::from_timestamp_micros(row.get(2)?)
                            .unwrap()
                            .naive_utc(),
                    ),
                },
                provider_pid: row.get(3)?,
                consumer_pid: row.get(4)?,
                data_address: row.get(5)?,
            })
        });
        Ok(())
    }
}
