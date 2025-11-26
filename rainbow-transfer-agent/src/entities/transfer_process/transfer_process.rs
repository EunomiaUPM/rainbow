use crate::data::entities::transfer_process::{
    self as transfer_process_model, EditTransferProcessModel, NewTransferProcessModel,
};
use crate::data::entities::transfer_process_identifier::{EditTransferIdentifierModel, NewTransferIdentifierModel};
use crate::data::factory_trait::TransferAgentRepoTrait;
use crate::data::repo_traits::transfer_process_repo::TransferProcessRepoErrors;
use crate::entities::transfer_process::{
    EditTransferProcessDto, NewTransferProcessDto, TransferAgentProcessesTrait, TransferProcessDto,
};
use log::error;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::utils::get_urn;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use urn::Urn;

pub struct TransferAgentProcessesService {
    pub transfer_repo: Arc<dyn TransferAgentRepoTrait>,
}

impl TransferAgentProcessesService {
    pub fn new(transfer_repo: Arc<dyn TransferAgentRepoTrait>) -> Self {
        Self { transfer_repo }
    }

    async fn enrich_process(&self, process: transfer_process_model::Model) -> anyhow::Result<TransferProcessDto> {
        let process_urn = Urn::from_str(&process.id).map_err(|e| {
            let err = CommonErrors::parse_new(&format!(
                "Critical: Invalid URN found in database for process {}. Error: {}",
                process.id, e
            ));
            error!("{}", err.log());
            err
        })?;

        let messages =
            self.transfer_repo.get_transfer_message_repo().get_messages_by_process_id(&process_urn).await.map_err(
                |e| {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("{}", err.log());
                    err
                },
            )?;

        let identifiers_models = self
            .transfer_repo
            .get_transfer_process_identifiers_repo()
            .get_identifiers_by_process_id(&process_urn)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        let ids_map: HashMap<String, String> = identifiers_models
            .into_iter()
            .filter_map(|id_model| match id_model.id_value {
                Some(val) => Some((id_model.id_key, val)),
                None => None,
            })
            .collect();

        Ok(TransferProcessDto { inner: process, identifiers: ids_map, messages })
    }
}

#[async_trait::async_trait]
impl TransferAgentProcessesTrait for TransferAgentProcessesService {
    async fn get_all_transfer_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<TransferProcessDto>> {
        let processes =
            self.transfer_repo.get_transfer_process_repo().get_all_transfer_processes(limit, page).await.map_err(
                |e| {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("{}", err.log());
                    err
                },
            )?;

        let mut dtos = Vec::with_capacity(processes.len());
        for p in processes {
            let dto = self.enrich_process(p).await?;
            dtos.push(dto);
        }

        Ok(dtos)
    }

    async fn get_batch_transfer_processes(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<TransferProcessDto>> {
        let processes =
            self.transfer_repo.get_transfer_process_repo().get_batch_transfer_processes(ids).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        let mut dtos = Vec::with_capacity(processes.len());
        for p in processes {
            let dto = self.enrich_process(p).await?;
            dtos.push(dto);
        }

        Ok(dtos)
    }

    async fn get_transfer_process_by_id(&self, id: &Urn) -> anyhow::Result<TransferProcessDto> {
        let process = self
            .transfer_repo
            .get_transfer_process_repo()
            .get_transfer_process_by_id(id)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?
            .ok_or_else(|| {
                let err = CommonErrors::missing_resource_new(&id.to_string(), "Transfer Process not found");
                error!("{}", err.log());
                err
            })?;

        self.enrich_process(process).await
    }

    async fn get_transfer_process_by_key_id(&self, key_id: &str, id: &Urn) -> anyhow::Result<TransferProcessDto> {
        let process = self
            .transfer_repo
            .get_transfer_process_repo()
            .get_transfer_process_by_key_id(key_id, id)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?
            .ok_or_else(|| {
                let err = CommonErrors::missing_resource_new(
                    &format!("Key: {} / ID: {}", key_id, id),
                    "Transfer Process not found by key identifier",
                );
                error!("{}", err.log());
                err
            })?;

        self.enrich_process(process).await
    }

    async fn get_transfer_process_by_key_value(&self, id: &Urn) -> anyhow::Result<TransferProcessDto> {
        let process = self
            .transfer_repo
            .get_transfer_process_repo()
            .get_transfer_process_by_key_value(id)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?
            .ok_or_else(|| {
                let err = CommonErrors::missing_resource_new(
                    &format!("ID: {}", id),
                    "Transfer Process not found by key identifier",
                );
                error!("{}", err.log());
                err
            })?;

        self.enrich_process(process).await
    }

    async fn create_transfer_process(
        &self,
        new_model_dto: &NewTransferProcessDto,
    ) -> anyhow::Result<TransferProcessDto> {
        let new_process_model: NewTransferProcessModel = new_model_dto.clone().into();

        let created_process =
            self.transfer_repo.get_transfer_process_repo().create_transfer_process(&new_process_model).await.map_err(
                |e| {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("{}", err.log());
                    err
                },
            )?;

        let process_urn = Urn::from_str(&created_process.id).map_err(|e| {
            let err = CommonErrors::parse_new(&format!("Generated ID is not a valid URN: {}", e));
            error!("{}", err.log());
            err
        })?;

        if let Some(identifiers) = &new_model_dto.identifiers {
            for (key, urn_value) in identifiers {
                let new_ident_model = NewTransferIdentifierModel {
                    id: None,
                    transfer_agent_process_id: process_urn.clone(),
                    id_key: key.clone(),
                    id_value: Some(urn_value.to_string()),
                };

                self.transfer_repo
                    .get_transfer_process_identifiers_repo()
                    .create_identifier(&new_ident_model)
                    .await
                    .map_err(|e| {
                        let err = CommonErrors::database_new(&format!("Error creating identifier {}: {}", key, e));
                        error!("{}", err.log());
                        err
                    })?;
            }
        }

        self.enrich_process(created_process).await
    }

    async fn put_transfer_process(
        &self,
        id: &Urn,
        edit_model_dto: &EditTransferProcessDto,
    ) -> anyhow::Result<TransferProcessDto> {
        let edit_model: EditTransferProcessModel = edit_model_dto.clone().into();

        let updated_process =
            self.transfer_repo.get_transfer_process_repo().put_transfer_process(id, &edit_model).await.map_err(
                |e| {
                    match e {
                        TransferProcessRepoErrors::TransferProcessNotFound => {
                            let err = CommonErrors::missing_resource_new(
                                &id.to_string(),
                                "Transfer process not found for update",
                            );
                            error!("{}", err.log());
                            err // Convertimos a anyhow::Error compatible
                        }
                        _ => {
                            let err = CommonErrors::database_new(&e.to_string());
                            error!("{}", err.log());
                            err
                        }
                    }
                },
            )?;

        let process_urn = Urn::from_str(&updated_process.id).map_err(|e| {
            let err = CommonErrors::parse_new(&format!("Updated ID is not a valid URN: {}", e));
            error!("{}", err.log());
            err
        })?;

        if let Some(identifiers) = &edit_model_dto.identifiers {
            for (key, urn_value) in identifiers {
                let new_ident_model = EditTransferIdentifierModel {
                    id_key: Option::from(key.clone()),
                    id_value: Some(urn_value.to_string()),
                };

                let identifier_model = self
                    .transfer_repo
                    .get_transfer_process_identifiers_repo()
                    .get_identifier_by_key(&process_urn, key)
                    .await
                    .map_err(|e| {
                        let err = CommonErrors::database_new(&e.to_string());
                        error!("{}", err.log());
                        err
                    })?;

                if identifier_model.is_none() {
                    self.transfer_repo
                        .get_transfer_process_identifiers_repo()
                        .create_identifier(&NewTransferIdentifierModel {
                            id: Some(get_urn(None)),
                            transfer_agent_process_id: process_urn.clone(),
                            id_key: key.clone(),
                            id_value: Some(urn_value.to_string()),
                        })
                        .await
                        .map_err(|e| {
                            let err =
                                CommonErrors::database_new(&format!("Error creating new identifier {}: {}", key, e));
                            error!("{}", err.log());
                            err
                        })?;
                } else {
                    let id_urn_ident = Urn::from_str(identifier_model.unwrap().id.as_str()).map_err(|e| {
                        let err = CommonErrors::parse_new(&format!("Identifier URN malformed: {}", e));
                        error!("{}", err.log());
                        err
                    })?;

                    self.transfer_repo
                        .get_transfer_process_identifiers_repo()
                        .put_identifier(&id_urn_ident, &new_ident_model)
                        .await
                        .map_err(|e| {
                            let err = CommonErrors::database_new(&format!("Error updating identifier {}: {}", key, e));
                            error!("{}", err.log());
                            err
                        })?;
                }
            }
        }

        self.enrich_process(updated_process).await
    }

    async fn delete_transfer_process(&self, id: &Urn) -> anyhow::Result<()> {
        self.transfer_repo.get_transfer_process_repo().delete_transfer_process(id).await.map_err(|e| match e {
            TransferProcessRepoErrors::TransferProcessNotFound => {
                let err =
                    CommonErrors::missing_resource_new(&id.to_string(), "Transfer process not found for deletion");
                error!("{}", err.log());
                err
            }
            _ => {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            }
        })?;
        Ok(())
    }
}
