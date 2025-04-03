use super::super::entities::agreement;
use super::super::entities::cn_message;
use super::super::entities::cn_offer;
use super::super::entities::cn_process;
use super::super::entities::participant;
use crate::contracts_provider::repo::{
    AgreementRepo, CnErrors, ContractNegotiationMessageRepo, ContractNegotiationOfferRepo,
    ContractNegotiationProcessRepo, EditAgreement, EditContractNegotiationMessage,
    EditContractNegotiationOffer, EditContractNegotiationProcess, EditParticipant, NewAgreement,
    NewContractNegotiationMessage, NewContractNegotiationOffer, NewContractNegotiationProcess,
    NewParticipant, Participant,
};
use json_value_merge::Merge;
use rainbow_common::config::database::get_db_connection;
use rainbow_common::utils::get_urn;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, JoinType, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};
use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::prelude::Condition;
use urn::Urn;


pub struct ContractNegotiationRepoForSql {}

#[async_trait]
impl ContractNegotiationProcessRepo for ContractNegotiationRepoForSql {
    async fn get_all_cn_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<cn_process::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_processes = cn_process::Entity::find()
            .limit(limit.unwrap_or(10000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?;
        Ok(cn_processes)
    }

    async fn get_cn_processes_by_provider_id(
        &self,
        provider_id: &Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_processes = cn_process::Entity::find()
            .filter(cn_process::Column::ProviderId.eq(provider_id.as_str()))
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?;
        Ok(cn_processes)
    }

    async fn get_cn_processes_by_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_processes = cn_process::Entity::find()
            .filter(cn_process::Column::ConsumerId.eq(consumer_id.as_str()))
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?;
        Ok(cn_processes)
    }

    async fn get_cn_process_by_cn_id(
        &self,
        cn_process_id: Urn,
    ) -> anyhow::Result<Option<cn_process::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_process = cn_process::Entity::find_by_id(cn_process_id.as_str())
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?;
        Ok(cn_process)
    }

    async fn put_cn_process(
        &self,
        cn_process_id: Urn,
        edit_cn_process: EditContractNegotiationProcess,
    ) -> anyhow::Result<cn_process::Model, CnErrors> {
        let db_connection = get_db_connection().await;
        let old_model = cn_process::Entity::find_by_id(cn_process_id.as_str())
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let mut old_active_model: cn_process::ActiveModel = old_model.into();

        if let Some(provider_id) = edit_cn_process.provider_id {
            old_active_model.provider_id = ActiveValue::Set(Some(provider_id.to_string()));
        }
        if let Some(consumer_id) = edit_cn_process.consumer_id {
            old_active_model.consumer_id = ActiveValue::Set(Some(consumer_id.to_string()));
        }
        if let Some(state) = edit_cn_process.state {
            old_active_model.state = ActiveValue::Set(state.to_string());
        }
        old_active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        let model = old_active_model
            .update(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorUpdatingCNProcess(err.into()))?;
        Ok(model)
    }

    async fn create_cn_process(
        &self,
        new_cn_process: NewContractNegotiationProcess,
    ) -> anyhow::Result<cn_process::Model, CnErrors> {
        let db_connection = get_db_connection().await;
        let model = cn_process::ActiveModel {
            cn_process_id: ActiveValue::Set(get_urn(None).to_string()),
            provider_id: ActiveValue::Set(Option::from(
                get_urn(new_cn_process.provider_id).to_string(),
            )),
            consumer_id: ActiveValue::Set(Option::from(
                get_urn(new_cn_process.consumer_id).to_string(),
            )),
            state: ActiveValue::Set(new_cn_process.state.to_string()),
            initiated_by: ActiveValue::Set(new_cn_process.initiated_by.to_string()),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(None),
        };

        let cn_process = cn_process::Entity::insert(model)
            .exec_with_returning(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorCreatingCNProcess(err.into()))?;
        Ok(cn_process)
    }

    async fn delete_cn_process(&self, cn_process_id: Urn) -> anyhow::Result<(), CnErrors> {
        let db_connection = get_db_connection().await;
        match cn_process::Entity::delete_by_id(cn_process_id.as_str())
            .exec(db_connection)
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
impl ContractNegotiationMessageRepo for ContractNegotiationRepoForSql {
    async fn get_all_cn_messages(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<cn_message::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_processes = cn_message::Entity::find()
            .limit(limit.unwrap_or(10000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?;
        Ok(cn_processes)
    }

    async fn get_cn_messages_by_cn_process_id(
        &self,
        cn_process_id: Urn,
    ) -> anyhow::Result<Vec<cn_message::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_process = self
            .get_cn_process_by_cn_id(cn_process_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_message = cn_message::Entity::find()
            .filter(cn_message::Column::CnProcessId.eq(cn_process.cn_process_id.as_str()))
            .all(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?;
        Ok(cn_message)
    }

    async fn get_cn_messages_by_cn_message_id(
        &self,
        cn_message_id: Urn,
    ) -> anyhow::Result<Option<cn_message::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_message = cn_message::Entity::find_by_id(cn_message_id.as_str())
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?;
        Ok(cn_message)
    }

    async fn get_cn_messages_by_provider_id(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Vec<cn_message::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_process = self
            .get_cn_processes_by_provider_id(&provider_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_messages = cn_message::Entity::find()
            .join(JoinType::InnerJoin, cn_message::Relation::CnProcess.def())
            .filter(cn_process::Column::ProviderId.eq(cn_process.provider_id.unwrap().as_str()))
            .all(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?;
        Ok(cn_messages)
    }

    async fn get_cn_messages_by_consumer_id(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<Vec<cn_message::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_process = self
            .get_cn_processes_by_consumer_id(consumer_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_messages = cn_message::Entity::find()
            .join(JoinType::InnerJoin, cn_message::Relation::CnProcess.def())
            .filter(cn_process::Column::ConsumerId.eq(cn_process.consumer_id.unwrap().as_str()))
            .all(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?;
        Ok(cn_messages)
    }

    async fn put_cn_message(
        &self,
        cn_process_id: Urn,
        cn_message_id: Urn,
        edit_cn_message: EditContractNegotiationMessage,
    ) -> anyhow::Result<cn_message::Model, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_process = self
            .get_cn_process_by_cn_id(cn_process_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let old_model = cn_message::Entity::find_by_id(cn_message_id.as_str())
            .one(db_connection)
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
            .update(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorUpdatingCNMessage(err.into()))?;
        Ok(model)
    }

    async fn create_cn_message(
        &self,
        cn_process_id: Urn,
        new_cn_message: NewContractNegotiationMessage,
    ) -> anyhow::Result<cn_message::Model, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_process = self
            .get_cn_process_by_cn_id(cn_process_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let model = cn_message::ActiveModel {
            cn_message_id: ActiveValue::Set(get_urn(None).to_string()),
            cn_process_id: ActiveValue::Set(cn_process_id.to_string()),
            _type: ActiveValue::Set(new_cn_message._type),
            from: ActiveValue::Set(new_cn_message.from),
            to: ActiveValue::Set(new_cn_message.to),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            content: ActiveValue::Set(new_cn_message.content),
        };

        let cn_process = cn_message::Entity::insert(model)
            .exec_with_returning(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorCreatingCNMessage(err.into()))?;
        Ok(cn_process)
    }

    async fn delete_cn_message(
        &self,
        cn_process_id: Urn,
        cn_message_id: Urn,
    ) -> anyhow::Result<(), CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_process = self
            .get_cn_process_by_cn_id(cn_process_id)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        match cn_message::Entity::delete_by_id(cn_message_id.as_str())
            .exec(db_connection)
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
impl ContractNegotiationOfferRepo for ContractNegotiationRepoForSql {
    async fn get_all_cn_offers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_offers = cn_offer::Entity::find()
            .limit(limit.unwrap_or(10000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?;
        Ok(cn_offers)
    }

    async fn get_all_cn_offers_by_message_id(
        &self,
        message_id: Urn,
    ) -> anyhow::Result<Option<cn_offer::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_message = self
            .get_cn_messages_by_cn_message_id(message_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?
            .ok_or(CnErrors::CNMessageNotFound)?;

        let cn_offer = cn_offer::Entity::find()
            .filter(cn_offer::Column::CnMessageId.eq(cn_message.cn_message_id))
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?;
        Ok(cn_offer)
    }

    async fn get_all_cn_offers_by_provider(
        &self,
        provider_id: Urn,
    ) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_offers = cn_offer::Entity::find()
            .join(JoinType::InnerJoin, cn_message::Relation::CnProcess.def())
            .join(JoinType::InnerJoin, cn_offer::Relation::CnMessage.def())
            .filter(cn_process::Column::ProviderId.eq(provider_id.as_str()))
            .all(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?;
        Ok(cn_offers)
    }

    async fn get_all_cn_offers_by_consumer(
        &self,
        consumer_id: Urn,
    ) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_offers = cn_offer::Entity::find()
            .join(JoinType::InnerJoin, cn_message::Relation::CnProcess.def())
            .join(JoinType::InnerJoin, cn_offer::Relation::CnMessage.def())
            .filter(cn_process::Column::ConsumerId.eq(consumer_id.as_str()))
            .all(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?;
        Ok(cn_offers)
    }

    async fn get_all_cn_offers_by_cn_process(
        &self,
        process_id: Urn,
    ) -> anyhow::Result<Vec<cn_offer::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_process = self
            .get_cn_process_by_cn_id(process_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_offers = cn_offer::Entity::find()
            .join(JoinType::InnerJoin, cn_offer::Relation::CnMessage.def())
            .filter(cn_message::Column::CnProcessId.eq(cn_process.cn_process_id.as_str()))
            .all(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?;
        Ok(cn_offers)
    }

    async fn get_last_cn_offers_by_cn_process(
        &self,
        process_id: Urn,
    ) -> anyhow::Result<Option<cn_offer::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_process = self
            .get_cn_process_by_cn_id(process_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_offers = cn_offer::Entity::find()
            .join(JoinType::InnerJoin, cn_offer::Relation::CnMessage.def())
            .filter(cn_message::Column::CnProcessId.eq(cn_process.cn_process_id.as_str()))
            .order_by_desc(cn_offer::Column::CreatedAt)
            .limit(1)
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?;
        Ok(cn_offers)
    }

    async fn get_cn_offer_by_id(
        &self,
        offer_id: Urn,
    ) -> anyhow::Result<Option<cn_offer::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_offers = cn_offer::Entity::find_by_id(offer_id.as_str())
            .one(db_connection)
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
        let db_connection = get_db_connection().await;

        let cn_process = self
            .get_cn_process_by_cn_id(process_id)
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
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNOffer(err.into()))?
            .ok_or(CnErrors::CNOfferNotFound)?;

        let mut old_active_model: cn_offer::ActiveModel = old_model.into();

        let model = old_active_model
            .update(db_connection)
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
        let db_connection = get_db_connection().await;
        let cn_process = self
            .get_cn_process_by_cn_id(process_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let cn_message = self
            .get_cn_messages_by_cn_message_id(message_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?
            .ok_or(CnErrors::CNMessageNotFound)?;

        let model = cn_offer::ActiveModel {
            // TODO review this...
            // offer_id: ActiveValue::Set(new_cn_offer.offer_id.to_string()),
            offer_id: ActiveValue::Set(get_urn(None).to_string()),
            cn_message_id: ActiveValue::Set(message_id.to_string()),
            offer_content: ActiveValue::Set(new_cn_offer.offer_content),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        let cn_offer = cn_offer::Entity::insert(model)
            .exec_with_returning(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorCreatingCNOffer(err.into()))?;
        Ok(cn_offer)
    }

    async fn delete_cn_offer(
        &self,
        process_id: Urn,
        message_id: Urn,
        offer_id: Urn,
    ) -> anyhow::Result<(), CnErrors> {
        let db_connection = get_db_connection().await;
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
            .exec(db_connection)
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
impl AgreementRepo for ContractNegotiationRepoForSql {
    async fn get_all_agreements(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<agreement::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let agreements = agreement::Entity::find()
            .limit(limit.unwrap_or(10000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingAgreement(err.into()))?;
        Ok(agreements)
    }

    async fn get_agreement_by_ag_id(
        &self,
        agreement_id: Urn,
    ) -> anyhow::Result<Option<agreement::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let agreement = agreement::Entity::find_by_id(agreement_id.as_str())
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingAgreement(err.into()))?;
        Ok(agreement)
    }

    async fn get_agreement_by_process_id(
        &self,
        process_id: Urn,
    ) -> anyhow::Result<Option<agreement::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_process = self
            .get_cn_process_by_cn_id(process_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNProcess(err.into()))?
            .ok_or(CnErrors::CNProcessNotFound)?;

        let agreement = agreement::Entity::find()
            .join(JoinType::InnerJoin, agreement::Relation::CnMessage.def())
            .filter(cn_message::Column::CnProcessId.eq(process_id.as_str()))
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingAgreement(err.into()))?;
        Ok(agreement)
    }

    async fn get_agreement_by_message_id(
        &self,
        message_id: Urn,
    ) -> anyhow::Result<Option<agreement::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let cn_message = self
            .get_cn_messages_by_cn_message_id(message_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingCNMessage(err.into()))?
            .ok_or(CnErrors::CNMessageNotFound)?;

        let agreement = agreement::Entity::find()
            .filter(agreement::Column::CnMessageId.eq(message_id.as_str()))
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingAgreement(err.into()))?;
        Ok(agreement)
    }

    async fn get_agreements_by_participant_id(
        &self,
        participant_id: Urn,
    ) -> anyhow::Result<Vec<agreement::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let participant = self
            .get_participant_by_p_id(participant_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingParticipant(err.into()))?
            .ok_or(CnErrors::ParticipantNotFound(
                "".to_string(),
                participant_id.clone(),
            ))?;

        let agreement = agreement::Entity::find()
            .filter(
                Condition::any()
                    .add(
                        agreement::Column::ProviderParticipantId
                            .eq(participant.participant_id.clone()),
                    )
                    .add(agreement::Column::ConsumerParticipantId.eq(participant.participant_id)),
            )
            .all(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingAgreement(err.into()))?;
        Ok(agreement)
    }

    async fn put_agreement(
        &self,
        process_id: Urn,
        message_id: Urn,
        agreement_id: Urn,
        edit_agreement: EditAgreement,
    ) -> anyhow::Result<agreement::Model, CnErrors> {
        let db_connection = get_db_connection().await;
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
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingAgreement(err.into()))?
            .ok_or(CnErrors::AgreementNotFound)?;

        let mut old_active_model: agreement::ActiveModel = old_model.into();
        if let Some(active) = edit_agreement.active {
            old_active_model.active = ActiveValue::Set(active);
        }

        let model = old_active_model
            .update(db_connection)
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
        let db_connection = get_db_connection().await;
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

        let consumer_participant = self
            .get_participant_by_p_id(new_agreement.consumer_participant_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingParticipant(err.into()))?
            .ok_or(CnErrors::ParticipantNotFound(
                "Consumer".to_string(),
                new_agreement.consumer_participant_id.clone(),
            ))?;

        let provider_participant = self
            .get_participant_by_p_id(new_agreement.provider_participant_id.clone())
            .await
            .map_err(|err| CnErrors::ErrorFetchingParticipant(err.into()))?
            .ok_or(CnErrors::ParticipantNotFound(
                "Provider".to_string(),
                new_agreement.provider_participant_id.clone(),
            ))?;

        let model = agreement::ActiveModel {
            agreement_id: ActiveValue::Set(get_urn(None).to_string()),
            consumer_participant_id: ActiveValue::Set(
                new_agreement.consumer_participant_id.to_string(),
            ),
            provider_participant_id: ActiveValue::Set(
                new_agreement.provider_participant_id.to_string(),
            ),
            cn_message_id: ActiveValue::Set(message_id.to_string()),
            agreement_content: ActiveValue::Set(new_agreement.agreement_content),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            active: ActiveValue::Set(true),
        };

        let agreement = agreement::Entity::insert(model)
            .exec_with_returning(db_connection)
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
        let db_connection = get_db_connection().await;
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
            .exec(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorDeletingAgreement(err.into()))?
            .rows_affected
        {
            0 => Err(CnErrors::AgreementNotFound),
            _ => Ok(()),
        }
    }
}

#[async_trait]
impl Participant for ContractNegotiationRepoForSql {
    async fn get_all_participants(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<participant::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let participants = participant::Entity::find()
            .limit(limit.unwrap_or(10000))
            .offset(page.unwrap_or(0))
            .all(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingParticipant(err.into()))?;
        Ok(participants)
    }

    async fn get_participant_by_p_id(
        &self,
        participant_id: Urn,
    ) -> anyhow::Result<Option<participant::Model>, CnErrors> {
        let db_connection = get_db_connection().await;
        let participant = participant::Entity::find_by_id(participant_id.as_str())
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingParticipant(err.into()))?;
        Ok(participant)
    }

    async fn put_participant(
        &self,
        participant_id: Urn,
        edit_participant: EditParticipant,
    ) -> anyhow::Result<participant::Model, CnErrors> {
        let db_connection = get_db_connection().await;
        let old_model = participant::Entity::find_by_id(participant_id.as_str())
            .one(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorFetchingParticipant(err.into()))?
            .ok_or(CnErrors::ParticipantNotFound(
                "".to_string(),
                participant_id,
            ))?;

        let mut old_active_model: participant::ActiveModel = old_model.into();

        if let Some(base_url) = edit_participant.base_url {
            old_active_model.base_url = ActiveValue::Set(base_url);
        }
        if let Some(extra_fields) = edit_participant.extra_fields {
            let mut old_json_content = old_active_model.extra_fields.unwrap();
            old_json_content.merge(&extra_fields);
            old_active_model.extra_fields = ActiveValue::Set(old_json_content);
        }

        let model = old_active_model
            .update(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorUpdatingParticipant(err.into()))?;
        Ok(model)
    }

    async fn create_participant(
        &self,
        new_participant: NewParticipant,
    ) -> anyhow::Result<participant::Model, CnErrors> {
        let db_connection = get_db_connection().await;
        let participant_id = new_participant.participant_id.unwrap_or(get_urn(None)).to_string();
        let model = participant::ActiveModel {
            participant_id: ActiveValue::Set(participant_id),
            identity_token: ActiveValue::Set(Option::from("TODO TOKENS".to_string())),
            _type: ActiveValue::Set(new_participant._type),
            base_url: ActiveValue::Set(new_participant.base_url),
            extra_fields: ActiveValue::Set(new_participant.extra_fields),
        };

        let participant = participant::Entity::insert(model)
            .exec_with_returning(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorCreatingParticipant(err.into()))?;
        Ok(participant)
    }

    async fn delete_participant(&self, participant_id: Urn) -> anyhow::Result<(), CnErrors> {
        let db_connection = get_db_connection().await;
        match participant::Entity::delete_by_id(participant_id.as_str())
            .exec(db_connection)
            .await
            .map_err(|err| CnErrors::ErrorDeletingParticipant(err.into()))?
            .rows_affected
        {
            0 => Err(CnErrors::ParticipantNotFound(
                "".to_string(),
                participant_id,
            )),
            _ => Ok(()),
        }
    }
}
