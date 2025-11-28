use crate::data::entities::data_plane_field::{EditDataPlaneFieldModel, NewDataPlaneFieldModel};
use crate::data::entities::data_plane_process;
use crate::data::entities::data_plane_process::{EditDataPlaneProcessModel, NewDataPlaneProcessModel};
use crate::data::factory_trait::DataPlaneRepoTrait;
use crate::data::repo_traits::data_plane_process_repo::DataPlaneProcessRepoErrors;
use crate::entities::data_plane_process::{
    DataPlaneProcessDto, DataPlaneProcessEntitiesTrait, EditDataPlaneProcessDto, NewDataPlaneProcessDto,
};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;
use urn::Urn;

pub struct DataPlaneProcessEntityService {
    pub data_plane_repo: Arc<dyn DataPlaneRepoTrait>,
}

impl DataPlaneProcessEntityService {
    pub fn new(data_plane_repo: Arc<dyn DataPlaneRepoTrait>) -> Self {
        Self { data_plane_repo }
    }

    async fn enrich_process(&self, process: data_plane_process::Model) -> anyhow::Result<DataPlaneProcessDto> {
        let process_urn = Urn::from_str(&process.id).map_err(|e| {
            let err = CommonErrors::parse_new(&format!(
                "Critical: Invalid URN found in database for process {}. Error: {}",
                process.id, e
            ));
            error!("{}", err.log());
            err
        })?;
        let fields = self
            .data_plane_repo
            .get_data_plane_fields_repo()
            .get_all_data_plane_fields_by_process_id(&process_urn)
            .await
            .map_err(|error| {
                let err = CommonErrors::database_new(&error.to_string());
                error!("{}", err);
                err
            })?;
        let ids_map: HashMap<String, String> = fields.into_iter().map(|field| (field.key, field.value)).collect();
        Ok(DataPlaneProcessDto { inner: process, data_plane_fields: ids_map })
    }
}

#[async_trait::async_trait]
impl DataPlaneProcessEntitiesTrait for DataPlaneProcessEntityService {
    async fn get_all_data_plane_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<DataPlaneProcessDto>> {
        let dp_processes = self
            .data_plane_repo
            .get_data_plane_process_repo()
            .get_all_data_plane_processes(limit, page)
            .await
            .map_err(|error| {
                let err = CommonErrors::database_new(&error.to_string());
                error!("{}", err);
                err
            })?;
        let mut dtos = Vec::with_capacity(dp_processes.len());
        for p in dp_processes {
            let dto = self.enrich_process(p).await?;
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_batch_data_plane_processes(&self, ids: Vec<Urn>) -> anyhow::Result<Vec<DataPlaneProcessDto>> {
        let dp_processes =
            self.data_plane_repo.get_data_plane_process_repo().get_batch_data_plane_processes(&ids).await.map_err(
                |error| {
                    let err = CommonErrors::database_new(&error.to_string());
                    error!("{}", err.log());
                    err
                },
            )?;
        let mut dtos = Vec::with_capacity(dp_processes.len());
        for p in dp_processes {
            let dto = self.enrich_process(p).await?;
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_data_plane_process_by_id(&self, id: &Urn) -> anyhow::Result<Option<DataPlaneProcessDto>> {
        let dp_process =
            self.data_plane_repo.get_data_plane_process_repo().get_data_plane_processes_by_id(id).await.map_err(
                |error| {
                    let err = CommonErrors::database_new(&error.to_string());
                    error!("{}", err.log());
                    err
                },
            )?;
        let dto = match dp_process {
            Some(process) => Some(self.enrich_process(process).await), // Devuelve Option<Result<...>>
            None => None,
        }
        .transpose()?;
        Ok(dto)
    }

    async fn create_data_plane_process(
        &self,
        new_data_plane_process: &NewDataPlaneProcessDto,
    ) -> anyhow::Result<DataPlaneProcessDto> {
        let new_dp_process_model: NewDataPlaneProcessModel = new_data_plane_process.clone().into();
        let created_process = self
            .data_plane_repo
            .get_data_plane_process_repo()
            .create_data_plane_processes(&new_dp_process_model)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let process_urn = Urn::from_str(&created_process.id).map_err(|e| {
            let err = CommonErrors::parse_new(&format!("Generated ID is not a valid URN: {}", e));
            error!("{}", err.log());
            err
        })?;

        if let Some(fields) = &new_data_plane_process.fields {
            for field in fields {
                let new_field_model = NewDataPlaneFieldModel { key: field.0.clone(), value: field.1.clone() };
                self.data_plane_repo
                    .get_data_plane_fields_repo()
                    .create_data_plane_field(&process_urn, &new_field_model)
                    .await
                    .map_err(|error| {
                        let err = CommonErrors::parse_new(&format!("Generated ID is not a valid URN: {}", error));
                        error!("{}", err.log());
                        err
                    })?;
            }
        }

        self.enrich_process(created_process).await
    }

    async fn put_data_plane_process(
        &self,
        id: &Urn,
        edit_data_plane_process: &EditDataPlaneProcessDto,
    ) -> anyhow::Result<DataPlaneProcessDto> {
        let edit_dp_process_model = EditDataPlaneProcessModel { state: edit_data_plane_process.state.clone() };
        let edited_process = self
            .data_plane_repo
            .get_data_plane_process_repo()
            .put_data_plane_processes(id, &edit_dp_process_model)
            .await
            .map_err(|error| match error {
                DataPlaneProcessRepoErrors::DataplaneProcessNotFound => {
                    let err =
                        CommonErrors::missing_resource_new(&id.to_string(), "Dataplane process not found for update");
                    error!("{}", err.log());
                    err
                }
                _ => {
                    let err = CommonErrors::database_new(&error.to_string());
                    error!("{}", err.log());
                    err
                }
            })?;

        let process_urn = Urn::from_str(&edited_process.id).map_err(|e| {
            let err = CommonErrors::parse_new(&format!("Updated ID is not a valid URN: {}", e));
            error!("{}", err.log());
            err
        })?;

        if let Some(incoming_fields) = &edit_data_plane_process.fields {
            let existing_fields = self
                .data_plane_repo
                .get_data_plane_fields_repo()
                .get_all_data_plane_fields_by_process_id(&process_urn)
                .await
                .map_err(|e| {
                    let err = CommonErrors::database_new(&format!("Failed to fetch existing fields: {}", e));
                    error!("{}", err.log());
                    err
                })?;

            let existing_fields_map: HashMap<String, String> =
                existing_fields.into_iter().map(|field| (field.key, field.id)).collect();

            for (key, new_value) in incoming_fields {
                if let Some(existing_field_id_str) = existing_fields_map.get(key) {
                    let field_urn = Urn::from_str(existing_field_id_str)
                        .map_err(|e| CommonErrors::parse_new(&format!("Invalid existing field URN in DB: {}", e)))?;
                    let edit_field_model = EditDataPlaneFieldModel { value: new_value.clone() };

                    self.data_plane_repo
                        .get_data_plane_fields_repo()
                        .put_data_plane_field(&field_urn, &edit_field_model)
                        .await
                        .map_err(|e| {
                            let err = CommonErrors::database_new(&format!("Failed to update field {}: {}", key, e));
                            error!("{}", err.log());
                            err
                        })?;
                } else {
                    let new_field_model = NewDataPlaneFieldModel { key: key.clone(), value: new_value.clone() };

                    self.data_plane_repo
                        .get_data_plane_fields_repo()
                        .create_data_plane_field(&process_urn, &new_field_model)
                        .await
                        .map_err(|e| {
                            let err = CommonErrors::database_new(&format!("Failed to create field {}: {}", key, e));
                            error!("{}", err.log());
                            err
                        })?;
                }
            }
        }

        self.enrich_process(edited_process).await
    }

    async fn delete_data_plane_process(&self, id: &Urn) -> anyhow::Result<()> {
        self.data_plane_repo.get_data_plane_process_repo().delete_data_plane_processes(id).await.map_err(|error| {
            let err = CommonErrors::database_new(&error.to_string());
            tracing::log::error!("{}", err.log());
            err
        })?;
        Ok(())
    }
}
