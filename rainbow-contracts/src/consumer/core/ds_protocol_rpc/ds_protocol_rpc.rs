use crate::consumer::core::ds_protocol::ds_protocol_errors::IdsaCNError;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_errors::DSRPCContractNegotiationConsumerErrors;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::{SetupAcceptanceRequest, SetupAcceptanceResponse, SetupRequestRequest, SetupRequestResponse, SetupTerminationRequest, SetupTerminationResponse, SetupVerificationRequest, SetupVerificationResponse};
use crate::consumer::core::ds_protocol_rpc::DSRPCContractNegotiationConsumerTrait;
use crate::consumer::core::rainbow_entities::rainbow_entities_errors::CnErrorConsumer;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement_verification::ContractAgreementVerificationMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::{ContractNegotiationEventMessage, NegotiationEventType};
use rainbow_common::protocol::contract::contract_negotiation_request::ContractRequestMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::protocol::contract::contract_odrl::OfferTypes;
use rainbow_common::protocol::ProtocolValidate;
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_db::contracts_consumer::entities::cn_process;
use rainbow_db::contracts_consumer::repo::{ContractNegotiationConsumerProcessRepo, NewContractNegotiationProcess};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;

pub struct DSRPCContractNegotiationConsumerService<T>
where
    T: ContractNegotiationConsumerProcessRepo + Send + Sync + 'static,
{
    repo: Arc<T>,
    client: Client,
}

impl<T> DSRPCContractNegotiationConsumerService<T>
where
    T: ContractNegotiationConsumerProcessRepo + Send + Sync + 'static,
{
    pub fn new(repo: Arc<T>) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { repo, client }
    }
}

#[async_trait]
impl<T> DSRPCContractNegotiationConsumerTrait for DSRPCContractNegotiationConsumerService<T>
where
    T: ContractNegotiationConsumerProcessRepo + Send + Sync + 'static,
{
    async fn setup_request(&self, input: SetupRequestRequest) -> anyhow::Result<SetupRequestResponse> {
        let SetupRequestRequest {
            provider_pid,
            consumer_pid,
            odrl_offer,
            provider_address,
            ..
        } = input;
        let is_re_request = provider_pid.is_some() && consumer_pid.is_some();

        if is_re_request {
            // validate consumer, provider and correlation
            let consumer = self.repo
                .get_cn_process_by_consumer_id(consumer_pid.clone().unwrap())
                .await
                .map_err(IdsaCNError::DbErr)?
                .ok_or(IdsaCNError::ProcessNotFound {
                    provider_pid: provider_pid.clone(),
                    consumer_pid: consumer_pid.clone(),
                })?;
            let provider = self.repo
                .get_cn_process_by_provider_id(provider_pid.clone().unwrap())
                .await
                .map_err(IdsaCNError::DbErr)?
                .ok_or(IdsaCNError::ProcessNotFound {
                    provider_pid: provider_pid.clone(),
                    consumer_pid: consumer_pid.clone(),
                })?;
            if consumer.cn_process_id != provider.cn_process_id {
                bail!(IdsaCNError::ValidationError("ProviderPid and ConsumerPid don't coincide".to_string()));
            }
        }

        // validate offer types
        let is_offer_err = match odrl_offer.clone() {
            OfferTypes::MessageOffer(message_offer) => message_offer.validate().is_err(),
            OfferTypes::Offer(offer) => offer.validate().is_err(),
            OfferTypes::Other(_) => false
        };
        if is_offer_err {
            bail!(IdsaCNError::ValidationError("Offer not valid".to_string()));
        }

        // create message
        let contract_offer_message = ContractRequestMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone().unwrap_or(get_urn(None)),
            odrl_offer: match odrl_offer.clone() {
                OfferTypes::MessageOffer(message_offer) => OfferTypes::MessageOffer(message_offer),
                OfferTypes::Offer(offer) => OfferTypes::Offer(offer),
                _ => {
                    bail!(IdsaCNError::ValidationError("Offer not valid".to_string()));
                }
            },
            ..Default::default()
        };

        // send message to provider
        let provider_base_url = provider_address.strip_suffix('/').unwrap_or(provider_address.as_str());
        let provider_url = format!("{}/negotiations/{}request", provider_base_url, match is_re_request {
            true => format!("{}/", provider_pid.clone().unwrap().to_string()),
            false => "".to_string()
        });
        let req = self.client
            .post(provider_url)
            .json(&contract_offer_message)
            .send()
            .await
            .map_err(|_| DSRPCContractNegotiationConsumerErrors::ConsumerNotReachable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        let status = req.status();
        if status.is_success() == false {
            bail!(DSRPCContractNegotiationConsumerErrors::ConsumerInternalError {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone())
            });
        }

        // response
        let response = req.json::<ContractAckMessage>()
            .await
            .map_err(|_| DSRPCContractNegotiationConsumerErrors::ConsumerResponseNotSerializable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;

        // persist cn_process
        let cn_process: cn_process::Model;
        if is_re_request == false {
            cn_process = self.repo
                .create_cn_process(NewContractNegotiationProcess {
                    provider_id: Some(get_urn_from_string(&response.provider_pid)?),
                    consumer_id: Some(get_urn_from_string(&response.consumer_pid)?),
                })
                .await
                .map_err(CnErrorConsumer::DbErr)?;
        } else {
            cn_process = self.repo
                .get_cn_process_by_consumer_id(consumer_pid.clone().unwrap())
                .await
                .map_err(CnErrorConsumer::DbErr)?
                .ok_or(CnErrorConsumer::NotFound {
                    id: consumer_pid.clone().unwrap(),
                    entity: "Consumer".to_string(),
                })?; // errors
        }

        // response
        let cn_ack: ContractAckMessage = cn_process.into();
        let response = SetupRequestResponse {
            consumer_pid: Option::from(get_urn_from_string(&response.consumer_pid)?),
            provider_pid: Option::from(get_urn_from_string(&response.provider_pid)?),
            odrl_offer: odrl_offer.clone(),
            message: cn_ack,
        };
        Ok(response)
    }

    async fn setup_acceptance(&self, input: SetupAcceptanceRequest) -> anyhow::Result<SetupAcceptanceResponse> {
        let SetupAcceptanceRequest {
            provider_pid,
            consumer_pid,
            provider_address,
            ..
        } = input;
        // validate consumer, provider and correlation
        let consumer = self.repo
            .get_cn_process_by_consumer_id(consumer_pid.clone())
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        let provider = self.repo
            .get_cn_process_by_provider_id(provider_pid.clone())
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        if consumer.cn_process_id != provider.cn_process_id {
            bail!(IdsaCNError::ValidationError("ProviderPid and ConsumerPid don't coincide".to_string()));
        }

        // create message
        let contract_acceptance_message = ContractNegotiationEventMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            event_type: NegotiationEventType::Accepted,
            ..Default::default()
        };

        // send message to provider
        let provider_base_url = provider_address.strip_suffix('/').unwrap_or(provider_address.as_str());
        let provider_url = format!("{}/negotiations/{}/events", provider_base_url, provider_pid.clone());
        let req = self.client
            .post(provider_url)
            .json(&contract_acceptance_message)
            .send()
            .await
            .map_err(|_| DSRPCContractNegotiationConsumerErrors::ConsumerNotReachable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        let status = req.status();
        if status.is_success() == false {
            bail!(DSRPCContractNegotiationConsumerErrors::ConsumerInternalError {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone())
            });
        }
        // response
        let response = req.json::<ContractAckMessage>()
            .await
            .map_err(|_| DSRPCContractNegotiationConsumerErrors::ConsumerResponseNotSerializable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        let response = SetupAcceptanceResponse {
            consumer_pid: get_urn_from_string(&response.consumer_pid)?,
            provider_pid: get_urn_from_string(&response.provider_pid)?,
            message: response,
        };
        Ok(response)
    }

    async fn setup_verification(&self, input: SetupVerificationRequest) -> anyhow::Result<SetupVerificationResponse> {
        let SetupVerificationRequest {
            provider_pid,
            consumer_pid,
            provider_address,
            ..
        } = input;
        // validate consumer, provider and correlation
        let consumer = self.repo
            .get_cn_process_by_consumer_id(consumer_pid.clone())
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        let provider = self.repo
            .get_cn_process_by_provider_id(provider_pid.clone())
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        if consumer.cn_process_id != provider.cn_process_id {
            bail!(IdsaCNError::ValidationError("ProviderPid and ConsumerPid don't coincide".to_string()));
        }

        // create message
        let contract_verification_message = ContractAgreementVerificationMessage {
            provider_pid: provider_pid.clone().to_string(),
            consumer_pid: consumer_pid.clone().to_string(),
            ..Default::default()
        };

        // send message to provider
        let provider_base_url = provider_address.strip_suffix('/').unwrap_or(provider_address.as_str());
        let provider_url = format!("{}/negotiations/{}/agreement/verification", provider_base_url, provider_pid.clone());
        let req = self.client
            .post(provider_url)
            .json(&contract_verification_message)
            .send()
            .await
            .map_err(|_| DSRPCContractNegotiationConsumerErrors::ConsumerNotReachable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        let status = req.status();
        if status.is_success() == false {
            bail!(DSRPCContractNegotiationConsumerErrors::ConsumerInternalError {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone())
            });
        }
        // response
        let response = req.json::<ContractAckMessage>()
            .await
            .map_err(|_| DSRPCContractNegotiationConsumerErrors::ConsumerResponseNotSerializable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        let response = SetupVerificationResponse {
            consumer_pid: get_urn_from_string(&response.consumer_pid)?,
            provider_pid: get_urn_from_string(&response.provider_pid)?,
            message: response,
        };
        Ok(response)
    }

    async fn setup_termination(&self, input: SetupTerminationRequest) -> anyhow::Result<SetupTerminationResponse> {
        let SetupTerminationRequest {
            provider_pid,
            consumer_pid,
            provider_address,
            ..
        } = input;
        // validate consumer, provider and correlation
        let consumer = self.repo
            .get_cn_process_by_consumer_id(consumer_pid.clone())
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        let provider = self.repo
            .get_cn_process_by_provider_id(provider_pid.clone())
            .await
            .map_err(IdsaCNError::DbErr)?
            .ok_or(IdsaCNError::ProcessNotFound {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        if consumer.cn_process_id != provider.cn_process_id {
            bail!(IdsaCNError::ValidationError("ProviderPid and ConsumerPid don't coincide".to_string()));
        }

        // create message
        let contract_termination_message = ContractTerminationMessage {
            provider_pid: provider_pid.clone(),
            consumer_pid: consumer_pid.clone(),
            ..Default::default()
        };

        // send message to provider
        let provider_base_url = provider_address.strip_suffix('/').unwrap_or(provider_address.as_str());
        let provider_url = format!("{}/negotiations/{}/agreement/verification", provider_base_url, provider_pid.clone());
        let req = self.client
            .post(provider_url)
            .json(&contract_termination_message)
            .send()
            .await
            .map_err(|_| DSRPCContractNegotiationConsumerErrors::ConsumerNotReachable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        let status = req.status();
        if status.is_success() == false {
            bail!(DSRPCContractNegotiationConsumerErrors::ConsumerInternalError {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone())
            });
        }
        // response
        let response = req.json::<ContractAckMessage>()
            .await
            .map_err(|_| DSRPCContractNegotiationConsumerErrors::ConsumerResponseNotSerializable {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Option::from(consumer_pid.clone()),
            })?;
        let response = SetupTerminationResponse {
            consumer_pid: get_urn_from_string(&response.consumer_pid)?,
            provider_pid: get_urn_from_string(&response.provider_pid)?,
            message: response,
        };
        Ok(response)
    }
}
