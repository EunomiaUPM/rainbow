/*
 *
 * * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 * *
 * * This program is free software: you can redistribute it and/or modify
 * * it under the terms of the GNU General Public License as published by
 * * the Free Software Foundation, either version 3 of the License, or
 * * (at your option) any later version.
 * *
 * * This program is distributed in the hope that it will be useful,
 * * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * * GNU General Public License for more details.
 * *
 * * You should have received a copy of the GNU General Public License
 * * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::data::entities::negotiation_process::{
    self as negotiation_process_model, EditNegotiationProcessModel, NewNegotiationProcessModel,
};
use crate::data::entities::negotiation_process_identifier::{
    EditNegotiationIdentifierModel, NewNegotiationIdentifierModel,
};
use crate::data::factory_trait::NegotiationAgentRepoTrait;
use crate::data::repo_traits::negotiation_process_repo::NegotiationProcessRepoErrors;
use crate::entities::negotiation_process::{
    EditNegotiationProcessDto, NegotiationAgentProcessesTrait, NegotiationProcessDto, NewNegotiationProcessDto,
};
use log::error;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::utils::get_urn;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use urn::Urn;

pub struct NegotiationAgentProcessesService {
    pub negotiation_repo: Arc<dyn NegotiationAgentRepoTrait>,
}

impl NegotiationAgentProcessesService {
    pub fn new(negotiation_repo: Arc<dyn NegotiationAgentRepoTrait>) -> Self {
        Self { negotiation_repo }
    }

    async fn enrich_process(&self, process: negotiation_process_model::Model) -> anyhow::Result<NegotiationProcessDto> {
        let process_urn = Urn::from_str(&process.id).map_err(|e| {
            let err = CommonErrors::parse_new(&format!(
                "Critical: Invalid URN found in database for process {}. Error: {}",
                process.id, e
            ));
            error!("{}", err.log());
            err
        })?;

        let messages = self
            .negotiation_repo
            .get_negotiation_message_repo()
            .get_messages_by_process_id(&process_urn)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        let offers =
            self.negotiation_repo.get_offer_repo().get_offers_by_negotiation_process(&process_urn).await.map_err(
                |e| {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("{}", err.log());
                    err
                },
            )?;

        let agreement_opt = self
            .negotiation_repo
            .get_agreement_repo()
            .get_agreement_by_negotiation_process(&process_urn)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        let identifiers_models = self
            .negotiation_repo
            .get_negotiation_process_identifiers_repo()
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

        Ok(NegotiationProcessDto { inner: process, identifiers: ids_map, messages, offers, agreement: agreement_opt })
    }
}

#[async_trait::async_trait]
impl NegotiationAgentProcessesTrait for NegotiationAgentProcessesService {
    async fn get_all_negotiation_processes(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<NegotiationProcessDto>> {
        let processes = self
            .negotiation_repo
            .get_negotiation_process_repo()
            .get_all_negotiation_processes(limit, page)
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

    async fn get_batch_negotiation_processes(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<NegotiationProcessDto>> {
        let processes =
            self.negotiation_repo.get_negotiation_process_repo().get_batch_negotiation_processes(ids).await.map_err(
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

    async fn get_negotiation_process_by_id(&self, id: &Urn) -> anyhow::Result<Option<NegotiationProcessDto>> {
        let process_opt =
            self.negotiation_repo.get_negotiation_process_repo().get_negotiation_process_by_id(id).await.map_err(
                |e| {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("{}", err.log());
                    err
                },
            )?;

        match process_opt {
            Some(process) => Ok(Some(self.enrich_process(process).await?)),
            None => Ok(None),
        }
    }

    async fn get_negotiation_process_by_key_id(
        &self,
        key_id: &str,
        id: &Urn,
    ) -> anyhow::Result<Option<NegotiationProcessDto>> {
        let process_opt = self
            .negotiation_repo
            .get_negotiation_process_repo()
            .get_negotiation_process_by_key_id(key_id, id)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        match process_opt {
            Some(process) => Ok(Some(self.enrich_process(process).await?)),
            None => Ok(None),
        }
    }

    async fn get_negotiation_process_by_key_value(&self, id: &Urn) -> anyhow::Result<Option<NegotiationProcessDto>> {
        let process_opt = self
            .negotiation_repo
            .get_negotiation_process_repo()
            .get_negotiation_process_by_key_value(id)
            .await
            .map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;

        match process_opt {
            Some(process) => Ok(Some(self.enrich_process(process).await?)),
            None => Ok(None),
        }
    }

    async fn create_negotiation_process(
        &self,
        new_model_dto: &NewNegotiationProcessDto,
    ) -> anyhow::Result<NegotiationProcessDto> {
        let new_process_model: NewNegotiationProcessModel = new_model_dto.clone().into();

        let created_process = self
            .negotiation_repo
            .get_negotiation_process_repo()
            .create_negotiation_process(&new_process_model)
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

        if let Some(identifiers) = &new_model_dto.identifiers {
            for (key, urn_value) in identifiers {
                let new_ident_model = NewNegotiationIdentifierModel {
                    id: None,
                    negotiation_agent_process_id: process_urn.clone(),
                    id_key: key.clone(),
                    id_value: Some(urn_value.to_string()),
                };

                self.negotiation_repo
                    .get_negotiation_process_identifiers_repo()
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

    async fn put_negotiation_process(
        &self,
        id: &Urn,
        edit_model_dto: &EditNegotiationProcessDto,
    ) -> anyhow::Result<NegotiationProcessDto> {
        let edit_model: EditNegotiationProcessModel = edit_model_dto.clone().into();

        let updated_process = self
            .negotiation_repo
            .get_negotiation_process_repo()
            .put_negotiation_process(id, &edit_model)
            .await
            .map_err(|e| match e {
                NegotiationProcessRepoErrors::NegotiationProcessNotFound => {
                    let err =
                        CommonErrors::missing_resource_new(&id.to_string(), "Negotiation process not found for update");
                    error!("{}", err.log());
                    err
                }
                _ => {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("{}", err.log());
                    err
                }
            })?;

        let process_urn = Urn::from_str(&updated_process.id).map_err(|e| {
            let err = CommonErrors::parse_new(&format!("Updated ID is not a valid URN: {}", e));
            error!("{}", err.log());
            err
        })?;

        if let Some(identifiers) = &edit_model_dto.identifiers {
            for (key, urn_value) in identifiers {
                let new_ident_model = EditNegotiationIdentifierModel {
                    id_key: Option::from(key.clone()),
                    id_value: Some(urn_value.to_string()),
                };

                let identifier_model = self
                    .negotiation_repo
                    .get_negotiation_process_identifiers_repo()
                    .get_identifier_by_key(&process_urn, key)
                    .await
                    .map_err(|e| {
                        let err = CommonErrors::database_new(&e.to_string());
                        error!("{}", err.log());
                        err
                    })?;

                if identifier_model.is_none() {
                    self.negotiation_repo
                        .get_negotiation_process_identifiers_repo()
                        .create_identifier(&NewNegotiationIdentifierModel {
                            id: Some(get_urn(None)),
                            negotiation_agent_process_id: process_urn.clone(),
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

                    self.negotiation_repo
                        .get_negotiation_process_identifiers_repo()
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

    async fn delete_negotiation_process(&self, id: &Urn) -> anyhow::Result<()> {
        self.negotiation_repo.get_negotiation_process_repo().delete_negotiation_process(id).await.map_err(
            |e| match e {
                NegotiationProcessRepoErrors::NegotiationProcessNotFound => {
                    let err = CommonErrors::missing_resource_new(
                        &id.to_string(),
                        "Negotiation process not found for deletion",
                    );
                    error!("{}", err.log());
                    err
                }
                _ => {
                    let err = CommonErrors::database_new(&e.to_string());
                    error!("{}", err.log());
                    err
                }
            },
        )?;
        Ok(())
    }
}
