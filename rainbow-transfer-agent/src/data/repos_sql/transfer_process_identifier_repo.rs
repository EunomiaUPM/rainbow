use crate::data::entities::transfer_process_identifier;
use crate::data::entities::transfer_process_identifier::{EditTransferIdentifierModel, NewTransferIdentifierModel};
use crate::data::repo_traits::transfer_process_identifier_repo::{
    TransferIdentifierRepoErrors, TransferIdentifierRepoTrait,
};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use urn::Urn;

pub struct TransferIdentifierRepoForSql {
    db_connection: DatabaseConnection,
}

impl TransferIdentifierRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl TransferIdentifierRepoTrait for TransferIdentifierRepoForSql {
    async fn get_all_identifiers(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<transfer_process_identifier::Model>, TransferIdentifierRepoErrors> {
        let identifiers = transfer_process_identifier::Entity::find()
            .limit(limit.unwrap_or(20))
            .offset(page.map(|p| p * limit.unwrap_or(20)).unwrap_or(0))
            .all(&self.db_connection)
            .await;

        match identifiers {
            Ok(identifiers) => Ok(identifiers),
            Err(e) => Err(TransferIdentifierRepoErrors::ErrorFetchingTransferIdentifier(e.into())),
        }
    }

    async fn get_identifiers_by_process_id(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<Vec<transfer_process_identifier::Model>, TransferIdentifierRepoErrors> {
        let pid = process_id.to_string();
        let identifiers = transfer_process_identifier::Entity::find()
            .filter(transfer_process_identifier::Column::TransferAgentProcessId.eq(pid))
            .all(&self.db_connection)
            .await;

        match identifiers {
            Ok(identifiers) => Ok(identifiers),
            Err(e) => Err(TransferIdentifierRepoErrors::ErrorFetchingTransferIdentifier(e.into())),
        }
    }

    async fn get_identifier_by_id(
        &self,
        id: &Urn,
    ) -> anyhow::Result<Option<transfer_process_identifier::Model>, TransferIdentifierRepoErrors> {
        let iid = id.to_string();
        let identifier = transfer_process_identifier::Entity::find_by_id(iid).one(&self.db_connection).await;

        match identifier {
            Ok(identifier) => Ok(identifier),
            Err(e) => Err(TransferIdentifierRepoErrors::ErrorFetchingTransferIdentifier(e.into())),
        }
    }

    async fn get_identifier_by_key(
        &self,
        process_id: &Urn,
        key: &str,
    ) -> anyhow::Result<Option<transfer_process_identifier::Model>, TransferIdentifierRepoErrors> {
        let pid = process_id.to_string();
        let identifier = transfer_process_identifier::Entity::find()
            .filter(transfer_process_identifier::Column::TransferAgentProcessId.eq(pid))
            .filter(transfer_process_identifier::Column::IdKey.eq(key))
            .one(&self.db_connection)
            .await;

        match identifier {
            Ok(identifier) => Ok(identifier),
            Err(e) => Err(TransferIdentifierRepoErrors::ErrorFetchingTransferIdentifier(e.into())),
        }
    }

    async fn create_identifier(
        &self,
        new_model: &NewTransferIdentifierModel,
    ) -> anyhow::Result<transfer_process_identifier::Model, TransferIdentifierRepoErrors> {
        let model: transfer_process_identifier::ActiveModel = new_model.clone().into();
        let result = transfer_process_identifier::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match result {
            Ok(identifier) => Ok(identifier),
            Err(e) => Err(TransferIdentifierRepoErrors::ErrorCreatingTransferIdentifier(e.into())),
        }
    }

    async fn put_identifier(
        &self,
        id: &Urn,
        edit_model: &EditTransferIdentifierModel,
    ) -> anyhow::Result<transfer_process_identifier::Model, TransferIdentifierRepoErrors> {
        let iid = id.to_string();
        let old_model = transfer_process_identifier::Entity::find_by_id(&iid).one(&self.db_connection).await;
        let old_model = match old_model {
            Ok(Some(model)) => model,
            Ok(None) => return Err(TransferIdentifierRepoErrors::TransferIdentifierNotFound),
            Err(e) => return Err(TransferIdentifierRepoErrors::ErrorFetchingTransferIdentifier(e.into())),
        };

        let mut active_model: transfer_process_identifier::ActiveModel = old_model.into();
        if let Some(key) = &edit_model.id_key {
            active_model.id_key = ActiveValue::Set(key.clone());
        }
        if let Some(value) = &edit_model.id_value {
            active_model.id_value = ActiveValue::Set(Some(value.clone()));
        }

        let result = active_model.update(&self.db_connection).await;
        match result {
            Ok(updated_model) => Ok(updated_model),
            Err(e) => Err(TransferIdentifierRepoErrors::ErrorUpdatingTransferIdentifier(e.into())),
        }
    }

    async fn delete_identifier(&self, id: &Urn) -> anyhow::Result<(), TransferIdentifierRepoErrors> {
        let iid = id.to_string();
        let result = transfer_process_identifier::Entity::delete_by_id(iid).exec(&self.db_connection).await;

        match result {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(TransferIdentifierRepoErrors::TransferIdentifierNotFound),
                _ => Ok(()),
            },
            Err(e) => Err(TransferIdentifierRepoErrors::ErrorDeletingTransferIdentifier(e.into())),
        }
    }
}
