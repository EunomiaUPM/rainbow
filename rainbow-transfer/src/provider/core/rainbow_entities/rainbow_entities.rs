use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use urn::Urn;
use log::error; // Importante para el logging
use rainbow_common::utils::get_urn;
use rainbow_common::errors::{CommonErrors, ErrorLog}; // Importamos el sistema de errores

use crate::db::entities::transfer_process::{
    self as transfer_process_model, EditTransferProcessModel, NewTransferProcessModel,
};
use crate::db::entities::transfer_process_identifier::{EditTransferIdentifierModel, NewTransferIdentifierModel};
use crate::db::factory_trait::TransferAgentRepoTrait;
use crate::entities::transfer_process::{
    EditTransferProcessDto, NewTransferProcessDto, TransferAgentProcessesTrait, TransferProcessDto,
};

pub struct TransferAgentProcessesService {
    pub transfer_repo: Arc<dyn TransferAgentRepoTrait>,
}

impl TransferAgentProcessesService {
    pub fn new(transfer_repo: Arc<dyn TransferAgentRepoTrait>) -> Self {
        Self { transfer_repo }
    }

    // Helper privado con manejo de errores integrado
    async fn enrich_process(&self, process: transfer_process_model::Model) -> anyhow::Result<TransferProcessDto> {
        // 1. Validación de URN (Error de integridad de datos)
        let process_urn = Urn::from_str(&process.id).map_err(|e| {
            let err = CommonErrors::internal_server_error_new(&format!(
                "Critical: Invalid URN found in database for process {}. Error: {}",
                process.id, e
            ));
            error!("{}", err.log());
            err
        })?;

        // 2. Obtener mensajes (Error de base de datos)
        let messages = self
            .transfer_repo
            .get_transfer_message_repo()
            .get_messages_by_process_id(&process_urn)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        // 3. Obtener identificadores (Error de base de datos)
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

        // 4. Mapeo de identificadores
        let ids_map: HashMap<String, Urn> = identifiers_models
            .into_iter()
            .filter_map(|id_model| match id_model.id_value {
                Some(val) => Urn::from_str(&val).ok().map(|u| (id_model.id_key, u)),
                None => None,
            })
            .collect();

        Ok(TransferProcessDto { inner: process, ids: ids_map, messages })
    }
}

#[async_trait::async_trait]
impl TransferAgentProcessesTrait for TransferAgentProcessesService {
    async fn get_all_transfer_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<TransferProcessDto>> {
        let processes = self
            .transfer_repo
            .get_transfer_process_repo()
            .get_all_transfer_processes(limit, page)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        let mut dtos = Vec::with_capacity(processes.len());
        for p in processes {
            // enrich_process ya maneja y loguea sus propios errores, solo propagamos con ?
            let dto = self.enrich_process(p).await?;
            dtos.push(dto);
        }

        Ok(dtos)
    }

    async fn get_batch_transfer_processes(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<TransferProcessDto>> {
        let processes = self
            .transfer_repo
            .get_transfer_process_repo()
            .get_batch_transfer_processes(ids)
            .await
            .map_err(|e| {
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

    async fn get_transfer_process_by_id(&self, id: &Urn) -> anyhow::Result<Option<TransferProcessDto>> {
        let process_opt = self
            .transfer_repo
            .get_transfer_process_repo()
            .get_transfer_process_by_id(id)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        match process_opt {
            Some(process) => {
                let dto = self.enrich_process(process).await?;
                Ok(Some(dto))
            }
            None => Ok(None), // Si devolvemos Option, None es válido y no es un error
        }
    }

    async fn get_transfer_process_by_key_id(
        &self,
        key_id: &str,
        id: &Urn,
    ) -> anyhow::Result<Option<TransferProcessDto>> {
        let process_opt = self
            .transfer_repo
            .get_transfer_process_repo()
            .get_transfer_process_by_key_id(key_id, id)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        match process_opt {
            Some(process) => {
                let dto = self.enrich_process(process).await?;
                Ok(Some(dto))
            }
            None => Ok(None),
        }
    }

    async fn create_transfer_process(
        &self,
        new_model_dto: &NewTransferProcessDto,
    ) -> anyhow::Result<TransferProcessDto> {
        let new_process_model: NewTransferProcessModel = new_model_dto.clone().into();

        // Crear proceso padre
        let created_process = self
            .transfer_repo
            .get_transfer_process_repo()
            .create_transfer_process(&new_process_model)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        let process_urn = Urn::from_str(&created_process.id).map_err(|e| {
            let err = CommonErrors::internal_server_error_new(&format!("Generated ID is not a valid URN: {}", e));
            error!("{}", err.log());
            err
        })?;

        // Crear identificadores asociados
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

        // Actualizar proceso padre
        let updated_process = self
            .transfer_repo
            .get_transfer_process_repo()
            .put_transfer_process(id, &edit_model)
            .await
            .map_err(|e| {
                // Si el repo devuelve un error específico de "Not Found", podríamos mapearlo aquí,
                // pero generalmente los repos SQL devuelven ErrorUpdating o similar.
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        let process_urn = Urn::from_str(&updated_process.id).map_err(|e| {
            let err = CommonErrors::internal_server_error_new(&format!("Updated ID is not a valid URN: {}", e));
            error!("{}", err.log());
            err
        })?;

        // Gestión de Identificadores
        if let Some(identifiers) = &edit_model_dto.identifiers {
            for (key, urn_value) in identifiers {
                let new_ident_model = EditTransferIdentifierModel {
                    id_key: Option::from(key.clone()),
                    id_value: Some(urn_value.to_string()),
                };

                // Check existencia
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
                    // CREATE identifier
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
                            let err = CommonErrors::database_new(&format!("Error creating new identifier {}: {}", key, e));
                            error!("{}", err.log());
                            err
                        })?;
                } else {
                    // UPDATE identifier
                    let id = Urn::from_str(identifier_model.unwrap().id.as_str()).map_err(|e| {
                        let err = CommonErrors::internal_server_error_new(&format!("Identifier URN in DB malformed: {}", e));
                        error!("{}", err.log());
                        err
                    })?;

                    self.transfer_repo
                        .get_transfer_process_identifiers_repo()
                        .put_identifier(&id, &new_ident_model)
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
        self.transfer_repo
            .get_transfer_process_repo()
            .delete_transfer_process(id)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        Ok(())
    }
}