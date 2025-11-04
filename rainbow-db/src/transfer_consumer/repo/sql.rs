/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::transfer_consumer::entities::transfer_callback;
use crate::transfer_consumer::entities::transfer_message;
use crate::transfer_consumer::repo::{
    EditTransferCallback, NewTransferCallback, TransferCallbackRepo, TransferConsumerRepoErrors,
    TransferConsumerRepoFactory, TransferMessagesConsumerRepo,
};
use crate::transfer_consumer::repo::{EditTransferMessageModel, NewTransferMessageModel};
use crate::transfer_consumer::transfer_process_projection::TransferConsumerProcessFromSQL;
use axum::async_trait;
use rainbow_common::utils::get_urn;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult,
    QueryFilter, QuerySelect, Statement,
};
use urn::Urn;

pub struct TransferConsumerRepoForSql {
    db_connection: DatabaseConnection,
}

impl TransferConsumerRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl TransferConsumerRepoFactory for TransferConsumerRepoForSql {
    fn create_repo(db_connection: DatabaseConnection) -> Self {
        Self::new(db_connection)
    }
}

#[async_trait]
impl TransferCallbackRepo for TransferConsumerRepoForSql {
    // async fn get_all_transfer_callbacks(
    //     &self,
    //     limit: Option<u64>,
    //     page: Option<u64>,
    // ) -> anyhow::Result<Vec<transfer_callback::Model>, TransferConsumerRepoErrors> {
    //     let transfer_callbacks = transfer_callback::Entity::find()
    //         .limit(limit.unwrap_or(100000))
    //         .offset(page.unwrap_or(0))
    //         .all(&self.db_connection)
    //         .await;
    //     match transfer_callbacks {
    //         Ok(transfer_callbacks) => Ok(transfer_callbacks),
    //         Err(e) => Err(TransferConsumerRepoErrors::ErrorFetchingConsumerTransferProcess(e.into())),
    //     }
    // }

    async fn get_all_transfer_callbacks(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<TransferConsumerProcessFromSQL>, TransferConsumerRepoErrors> {
        let sql = r#"
            WITH RankedMessages AS (
                SELECT
                    m.transfer_process_id,
                    m.message_type,
                    m.from,
                    m.created_at,
                    ROW_NUMBER() OVER(PARTITION BY m.transfer_process_id ORDER BY m.created_at DESC) as rn
                FROM
                    transfer_messages m
            )
            SELECT
                p.*,
                rm.message_type,
                rm.from,
                rm.created_at AS "message_at"
            FROM
                transfer_callbacks p
                    LEFT JOIN
                RankedMessages rm ON p.id = rm.transfer_process_id
            WHERE
                rm.rn = 1
            ORDER BY
                p.created_at DESC;
        "#;
        let stmt = Statement::from_string(DbBackend::Postgres, sql.to_owned());
        let transfer_processes = TransferConsumerProcessFromSQL::find_by_statement(stmt)
            .all(&self.db_connection)
            .await
            .map_err(|err| TransferConsumerRepoErrors::ErrorFetchingConsumerTransferProcess(err.into()))?;

        Ok(transfer_processes)
    }

    async fn get_batch_transfer_processes(
        &self,
        transfer_ids: &Vec<Urn>,
    ) -> Result<Vec<TransferConsumerProcessFromSQL>, TransferConsumerRepoErrors> {
        let transfer_ids = transfer_ids.iter().map(|t| t.to_string()).collect::<Vec<_>>();
        let sql = r#"
            WITH RankedMessages AS (
                SELECT
                    m.transfer_process_id,
                    m.message_type,
                    m.from,
                    m.created_at,
                    ROW_NUMBER() OVER(PARTITION BY m.transfer_process_id ORDER BY m.created_at DESC) as rn
                FROM
                    transfer_messages m
            )
            SELECT
                p.*,
                rm.message_type,
                rm.from,
                rm.created_at AS "message_at"
            FROM
                transfer_callbacks p
                    LEFT JOIN
                RankedMessages rm ON p.id = rm.transfer_process_id
            WHERE
                rm.rn = 1
                AND
                p.id = ANY($1)
            ORDER BY
                p.created_at DESC;
        "#;
        let stmt = Statement::from_sql_and_values(DbBackend::Postgres, sql.to_owned(), [transfer_ids.into()]);
        let transfer_processes = TransferConsumerProcessFromSQL::find_by_statement(stmt)
            .all(&self.db_connection)
            .await
            .map_err(|err| TransferConsumerRepoErrors::ErrorFetchingConsumerTransferProcess(err.into()))?;

        Ok(transfer_processes)
    }

    // async fn get_transfer_callbacks_by_id(
    //     &self,
    //     callback_id: Urn,
    // ) -> anyhow::Result<Option<transfer_callback::Model>, TransferConsumerRepoErrors> {
    //     let callback_id = callback_id.to_string();
    //     let transfer_callback = transfer_callback::Entity::find_by_id(callback_id).one(&self.db_connection).await;
    //     match transfer_callback {
    //         Ok(transfer_callback) => Ok(transfer_callback),
    //         Err(e) => Err(TransferConsumerRepoErrors::ErrorFetchingConsumerTransferProcess(e.into())),
    //     }
    // }

    async fn get_transfer_callbacks_by_id(
        &self,
        callback_id: Urn,
    ) -> anyhow::Result<Option<TransferConsumerProcessFromSQL>, TransferConsumerRepoErrors> {
        let sql = r#"
            WITH RankedMessages AS (
                SELECT
                    m.transfer_process_id,
                    m.message_type,
                    m.from,
                    m.created_at,
                    ROW_NUMBER() OVER(PARTITION BY m.transfer_process_id ORDER BY m.created_at DESC) as rn
                FROM
                    transfer_messages m
            )
            SELECT
                p.*,
                rm.message_type,
                rm.from,
                rm.created_at AS "message_at"
            FROM
                transfer_callbacks p
                    LEFT JOIN
                RankedMessages rm ON p.id = rm.transfer_process_id
            WHERE
                p.id = $1
                AND
                rm.rn = 1
            ORDER BY
                p.created_at DESC;
        "#;
        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql.to_owned(),
            [callback_id.to_string().into()],
        );
        let transfer_processes = TransferConsumerProcessFromSQL::find_by_statement(stmt)
            .one(&self.db_connection)
            .await
            .map_err(|err| TransferConsumerRepoErrors::ErrorFetchingConsumerTransferProcess(err.into()))?;

        Ok(transfer_processes)
    }

    // async fn get_transfer_callback_by_consumer_id(
    //     &self,
    //     consumer_pid: Urn,
    // ) -> anyhow::Result<Option<transfer_callback::Model>, TransferConsumerRepoErrors> {
    //     let consumer_pid = consumer_pid.to_string();
    //     let transfer_callback = transfer_callback::Entity::find()
    //         .filter(transfer_callback::Column::ConsumerPid.eq(consumer_pid))
    //         .one(&self.db_connection)
    //         .await;
    //     match transfer_callback {
    //         Ok(transfer_callback) => Ok(transfer_callback),
    //         Err(e) => Err(TransferConsumerRepoErrors::ErrorFetchingConsumerTransferProcess(e.into())),
    //     }
    // }

    async fn get_transfer_callback_by_consumer_id(
        &self,
        consumer_pid: Urn,
    ) -> anyhow::Result<Option<TransferConsumerProcessFromSQL>, TransferConsumerRepoErrors> {
        let sql = r#"
            WITH RankedMessages AS (
                SELECT
                    m.transfer_process_id,
                    m.message_type,
                    m.from,
                    m.created_at,
                    ROW_NUMBER() OVER(PARTITION BY m.transfer_process_id ORDER BY m.created_at DESC) as rn
                FROM
                    transfer_messages m
            )
            SELECT
                p.*,
                rm.message_type,
                rm.from,
                rm.created_at AS "message_at"
            FROM
                transfer_callbacks p
                    LEFT JOIN
                RankedMessages rm ON p.id = rm.transfer_process_id
            WHERE
                p.consumer_pid = $1 AND
                rm.rn = 1
            ORDER BY
                p.created_at DESC;
        "#;
        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql.to_owned(),
            [consumer_pid.to_string().into()],
        );
        let transfer_processes = TransferConsumerProcessFromSQL::find_by_statement(stmt)
            .one(&self.db_connection)
            .await
            .map_err(|err| TransferConsumerRepoErrors::ErrorFetchingConsumerTransferProcess(err.into()))?;

        Ok(transfer_processes)
    }

    // async fn get_transfer_callback_by_provider_id(
    //     &self,
    //     provider_id: Urn,
    // ) -> anyhow::Result<Option<transfer_callback::Model>, TransferConsumerRepoErrors> {
    //     let consumer_pid = provider_id.to_string();
    //     let transfer_callback = transfer_callback::Entity::find()
    //         .filter(transfer_callback::Column::ProviderPid.eq(consumer_pid))
    //         .one(&self.db_connection)
    //         .await;
    //     match transfer_callback {
    //         Ok(transfer_callback) => Ok(transfer_callback),
    //         Err(e) => Err(TransferConsumerRepoErrors::ErrorFetchingConsumerTransferProcess(e.into())),
    //     }
    // }

    async fn get_transfer_callback_by_provider_id(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Option<TransferConsumerProcessFromSQL>, TransferConsumerRepoErrors> {
        let sql = r#"
            WITH RankedMessages AS (
                SELECT
                    m.transfer_process_id,
                    m.message_type,
                    m.from,
                    m.created_at,
                    ROW_NUMBER() OVER(PARTITION BY m.transfer_process_id ORDER BY m.created_at DESC) as rn
                FROM
                    transfer_messages m
            )
            SELECT
                p.*,
                rm.message_type,
                rm.from,
                rm.created_at AS "message_at"
            FROM
                transfer_callbacks p
                    LEFT JOIN
                RankedMessages rm ON p.id = rm.transfer_process_id
            WHERE
                p.provider_pid = $1 AND
                rm.rn = 1
            ORDER BY
                p.created_at DESC;
        "#;
        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql.to_owned(),
            [provider_id.to_string().into()],
        );
        let transfer_processes = TransferConsumerProcessFromSQL::find_by_statement(stmt)
            .one(&self.db_connection)
            .await
            .map_err(|err| TransferConsumerRepoErrors::ErrorFetchingConsumerTransferProcess(err.into()))?;

        Ok(transfer_processes)
    }

    async fn put_transfer_callback(
        &self,
        callback_id: Urn,
        new_transfer_callback: EditTransferCallback,
    ) -> anyhow::Result<transfer_callback::Model, TransferConsumerRepoErrors> {
        let callback_id = callback_id.to_string();
        let old_model = transfer_callback::Entity::find_by_id(callback_id).one(&self.db_connection).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(TransferConsumerRepoErrors::ConsumerTransferProcessNotFound),
            },
            Err(e) => return Err(TransferConsumerRepoErrors::ErrorFetchingConsumerTransferProcess(e.into())),
        };

        let mut old_active_model: transfer_callback::ActiveModel = old_model.into();
        if let Some(provider_pid) = new_transfer_callback.provider_pid {
            old_active_model.provider_pid = ActiveValue::Set(Option::from(provider_pid.to_string()));
        }
        if let Some(consumer_pid) = new_transfer_callback.consumer_pid {
            old_active_model.consumer_pid = ActiveValue::Set(consumer_pid.to_string());
        }
        if let Some(data_plane_id) = new_transfer_callback.data_plane_id {
            old_active_model.data_plane_id = ActiveValue::Set(Option::from(data_plane_id.to_string()));
        }
        if let Some(data_address) = new_transfer_callback.data_address {
            old_active_model.data_address = ActiveValue::Set(Option::from(data_address));
        }
        if let Some(restart_flag) = new_transfer_callback.restart_flag {
            old_active_model.restart_flag = ActiveValue::Set(restart_flag);
        }
        old_active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(e) => Err(TransferConsumerRepoErrors::ErrorUpdatingConsumerTransferProcess(e.into())),
        }
    }

    async fn put_transfer_callback_by_consumer(
        &self,
        consumer_pid: Urn,
        new_transfer_callback: EditTransferCallback,
    ) -> anyhow::Result<transfer_callback::Model, TransferConsumerRepoErrors> {
        let consumer_pid = consumer_pid.to_string();
        let old_model = transfer_callback::Entity::find()
            .filter(transfer_callback::Column::ConsumerPid.eq(consumer_pid))
            .one(&self.db_connection)
            .await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(TransferConsumerRepoErrors::ConsumerTransferProcessNotFound),
            },
            Err(e) => return Err(TransferConsumerRepoErrors::ErrorFetchingConsumerTransferProcess(e.into())),
        };

        let mut old_active_model: transfer_callback::ActiveModel = old_model.into();
        if let Some(provider_pid) = new_transfer_callback.provider_pid {
            old_active_model.provider_pid = ActiveValue::Set(Option::from(provider_pid.to_string()));
        }
        if let Some(consumer_pid) = new_transfer_callback.consumer_pid {
            old_active_model.consumer_pid = ActiveValue::Set(consumer_pid.to_string());
        }
        if let Some(data_plane_id) = new_transfer_callback.data_plane_id {
            old_active_model.data_plane_id = ActiveValue::Set(Option::from(data_plane_id.to_string()));
        }
        if let Some(data_address) = new_transfer_callback.data_address {
            old_active_model.data_address = ActiveValue::Set(Option::from(data_address));
        }
        if let Some(restart_flag) = new_transfer_callback.restart_flag {
            old_active_model.restart_flag = ActiveValue::Set(restart_flag);
        }
        old_active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(e) => Err(TransferConsumerRepoErrors::ErrorUpdatingConsumerTransferProcess(e.into())),
        }
    }

    async fn create_transfer_callback(
        &self,
        new_transfer_callback: NewTransferCallback,
    ) -> anyhow::Result<transfer_callback::Model, TransferConsumerRepoErrors> {
        let consumer_pid = new_transfer_callback.consumer_pid.map(|p| p);
        let provider_pid = new_transfer_callback.provider_pid.map(|p| p.to_string());
        let callback_id = new_transfer_callback.callback_id.map(|p| p);
        let model = transfer_callback::ActiveModel {
            id: ActiveValue::Set(get_urn(callback_id).to_string()),
            consumer_pid: ActiveValue::Set(get_urn(consumer_pid).to_string()),
            provider_pid: ActiveValue::Set(provider_pid),
            associated_provider: ActiveValue::Set(new_transfer_callback.associated_provider.map(|a| a.to_string())),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(None),
            data_plane_id: ActiveValue::Set(None),
            data_address: ActiveValue::Set(new_transfer_callback.data_address),
            restart_flag: ActiveValue::Set(false),
        };
        let transfer_callback = transfer_callback::Entity::insert(model).exec_with_returning(&self.db_connection).await;

        match transfer_callback {
            Ok(transfer_callback) => Ok(transfer_callback),
            Err(e) => Err(TransferConsumerRepoErrors::ErrorCreatingConsumerTransferProcess(e.into())),
        }
    }

    async fn delete_transfer_callback(&self, callback_id: Urn) -> anyhow::Result<(), TransferConsumerRepoErrors> {
        let callback_id = callback_id.to_string();
        let transfer_callback = transfer_callback::Entity::delete_by_id(callback_id).exec(&self.db_connection).await;
        match transfer_callback {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(TransferConsumerRepoErrors::ConsumerTransferProcessNotFound),
                _ => Ok(()),
            },
            Err(e) => Err(TransferConsumerRepoErrors::ErrorDeletingConsumerTransferProcess(e.into())),
        }
    }
}

#[async_trait]
impl TransferMessagesConsumerRepo for TransferConsumerRepoForSql {
    async fn get_all_transfer_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_message::Model>, TransferConsumerRepoErrors> {
        let transfer_message = transfer_message::Entity::find()
            .limit(limit.unwrap_or(100000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match transfer_message {
            Ok(transfer_message) => Ok(transfer_message),
            Err(e) => Err(TransferConsumerRepoErrors::ErrorFetchingConsumerTransferMessage(e.into())),
        }
    }

    async fn get_all_transfer_messages_by_consumer(
        &self,
        pid: Urn,
    ) -> anyhow::Result<Vec<transfer_message::Model>, TransferConsumerRepoErrors> {
        let transfer_process = self
            .get_transfer_callbacks_by_id(pid.clone())
            .await
            .map_err(|e| TransferConsumerRepoErrors::ErrorFetchingConsumerTransferProcess(e.into()))?
            .ok_or(TransferConsumerRepoErrors::ConsumerTransferProcessNotFound)?;
        let transfer_message = transfer_message::Entity::find()
            .filter(transfer_message::Column::TransferProcessId.eq(pid.to_string()))
            .all(&self.db_connection)
            .await;
        match transfer_message {
            Ok(transfer_message) => Ok(transfer_message),
            Err(e) => Err(TransferConsumerRepoErrors::ErrorFetchingConsumerTransferMessage(e.into())),
        }
    }

    async fn get_transfer_message_by_id(
        &self,
        pid: Urn,
        mid: Urn,
    ) -> anyhow::Result<Option<transfer_message::Model>, TransferConsumerRepoErrors> {
        let transfer_process = self
            .get_transfer_callbacks_by_id(pid.clone())
            .await
            .map_err(|e| TransferConsumerRepoErrors::ErrorFetchingConsumerTransferProcess(e.into()))?
            .ok_or(TransferConsumerRepoErrors::ConsumerTransferProcessNotFound)?;

        let transfer_message = transfer_message::Entity::find_by_id(mid.to_string()).one(&self.db_connection).await;
        match transfer_message {
            Ok(transfer_message) => Ok(transfer_message),
            Err(e) => Err(TransferConsumerRepoErrors::ErrorFetchingConsumerTransferMessage(e.into())),
        }
    }

    async fn put_transfer_message(
        &self,
        pid: Urn,
        edit_transfer_message: EditTransferMessageModel,
    ) -> anyhow::Result<Option<transfer_message::Model>, TransferConsumerRepoErrors> {
        Ok(None)
    }

    async fn create_transfer_message(
        &self,
        pid: Urn,
        new_transfer_message: NewTransferMessageModel,
    ) -> anyhow::Result<transfer_message::Model, TransferConsumerRepoErrors> {
        let pid = pid.to_string();

        let model = transfer_message::ActiveModel {
            id: ActiveValue::Set(get_urn(None).to_string()),
            transfer_process_id: ActiveValue::Set(pid),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            message_type: ActiveValue::Set(new_transfer_message.message_type),
            from: ActiveValue::Set(new_transfer_message.from.to_string()),
            to: ActiveValue::Set(new_transfer_message.to.to_string()),
            content: ActiveValue::Set(new_transfer_message.content),
        };

        let model = transfer_message::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(e) => Err(TransferConsumerRepoErrors::ErrorCreatingConsumerTransferMessage(e.into())),
        }
    }

    async fn delete_transfer_message(&self, pid: Urn) -> anyhow::Result<(), TransferConsumerRepoErrors> {
        let pid = pid.to_string();

        let transfer_message = transfer_message::Entity::delete_by_id(pid).exec(&self.db_connection).await;
        match transfer_message {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(TransferConsumerRepoErrors::ConsumerTransferMessageNotFound),
                _ => Ok(()),
            },
            Err(e) => Err(TransferConsumerRepoErrors::ErrorDeletingConsumerTransferMessage(e.into())),
        }
    }
}
