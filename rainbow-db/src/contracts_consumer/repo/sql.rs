/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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

use super::super::entities::cn_process;
use crate::contracts_consumer::cn_process_projection::CnConsumerProcessFromSQL;
use crate::contracts_consumer::entities::{agreement, cn_message, cn_offer};
use crate::contracts_consumer::repo::{
    AgreementConsumerRepo, CnErrors, ContractNegotiationConsumerMessageRepo, ContractNegotiationConsumerOfferRepo,
    ContractNegotiationConsumerProcessRepo, ContractNegotiationConsumerRepoFactory, EditAgreement,
    EditContractNegotiationMessage, EditContractNegotiationOffer, EditContractNegotiationProcess, NewAgreement,
    NewContractNegotiationMessage, NewContractNegotiationOffer, NewContractNegotiationProcess,
};
use axum::async_trait;
use cn_message::Model;
use json_value_merge::Merge;
use rainbow_common::utils::{get_urn, get_urn_from_string};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, JoinType, ModelTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Statement};
use serde_json::to_value;
use urn::Urn;

pub struct ContractNegotiationConsumerRepoForSql {
    db_connection: DatabaseConnection,
}

impl ContractNegotiationConsumerRepoForSql {
    fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl ContractNegotiationConsumerRepoFactory for ContractNegotiationConsumerRepoForSql {
    fn create_repo(database_connection: DatabaseConnection) -> Self {
        Self::new(database_connection)
    }
}

#[async_trait]
impl ContractNegotiationConsumerProcessRepo for ContractNegotiationConsumerRepoForSql {
    async fn get_all_cn_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<CnConsumerProcessFromSQL>, CnErrors> {
        let sql = r#"
            WITH RankedMessages AS (
                SELECT
                    m.cn_process_id,
                    m.type,
                    m.subtype,
                    m.created_at,
                    ROW_NUMBER() OVER(PARTITION BY m.cn_process_id ORDER BY m.created_at DESC) as rn
                FROM
                    cn_messages m
            )
            SELECT
                p.consumer_id,
                p.provider_id,
                p.associated_provider,
                p.is_business,
                p.created_at,
                p.updated_at,
                rm.type AS "message_type",
                rm.subtype AS "message_subtype",
                rm.created_at AS "message_at"
            FROM
                cn_processes p
                    LEFT JOIN
                RankedMessages rm ON p.consumer_id = rm.cn_process_id
            WHERE
                rm.rn = 1
            ORDER BY
                p.created_at DESC;
        "#;
        let stmt = Statement::from_string(DbBackend::Postgres, sql.to_owned());
        let cn_processes = CnConsumerProcessFromSQL::find_by_statement(stmt)
            .all(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?;

        Ok(cn_processes)
    }

    async fn get_cn_process_by_provider_id(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Option<CnConsumerProcessFromSQL>, CnErrors> {
        let sql = r#"
            WITH RankedMessages AS (
                SELECT
                    m.cn_process_id,
                    m.type,
                    m.subtype,
                    m.created_at,
                    ROW_NUMBER() OVER(PARTITION BY m.cn_process_id ORDER BY m.created_at DESC) as rn
                FROM
                    cn_messages m
            )
            SELECT
                p.consumer_id,
                p.provider_id,
                p.associated_provider,
                p.is_business,
                p.created_at,
                p.updated_at,
                rm.type AS "message_type",
                rm.subtype AS "message_subtype",
                rm.created_at AS "message_at"
            FROM
                cn_processes p
                    LEFT JOIN
                RankedMessages rm ON p.consumer_id = rm.cn_process_id
            WHERE
                p.provider_id = $1 AND
                rm.rn = 1
            ORDER BY
                p.created_at DESC;
        "#;
        let stmt = Statement::from_sql_and_values(DbBackend::Postgres, sql.to_owned(), [provider_id.to_string().into()]);
        let cn_processes = CnConsumerProcessFromSQL::find_by_statement(stmt)
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?;

        Ok(cn_processes)
    }

    async fn get_cn_process_by_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<Option<CnConsumerProcessFromSQL>, CnErrors> {
        let sql = r#"
            WITH RankedMessages AS (
                SELECT
                    m.cn_process_id,
                    m.type,
                    m.subtype,
                    m.created_at,
                    ROW_NUMBER() OVER(PARTITION BY m.cn_process_id ORDER BY m.created_at DESC) as rn
                FROM
                    cn_messages m
            )
            SELECT
                p.consumer_id,
                p.provider_id,
                p.associated_provider,
                p.is_business,
                p.created_at,
                p.updated_at,
                rm.type AS "message_type",
                rm.subtype AS "message_subtype",
                rm.created_at AS "message_at"
            FROM
                cn_processes p
                    LEFT JOIN
                RankedMessages rm ON p.consumer_id = rm.cn_process_id
            WHERE
                p.consumer_id = $1 AND
                rm.rn = 1
            ORDER BY
                p.created_at DESC;
        "#;
        let stmt = Statement::from_sql_and_values(DbBackend::Postgres, sql.to_owned(), [consumer_id.to_string().into()]);
        let cn_processes = CnConsumerProcessFromSQL::find_by_statement(stmt)
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?;

        Ok(cn_processes)
    }

    async fn get_cn_process_by_cn_id(
        &self,
        cn_process_id: Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors> {
        let response = cn_process::Entity::find_by_id(cn_process_id.to_string())
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?;
        Ok(response)
    }

    async fn put_cn_process(
        &self,
        cn_process_id: Urn,
        edit_cn_process: EditContractNegotiationProcess,
    ) -> anyhow::Result<CnConsumerProcessFromSQL, CnErrors> {
        let old_model = cn_process::Entity::find_by_id(cn_process_id.as_str())
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let mut old_active_model: cn_process::ActiveModel = old_model.into();
        old_active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model
            .update(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorUpdatingCNProcess(err.into()))?;

        let urn = get_urn_from_string(&model.consumer_id)
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?;
        let model_out = self.get_cn_process_by_consumer_id(urn).await?
            .ok_or(CnErrors::CNProcessNotFound)?;

        Ok(model_out)
    }

    async fn create_cn_process(
        &self,
        new_cn_process: NewContractNegotiationProcess,
    ) -> anyhow::Result<cn_process::Model, CnErrors> {
        let model = cn_process::ActiveModel {
            consumer_id: ActiveValue::Set(new_cn_process.consumer_id.unwrap_or(get_urn(None)).to_string()),
            provider_id: ActiveValue::Set(Option::from(
                get_urn(new_cn_process.provider_id).to_string(),
            )),
            associated_provider: ActiveValue::Set(new_cn_process.associated_provider.map(|a| a.to_string())),
            is_business: ActiveValue::Set(new_cn_process.is_business),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(None),
        };

        let cn_process = cn_process::Entity::insert(model)
            .exec_with_returning(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorCreatingCNProcess(err.into()))?;

        Ok(cn_process)
    }

    async fn delete_cn_process(&self, cn_process_id: Urn) -> anyhow::Result<(), CnErrors> {
        match cn_process::Entity::delete_by_id(cn_process_id.as_str())
            .exec(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorDeletingCNProcess(err.into()))?
            .rows_affected
        {
            0 => Err(CnErrors::CNProcessNotFound),
            _ => Ok(()),
        }
    }
}

#[async_trait]
impl ContractNegotiationConsumerMessageRepo for ContractNegotiationConsumerRepoForSql {
    async fn get_all_cn_messages(&self, limit: Option<u64>, page: Option<u64>) -> anyhow::Result<Vec<Model>, CnErrors> {
        let cn_processes = cn_message::Entity::find()
            .limit(limit.unwrap_or(10000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?;
        Ok(cn_processes)
    }

    async fn get_cn_messages_by_cn_process_id(&self, cn_process_id: Urn) -> anyhow::Result<Vec<Model>, CnErrors> {
        let cn_process = self
            .get_cn_process_by_cn_id(cn_process_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;
        let cn_message = cn_message::Entity::find()
            .filter(cn_message::Column::CnProcessId.eq(cn_process.consumer_id.as_str()))
            .all(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?;
        Ok(cn_message)
    }

    async fn get_cn_messages_by_cn_message_id(&self, cn_message_id: Urn) -> anyhow::Result<Option<Model>, CnErrors> {
        let cn_message = cn_message::Entity::find_by_id(cn_message_id.as_str())
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?;
        Ok(cn_message)
    }

    async fn get_cn_messages_by_provider_id(&self, provider_id: Urn) -> anyhow::Result<Vec<Model>, CnErrors> {
        let cn_process = self
            .get_cn_process_by_cn_id(provider_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_messages = cn_message::Entity::find()
            .join(JoinType::InnerJoin, cn_message::Relation::CnProcesses.def())
            .filter(cn_process::Column::ProviderId.eq(cn_process.provider_id.unwrap().as_str()))
            .all(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?;
        Ok(cn_messages)
    }

    async fn get_cn_messages_by_consumer_id(&self, consumer_id: Urn) -> anyhow::Result<Vec<Model>, CnErrors> {
        let cn_process = self
            .get_cn_process_by_cn_id(consumer_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;
        let cn_messages = cn_message::Entity::find()
            .join(JoinType::InnerJoin, cn_message::Relation::CnProcesses.def())
            .filter(cn_process::Column::ConsumerId.eq(cn_process.consumer_id.as_str()))
            .all(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?;
        Ok(cn_messages)
    }

    async fn put_cn_message(
        &self,
        cn_process_id: Urn,
        cn_message_id: Urn,
        edit_cn_message: EditContractNegotiationMessage,
    ) -> anyhow::Result<Model, CnErrors> {
        let cn_process = self
            .get_cn_process_by_cn_id(cn_process_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let old_model = cn_message::Entity::find_by_id(cn_message_id.as_str())
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?
            .ok_or(CnErrors::CNMessageNotFound)?;

        let mut old_active_model: cn_message::ActiveModel = old_model.into();
        if let Some(_type) = edit_cn_message._type {
            old_active_model._type = ActiveValue::Set(_type);
        }
        if let Some(from) = edit_cn_message.from {
            old_active_model.from = ActiveValue::Set(from);
        }
        if let Some(to) = edit_cn_message.to {
            old_active_model.to = ActiveValue::Set(to);
        }
        let mut old_json_content = old_active_model.content.unwrap();
        let new_json_content = edit_cn_message.content.unwrap();
        old_json_content.merge(&new_json_content);
        old_active_model.content = ActiveValue::Set(old_json_content);

        let model = old_active_model
            .update(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorUpdatingCNMessage(err.into()))?;
        Ok(model)
    }

    async fn create_cn_message(
        &self,
        cn_process_id: Urn,
        new_cn_message: NewContractNegotiationMessage,
    ) -> anyhow::Result<Model, CnErrors> {
        let cn_process = self
            .get_cn_process_by_cn_id(cn_process_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let model = cn_message::ActiveModel {
            cn_message_id: ActiveValue::Set(get_urn(None).to_string()),
            cn_process_id: ActiveValue::Set(cn_process_id.to_string()),
            _type: ActiveValue::Set(new_cn_message._type),
            subtype: ActiveValue::Set(new_cn_message.subtype),
            from: ActiveValue::Set(new_cn_message.from),
            to: ActiveValue::Set(new_cn_message.to),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            content: ActiveValue::Set(new_cn_message.content),
        };

        let cn_process = cn_message::Entity::insert(model)
            .exec_with_returning(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorCreatingCNMessage(err.into()))?;
        Ok(cn_process)
    }

    async fn delete_cn_message(&self, cn_process_id: Urn, cn_message_id: Urn) -> anyhow::Result<(), CnErrors> {
        let cn_process = self
            .get_cn_process_by_cn_id(cn_process_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        match cn_message::Entity::delete_by_id(cn_message_id.as_str())
            .exec(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorDeletingCNMessage(err.into()))?
            .rows_affected
        {
            0 => Err(CnErrors::CNMessageNotFound),
            _ => Ok(()),
        }
    }
}

#[async_trait]
impl ContractNegotiationConsumerOfferRepo for ContractNegotiationConsumerRepoForSql {
    async fn get_all_cn_offers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors> {
        let cn_offers = cn_offer::Entity::find()
            .limit(limit.unwrap_or(10000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?;
        Ok(cn_offers)
    }

    async fn get_all_cn_offers_by_message_id(
        &self,
        message_id: Urn,
    ) -> anyhow::Result<Option<cn_offer::Model>, CnErrors> {
        let cn_message = self
            .get_cn_messages_by_cn_message_id(message_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?
            .ok_or(CnErrors::CNMessageNotFound)?;

        let cn_offer = cn_offer::Entity::find()
            .filter(cn_offer::Column::CnMessageId.eq(cn_message.cn_message_id))
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?;
        Ok(cn_offer)
    }

    async fn get_all_cn_offers_by_provider(&self, provider_id: Urn) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors> {
        let cn_offers = cn_offer::Entity::find()
            .join(JoinType::InnerJoin, cn_message::Relation::CnProcesses.def())
            .join(JoinType::InnerJoin, cn_offer::Relation::CnMessage.def())
            .filter(cn_process::Column::ProviderId.eq(provider_id.as_str()))
            .all(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?;
        Ok(cn_offers)
    }

    async fn get_all_cn_offers_by_cn_process(&self, process_id: Urn) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors> {
        let cn_process = self
            .get_cn_process_by_cn_id(process_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_offers = cn_offer::Entity::find()
            .join(JoinType::InnerJoin, cn_offer::Relation::CnMessage.def())
            .filter(cn_message::Column::CnProcessId.eq(cn_process.consumer_id.as_str()))
            .all(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?;
        Ok(cn_offers)
    }

    async fn get_all_cn_offers_by_consumer(&self, consumer_id: Urn) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors> {
        let cn_offers = cn_offer::Entity::find()
            .join(JoinType::InnerJoin, cn_message::Relation::CnProcesses.def())
            .join(JoinType::InnerJoin, cn_offer::Relation::CnMessage.def())
            .filter(cn_process::Column::ConsumerId.eq(consumer_id.as_str()))
            .all(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?;
        Ok(cn_offers)
    }

    async fn get_last_cn_offers_by_cn_process(
        &self,
        process_id: Urn,
    ) -> anyhow::Result<Option<cn_offer::Model>, CnErrors> {
        let cn_process = self
            .get_cn_process_by_cn_id(process_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_offers = cn_offer::Entity::find()
            .join(JoinType::InnerJoin, cn_offer::Relation::CnMessage.def())
            .filter(cn_message::Column::CnProcessId.eq(cn_process.consumer_id.as_str()))
            .order_by_desc(cn_offer::Column::CreatedAt)
            .limit(1)
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?;
        Ok(cn_offers)
    }

    async fn get_cn_offer_by_id(&self, offer_id: Urn) -> anyhow::Result<Option<cn_offer::Model>, CnErrors> {
        let cn_offers = cn_offer::Entity::find_by_id(offer_id.as_str())
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?;
        Ok(cn_offers)
    }

    async fn put_cn_offer(
        &self,
        process_id: Urn,
        message_id: Urn,
        offer_id: Urn,
        edit_cn_offer: EditContractNegotiationOffer,
    ) -> anyhow::Result<cn_offer::Model, CnErrors> {
        let cn_process = self
            .get_cn_process_by_consumer_id(process_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_message = self
            .get_cn_messages_by_cn_message_id(message_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?
            .ok_or(CnErrors::CNMessageNotFound)?;

        let cn_offer = self
            .get_cn_offer_by_id(offer_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?
            .ok_or(CnErrors::CNOfferNotFound)?;

        let old_model = cn_offer::Entity::find_by_id(offer_id.as_str())
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?
            .ok_or(CnErrors::CNOfferNotFound)?;

        let mut old_active_model: cn_offer::ActiveModel = old_model.into();

        let model = old_active_model
            .update(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorUpdatingCNOffer(err.into()))?;
        Ok(model)
    }

    async fn create_cn_offer(
        &self,
        process_id: Urn,
        message_id: Urn,
        new_cn_offer: NewContractNegotiationOffer,
    ) -> anyhow::Result<cn_offer::Model, CnErrors> {
        let cn_process = self
            .get_cn_process_by_consumer_id(process_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_message = self
            .get_cn_messages_by_cn_message_id(message_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?
            .ok_or(CnErrors::CNMessageNotFound)?;

        let model = cn_offer::ActiveModel {
            offer_id: ActiveValue::Set(get_urn(None).to_string()),
            cn_message_id: ActiveValue::Set(message_id.to_string()),
            offer_content: ActiveValue::Set(new_cn_offer.offer_content),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        let cn_offer = cn_offer::Entity::insert(model)
            .exec_with_returning(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorCreatingCNOffer(err.into()))?;
        Ok(cn_offer)
    }

    async fn delete_cn_offer(&self, process_id: Urn, message_id: Urn, offer_id: Urn) -> anyhow::Result<(), CnErrors> {
        let cn_process = self
            .get_cn_process_by_cn_id(process_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_message = self
            .get_cn_messages_by_cn_message_id(message_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?
            .ok_or(CnErrors::CNMessageNotFound)?;

        match cn_offer::Entity::delete_by_id(offer_id.as_str())
            .exec(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorDeletingCNOffer(err.into()))?
            .rows_affected
        {
            0 => Err(CnErrors::CNOfferNotFound),
            _ => Ok(()),
        }
    }
}

#[async_trait]
impl AgreementConsumerRepo for ContractNegotiationConsumerRepoForSql {
    async fn get_all_agreements(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<agreement::Model>, CnErrors> {
        let agreements = agreement::Entity::find()
            .limit(limit.unwrap_or(10000))
            .offset(page.unwrap_or(0))
            .all(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingAgreement(err.into()))?;
        Ok(agreements)
    }

    async fn get_agreement_by_ag_id(&self, agreement_id: Urn) -> anyhow::Result<Option<agreement::Model>, CnErrors> {
        let agreement = agreement::Entity::find_by_id(agreement_id.as_str())
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingAgreement(err.into()))?;
        Ok(agreement)
    }

    async fn get_agreement_by_process_id(&self, process_id: Urn) -> anyhow::Result<Option<agreement::Model>, CnErrors> {
        let cn_process = self
            .get_cn_process_by_cn_id(process_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let agreement = agreement::Entity::find()
            .join(JoinType::InnerJoin, agreement::Relation::CnMessage.def())
            .filter(cn_message::Column::CnProcessId.eq(process_id.as_str()))
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingAgreement(err.into()))?;
        Ok(agreement)
    }

    async fn get_agreement_by_message_id(&self, message_id: Urn) -> anyhow::Result<Option<agreement::Model>, CnErrors> {
        let cn_message = self
            .get_cn_messages_by_cn_message_id(message_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?
            .ok_or(CnErrors::CNMessageNotFound)?;

        let agreement = agreement::Entity::find()
            .filter(agreement::Column::CnMessageId.eq(message_id.as_str()))
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingAgreement(err.into()))?;
        Ok(agreement)
    }

    async fn get_agreements_by_participant_id(
        &self,
        participant_id: Urn,
    ) -> anyhow::Result<Vec<agreement::Model>, CnErrors> {
        todo!()
    }

    async fn put_agreement(
        &self,
        process_id: Urn,
        message_id: Urn,
        agreement_id: Urn,
        edit_agreement: EditAgreement,
    ) -> anyhow::Result<agreement::Model, CnErrors> {
        let cn_process = self
            .get_cn_process_by_cn_id(process_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_message = self
            .get_cn_messages_by_cn_message_id(message_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?
            .ok_or(CnErrors::CNMessageNotFound)?;

        let agreement = self
            .get_agreement_by_ag_id(agreement_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingAgreement(err.into()))?
            .ok_or(CnErrors::AgreementNotFound)?;

        let old_model = agreement::Entity::find_by_id(agreement_id.as_str())
            .one(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingAgreement(err.into()))?
            .ok_or(CnErrors::AgreementNotFound)?;

        let mut old_active_model: agreement::ActiveModel = old_model.into();
        if let Some(active) = edit_agreement.active {
            old_active_model.active = ActiveValue::Set(active);
        }

        let model = old_active_model
            .update(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorUpdatingAgreement(err.into()))?;
        Ok(model)
    }

    async fn create_agreement(
        &self,
        process_id: Urn,
        message_id: Urn,
        new_agreement: NewAgreement,
    ) -> anyhow::Result<agreement::Model, CnErrors> {
        let cn_process = self
            .get_cn_process_by_cn_id(process_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_message = self
            .get_cn_messages_by_cn_message_id(message_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?
            .ok_or(CnErrors::CNMessageNotFound)?;

        // let consumer_participant = self
        //     .get_participant_by_p_id(new_agreement.consumer_participant_id.clone())
        //     .await
        //     .map_err(|err| CnErrors::ErrorFetchingParticipant(err.into()))?
        //     .ok_or(CnErrors::ParticipantNotFound(
        //         "Consumer".to_string(),
        //         new_agreement.consumer_participant_id.clone(),
        //     ))?;
        //
        // let provider_participant = self
        //     .get_participant_by_p_id(new_agreement.provider_participant_id.clone())
        //     .await
        //     .map_err(|err| CnErrors::ErrorFetchingParticipant(err.into()))?
        //     .ok_or(CnErrors::ParticipantNotFound(
        //         "Provider".to_string(),
        //         new_agreement.provider_participant_id.clone(),
        //     ))?;

        let agreement_as_json =
            to_value(new_agreement.agreement_content).map_err(|err| CnErrors::ErrorCreatingAgreement(err.into()))?;

        let agreement_id = new_agreement.agreement_id.map(|a| a.to_string()).unwrap_or(get_urn(None).to_string());
        let model = agreement::ActiveModel {
            agreement_id: ActiveValue::Set(agreement_id),
            consumer_participant_id: ActiveValue::Set(new_agreement.consumer_participant_id.to_string()),
            provider_participant_id: ActiveValue::Set(new_agreement.provider_participant_id.to_string()),
            cn_message_id: ActiveValue::Set(message_id.to_string()),
            agreement_content: ActiveValue::Set(agreement_as_json),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            active: ActiveValue::Set(true),
        };

        let agreement = agreement::Entity::insert(model)
            .exec_with_returning(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorCreatingAgreement(err.into()))?;
        Ok(agreement)
    }

    async fn delete_agreement(
        &self,
        process_id: Urn,
        message_id: Urn,
        agreement_id: Urn,
    ) -> anyhow::Result<(), CnErrors> {
        let cn_process = self
            .get_cn_process_by_cn_id(process_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_message = self
            .get_cn_messages_by_cn_message_id(message_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?
            .ok_or(CnErrors::CNMessageNotFound)?;

        let agreement = self
            .get_agreement_by_ag_id(agreement_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingAgreement(err.into()))?
            .ok_or(CnErrors::AgreementNotFound)?;

        match agreement::Entity::delete_by_id(agreement_id.as_str())
            .exec(&self.db_connection)
            .await
            .map_err(|err| CnErrors::ErrorDeletingAgreement(err.into()))?
            .rows_affected
        {
            0 => Err(CnErrors::AgreementNotFound),
            _ => Ok(()),
        }
    }
}
