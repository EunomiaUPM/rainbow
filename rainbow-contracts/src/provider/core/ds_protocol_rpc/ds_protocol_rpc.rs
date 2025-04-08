use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_errors::DSRPCContractNegotiationProviderErrors;
use crate::provider::core::ds_protocol_rpc::ds_protocol_rpc_types::{SetupAgreementRequest, SetupAgreementResponse, SetupFinalizationRequest, SetupFinalizationResponse, SetupOfferRequest, SetupOfferResponse, SetupTerminationRequest, SetupTerminationResponse};
use crate::provider::core::ds_protocol_rpc::DSRPCContractNegotiationProviderTrait;
use crate::provider::core::rainbow_entities::rainbow_entities_errors::CnErrorProvider;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::config::config::ConfigRoles;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement::ContractAgreementMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::{ContractNegotiationEventMessage, NegotiationEventType};
use rainbow_common::protocol::contract::contract_offer::ContractOfferMessage;
use rainbow_common::protocol::contract::ContractNegotiationMessages;
use rainbow_common::protocol::ProtocolValidate;
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_db::contracts_provider::entities::cn_process;
use rainbow_db::contracts_provider::repo::{AgreementRepo, ContractNegotiationMessageRepo, ContractNegotiationOfferRepo, ContractNegotiationProcessRepo, EditContractNegotiationProcess, NewContractNegotiationMessage, NewContractNegotiationOffer, NewContractNegotiationProcess, Participant};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;

pub struct DSRPCContractNegotiationProviderService<T>
where
    T: ContractNegotiationProcessRepo
    + ContractNegotiationMessageRepo
    + ContractNegotiationOfferRepo
    + AgreementRepo
    + Participant
    + Send
    + Sync
    + 'static,
{
    repo: Arc<T>,
    client: Client,
}

impl<T> DSRPCContractNegotiationProviderService<T>
where
    T: ContractNegotiationProcessRepo
    + ContractNegotiationMessageRepo
    + ContractNegotiationOfferRepo
    + AgreementRepo
    + Participant
    + Send
    + Sync
    + 'static,
{
    pub fn new(repo: Arc<T>) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { repo, client }
    }
}

#[async_trait]
impl<T> DSRPCContractNegotiationProviderTrait for DSRPCContractNegotiationProviderService<T>
where
    T: ContractNegotiationProcessRepo
    + ContractNegotiationMessageRepo
    + ContractNegotiationOfferRepo
    + AgreementRepo
    + Participant
    + Send
    + Sync
    + 'static,
{
    async fn setup_offer(&self, input: SetupOfferRequest) -> anyhow::Result<SetupOfferResponse> {
        let SetupOfferRequest {
            consumer_pid,
            provider_pid,
            odrl_offer,
            consumer_participant_id,
            ..
        } = input;

        // This message could be from scratch or part of a negotiation
        let is_reoffer = provider_pid.clone().is_some() && consumer_pid.clone().is_some();

        // validate consumer_participant_id
        let participant = self.repo.get_participant_by_p_id(consumer_participant_id.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::NotFound {
                id: consumer_participant_id.clone(),
                entity: "Participant".to_string(),
            })?;
        let consumer_base_url = participant.base_url;

        if is_reoffer {
            // validate consumer
            let consumer = self.repo
                .get_cn_processes_by_consumer_id(consumer_pid.clone().unwrap())
                .await
                .map_err(CnErrorProvider::DbErr)?
                .ok_or(CnErrorProvider::NotFound {
                    id: consumer_participant_id.clone(),
                    entity: "Consumer".to_string(),
                })?;

            // validate provider
            let provider = self.repo
                .get_cn_processes_by_provider_id(&provider_pid.clone().unwrap())
                .await
                .map_err(CnErrorProvider::DbErr)?
                .ok_or(CnErrorProvider::NotFound {
                    id: consumer_participant_id.clone(),
                    entity: "Provider".to_string(),
                })?;

            // validate correlation
            if consumer.cn_process_id != provider.cn_process_id {
                bail!(DSRPCContractNegotiationProviderErrors::ConsumerAndProviderCorrelationError {
                    provider_pid: get_urn_from_string(&provider.cn_process_id)?,
                    consumer_pid: get_urn_from_string(&consumer.cn_process_id)?,
                });
            }
        }

        // validate offer
        if odrl_offer.validate().is_err() {
            bail!(DSRPCContractNegotiationProviderErrors::OdrlValidationError);
        }

        // create message
        let provider_pid = get_urn(None);
        let contract_offer_message = ContractOfferMessage {
            provider_pid: provider_pid.to_string(),
            odrl_offer: odrl_offer.clone(),
            ..Default::default()
        };

        // send message to consumer
        let consumer_url = format!("{}/negotiations/{}offers", consumer_base_url, match is_reoffer {
            true => format!("{}/", consumer_pid.clone().unwrap()),
            false => "".to_string()
        });
        let req = self.client
            .post(consumer_url)
            .json(&contract_offer_message)
            .send()
            .await
            .map_err(|_| DSRPCContractNegotiationProviderErrors::ConsumerNotReachable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        let status = req.status();
        if status.is_success() == false {
            bail!(DSRPCContractNegotiationProviderErrors::ConsumerInternalError {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone())
            });
        }

        // response
        let response = req.json::<ContractAckMessage>()
            .await
            .map_err(|_| DSRPCContractNegotiationProviderErrors::ConsumerResponseNotSerializable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;


        let mut cn_process: cn_process::Model;
        if is_reoffer == false {
            cn_process = self.repo
                .create_cn_process(NewContractNegotiationProcess {
                    provider_id: Some(get_urn_from_string(&response.provider_pid)?),
                    consumer_id: Some(get_urn_from_string(&response.consumer_pid)?),
                    state: response.state,
                    initiated_by: ConfigRoles::Provider,
                })
                .await
                .map_err(CnErrorProvider::DbErr)?;
        } else {
            cn_process = self.repo
                .get_cn_processes_by_consumer_id(consumer_pid.clone().unwrap())
                .await
                .map_err(CnErrorProvider::DbErr)?
                .ok_or(CnErrorProvider::NotFound {
                    id: consumer_pid.clone().unwrap(),
                    entity: "Consumer".to_string(),
                })?;
            cn_process = self.repo
                .put_cn_process(cn_process.cn_process_id.parse()?, EditContractNegotiationProcess {
                    provider_id: None,
                    consumer_id: None,
                    state: Some(response.state),
                })
                .await
                .map_err(CnErrorProvider::DbErr)?;
        }


        // persist cn_message
        let cn_message = self.repo
            .create_cn_message(
                cn_process.cn_process_id.parse().unwrap(),
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractOfferMessage.to_string(),
                    from: ConfigRoles::Provider.to_string(),
                    to: ConfigRoles::Consumer.to_string(),
                    content: serde_json::to_value(contract_offer_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;

        // persist cn_offer
        let _ = self.repo
            .create_cn_offer(
                cn_process.cn_process_id.parse().unwrap(),
                cn_message.cn_message_id.parse().unwrap(),
                NewContractNegotiationOffer {
                    offer_id: get_urn(None),
                    offer_content: serde_json::to_value(odrl_offer.clone()).unwrap(),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;

        // Create response
        let cn_ack: ContractAckMessage = cn_process.into();
        let response: SetupOfferResponse;
        if is_reoffer == false {
            response = SetupOfferResponse {
                consumer_participant_id: consumer_participant_id.clone(),
                consumer_pid: Some(cn_ack.consumer_pid.clone().parse()?),
                provider_pid: Some(cn_ack.provider_pid.clone().parse()?),
                odrl_offer: odrl_offer.clone(),
                message: cn_ack,
            };
        } else {
            response = SetupOfferResponse {
                consumer_participant_id: consumer_participant_id.clone(),
                consumer_pid,
                provider_pid: Option::from(provider_pid),
                odrl_offer: odrl_offer.clone(),
                message: cn_ack,
            };
        }

        Ok(response)
    }

    async fn setup_agreement(&self, input: SetupAgreementRequest) -> anyhow::Result<SetupAgreementResponse> {
        let SetupAgreementRequest { consumer_participant_id, consumer_pid, provider_pid, odrl_agreement, .. } = input;
        // validate consumerParticipant
        let participant = self.repo
            .get_participant_by_p_id(consumer_participant_id.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::NotFound {
                id: consumer_participant_id.clone(),
                entity: "Participant".to_string(),
            })?;
        let consumer_base_url = participant.base_url;

        // validate consumer
        let consumer = self.repo
            .get_cn_processes_by_consumer_id(consumer_pid.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::NotFound {
                id: consumer_participant_id.clone(),
                entity: "Consumer".to_string(),
            })?;

        // validate provider
        let provider = self.repo
            .get_cn_processes_by_provider_id(&provider_pid.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::NotFound {
                id: consumer_participant_id.clone(),
                entity: "Provider".to_string(),
            })?;

        // validate correlation
        if consumer.cn_process_id != provider.cn_process_id {
            bail!(DSRPCContractNegotiationProviderErrors::ConsumerAndProviderCorrelationError {
                    provider_pid: get_urn_from_string(&provider.cn_process_id)?,
                    consumer_pid: get_urn_from_string(&consumer.cn_process_id)?,
                });
        }

        // validate offer
        if odrl_agreement.validate().is_err() {
            bail!(DSRPCContractNegotiationProviderErrors::OdrlValidationError);
        }

        // create message
        let contract_agreement_message = ContractAgreementMessage {
            provider_pid: provider_pid.to_string(),
            consumer_pid: consumer_pid.to_string(),
            odrl_agreement: odrl_agreement.clone(),
            ..Default::default()
        };

        // send message to consumer
        let consumer_url = format!("{}/negotiations/{}/agreement", consumer_base_url, consumer_pid.clone());
        let req = self.client
            .post(consumer_url)
            .json(&contract_agreement_message)
            .send()
            .await
            .map_err(|_| DSRPCContractNegotiationProviderErrors::ConsumerNotReachable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;

        // if status is not CREATED, return error
        let status = req.status();
        if status.is_success() == false {
            bail!(DSRPCContractNegotiationProviderErrors::ConsumerInternalError {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone())
            });
        }

        // response
        let response = req.json::<ContractAckMessage>()
            .await
            .map_err(|_| DSRPCContractNegotiationProviderErrors::ConsumerResponseNotSerializable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;

        // persist cn_process
        let process_id = get_urn_from_string(&provider.cn_process_id.clone())?;
        let cn_process = self.repo
            .put_cn_process(
                process_id,
                EditContractNegotiationProcess {
                    provider_id: None,
                    consumer_id: None,
                    state: Option::from(response.state),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;

        // persist cn_message
        let cn_message = self.repo
            .create_cn_message(
                cn_process.cn_process_id.parse().unwrap(),
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractAgreementMessage.to_string(),
                    from: ConfigRoles::Provider.to_string(),
                    to: ConfigRoles::Consumer.to_string(),
                    content: serde_json::to_value(contract_agreement_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;

        // persist cn_offer
        let _ = self.repo
            .create_cn_offer(
                cn_process.cn_process_id.parse().unwrap(),
                cn_message.cn_message_id.parse().unwrap(),
                NewContractNegotiationOffer {
                    offer_id: get_urn(None),
                    offer_content: serde_json::to_value(odrl_agreement.clone()).unwrap(),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;

        let cn_ack: ContractAckMessage = cn_process.into();
        let response = SetupAgreementResponse {
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
            odrl_agreement: odrl_agreement.clone(),
            message: cn_ack,
        };

        Ok(response)
    }

    async fn setup_finalization(&self, input: SetupFinalizationRequest) -> anyhow::Result<SetupFinalizationResponse> {
        let SetupFinalizationRequest { consumer_participant_id, consumer_pid, provider_pid, .. } = input;

        // validate consumerParticipant
        let participant = self.repo
            .get_participant_by_p_id(consumer_participant_id.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::NotFound {
                id: consumer_participant_id.clone(),
                entity: "Participant".to_string(),
            })?;
        let consumer_base_url = participant.base_url;

        // validate consumer
        let consumer = self.repo
            .get_cn_processes_by_consumer_id(consumer_pid.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::NotFound {
                id: consumer_participant_id.clone(),
                entity: "Consumer".to_string(),
            })?;

        // validate provider
        let provider = self.repo
            .get_cn_processes_by_provider_id(&provider_pid.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::NotFound {
                id: consumer_participant_id.clone(),
                entity: "Provider".to_string(),
            })?;

        // validate correlation
        if consumer.cn_process_id != provider.cn_process_id {
            bail!(DSRPCContractNegotiationProviderErrors::ConsumerAndProviderCorrelationError {
                    provider_pid: get_urn_from_string(&provider.cn_process_id)?,
                    consumer_pid: get_urn_from_string(&consumer.cn_process_id)?,
                });
        }


        // create message
        let contract_verification_message = ContractNegotiationEventMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            event_type: NegotiationEventType::Finalized,
            ..Default::default()
        };

        // send message to consumer
        let consumer_url = format!("{}/negotiations/{}/events", consumer_base_url, consumer_pid.clone());
        let req = self.client
            .post(consumer_url)
            .json(&contract_verification_message)
            .send()
            .await
            .map_err(|_| DSRPCContractNegotiationProviderErrors::ConsumerNotReachable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;

        // if status is not CREATED, return error
        let status = req.status();
        if status.is_success() == false {
            bail!(DSRPCContractNegotiationProviderErrors::ConsumerInternalError {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone())
            });
        }

        // response
        let response = req.json::<ContractAckMessage>()
            .await
            .map_err(|_| DSRPCContractNegotiationProviderErrors::ConsumerResponseNotSerializable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;

        // persist cn_process
        let process_id = get_urn_from_string(&provider.cn_process_id.clone())?;
        let cn_process = self.repo
            .put_cn_process(
                process_id,
                EditContractNegotiationProcess {
                    provider_id: None,
                    consumer_id: None,
                    state: Option::from(response.state),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;

        // persist cn_message
        let _ = self.repo
            .create_cn_message(
                cn_process.cn_process_id.parse().unwrap(),
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractNegotiationEventMessage.to_string(),
                    from: ConfigRoles::Provider.to_string(),
                    to: ConfigRoles::Consumer.to_string(),
                    content: serde_json::to_value(contract_verification_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;


        let cn_ack: ContractAckMessage = cn_process.into();
        let response = SetupFinalizationResponse {
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
            message: cn_ack,
        };

        Ok(response)
    }

    async fn setup_termination(&self, input: SetupTerminationRequest) -> anyhow::Result<SetupTerminationResponse> {
        let SetupTerminationRequest { consumer_participant_id, consumer_pid, provider_pid, .. } = input;

        // validate consumerParticipant
        let participant = self.repo
            .get_participant_by_p_id(consumer_participant_id.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::NotFound {
                id: consumer_participant_id.clone(),
                entity: "Participant".to_string(),
            })?;
        let consumer_base_url = participant.base_url;

        // validate consumer
        let consumer = self.repo
            .get_cn_processes_by_consumer_id(consumer_pid.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::NotFound {
                id: consumer_participant_id.clone(),
                entity: "Consumer".to_string(),
            })?;

        // validate provider
        let provider = self.repo
            .get_cn_processes_by_provider_id(&provider_pid.clone())
            .await
            .map_err(CnErrorProvider::DbErr)?
            .ok_or(CnErrorProvider::NotFound {
                id: consumer_participant_id.clone(),
                entity: "Provider".to_string(),
            })?;

        // validate correlation
        if consumer.cn_process_id != provider.cn_process_id {
            bail!(DSRPCContractNegotiationProviderErrors::ConsumerAndProviderCorrelationError {
                    provider_pid: get_urn_from_string(&provider.cn_process_id)?,
                    consumer_pid: get_urn_from_string(&consumer.cn_process_id)?,
                });
        }


        // create message
        let contract_termination_message = ContractNegotiationEventMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            ..Default::default()
        };

        // send message to consumer
        let consumer_url = format!("{}/negotiations/{}/termination", consumer_base_url, consumer_pid.clone());
        let req = self.client
            .post(consumer_url)
            .json(&contract_termination_message)
            .send()
            .await
            .map_err(|_| DSRPCContractNegotiationProviderErrors::ConsumerNotReachable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;

        // if status is not CREATED, return error
        let status = req.status();
        if status.is_success() == false {
            bail!(DSRPCContractNegotiationProviderErrors::ConsumerInternalError {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone())
            });
        }

        // response
        let response = req.json::<ContractAckMessage>()
            .await
            .map_err(|_| DSRPCContractNegotiationProviderErrors::ConsumerResponseNotSerializable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;

        // persist cn_process
        let process_id = get_urn_from_string(&provider.cn_process_id.clone())?;
        let cn_process = self.repo
            .put_cn_process(
                process_id,
                EditContractNegotiationProcess {
                    provider_id: None,
                    consumer_id: None,
                    state: Option::from(response.state),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;

        // persist cn_message
        let _ = self.repo
            .create_cn_message(
                cn_process.cn_process_id.parse().unwrap(),
                NewContractNegotiationMessage {
                    _type: ContractNegotiationMessages::ContractNegotiationTerminationMessage.to_string(),
                    from: ConfigRoles::Provider.to_string(),
                    to: ConfigRoles::Consumer.to_string(),
                    content: serde_json::to_value(contract_termination_message).unwrap(),
                },
            )
            .await
            .map_err(CnErrorProvider::DbErr)?;


        let cn_ack: ContractAckMessage = cn_process.into();
        let response = SetupTerminationResponse {
            consumer_pid: consumer_pid.clone(),
            provider_pid: provider_pid.clone(),
            message: cn_ack,
        };

        Ok(response)
    }
}
