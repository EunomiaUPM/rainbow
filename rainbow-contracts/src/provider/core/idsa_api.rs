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
use crate::provider::core::idsa_api_errors::IdsaCNError;
use anyhow::bail;
use clap::builder::TypedValueParser;
use rainbow_common::config::config::ConfigRoles;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement_verification::ContractAgreementVerificationMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::{
    ContractNegotiationEventMessage, NegotiationEventType,
};
use rainbow_common::protocol::contract::contract_negotiation_request::ContractRequestMessage;
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::protocol::contract::contract_odrl::OfferTypes;
use rainbow_common::protocol::contract::ContractNegotiationState;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::contracts_provider::repo::{
    EditContractNegotiationProcess, NewContractNegotiationMessage, NewContractNegotiationOffer,
    NewContractNegotiationProcess, CONTRACT_PROVIDER_REPO,
};
use urn::Urn;

///
/// # Get negotiation by provider_pid
/// Returns a ContractAck with the state of CN negotiation
///
pub async fn get_negotiation(provider_pid: Urn) -> anyhow::Result<ContractAckMessage> {
    let cn_process = CONTRACT_PROVIDER_REPO
        .get_cn_processes_by_provider_id(&provider_pid)
        .await
        .map_err(IdsaCNError::DbErr)?
        .ok_or(IdsaCNError::ProcessNotFound {
            provider_pid: Some(provider_pid),
            consumer_pid: None,
        })?;

    Ok(cn_process.into())
}

///
/// # Consumer sends ContractRequestMessage
/// In this case is done from scratch
///
pub async fn post_request(input: ContractRequestMessage) -> anyhow::Result<ContractAckMessage> {
    // Semantic Validate ContractRequestMessage
    // TODO may be in middleware
    input.validate().map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;

    // TODO Check schema compliance in middleware
    if let Some(provider_pid) = input.provider_pid.clone() {
        bail!(IdsaCNError::ValidationError(format!(
            "Provider PID is not allowed in request. Found: {}",
            provider_pid
        )));
    }

    // TODO Check transition protocol validations
    // Check if consumer_pid exists in the database
    // all this in middleware

    let cn_process = CONTRACT_PROVIDER_REPO
        .create_cn_process(NewContractNegotiationProcess {
            provider_id: None,
            consumer_id: Option::from(input.consumer_pid.clone()),
            state: ContractNegotiationState::Requested,
            initiated_by: ConfigRoles::Consumer,
        })
        .await
        .map_err(IdsaCNError::DbErr)?;

    let cn_message = CONTRACT_PROVIDER_REPO
        .create_cn_message(
            get_urn_from_string(&cn_process.cn_process_id)?,
            NewContractNegotiationMessage {
                _type: input._type.to_string(),
                from: "Consumer".to_string(),
                to: "Provider".to_string(),
                content: serde_json::to_value(&input).unwrap(),
            },
        )
        .await
        .map_err(IdsaCNError::DbErr)?;

    let cn_offer = CONTRACT_PROVIDER_REPO
        .create_cn_offer(
            get_urn_from_string(&cn_process.cn_process_id)?,
            get_urn_from_string(&cn_message.cn_message_id)?,
            NewContractNegotiationOffer {
                offer_id: match input.odrl_offer {
                    OfferTypes::Offer(ref offer) => offer.id.clone(),
                    OfferTypes::MessageOffer(ref offer) => offer.id.clone(),
                    _ => bail!("Invalid offer type"),
                },
                offer_content: serde_json::to_value(&input.odrl_offer)?,
            },
        )
        .await
        .map_err(IdsaCNError::DbErr)?;

    Ok(cn_process.into())
}

///
/// # Consumer sends ContractRequestMessage
/// In this case is done from OFFERED state
///
pub async fn post_provider_request(
    provider_pid: Urn,
    input: ContractRequestMessage,
) -> anyhow::Result<ContractAckMessage> {
    println!("{:?}", provider_pid);
    println!("{:?}", input);

    // Semantic Validate ContractRequestMessage
    // TODO may be in middleware
    input.validate().map_err(|e| IdsaCNError::ValidationError(e.to_string()))?;

    // Check if provider_pid in input and path match
    if let Some(provider_pid_in_input) = input.provider_pid.clone() {
        if provider_pid_in_input != provider_pid {
            bail!(IdsaCNError::ValidationError(format!(
                "Provider PID in path and in request body do not match. Path: {}, Request: {}",
                provider_pid, provider_pid_in_input
            )));
        }
    }

    // TODO Check transition protocol validations
    // Check if consumer_pid exists in the database
    // all this in middleware
    let _ = CONTRACT_PROVIDER_REPO
        .get_cn_processes_by_consumer_id(input.consumer_pid.clone())
        .await?
        .ok_or(IdsaCNError::ProcessNotFound {
            provider_pid: Some(provider_pid.clone()),
            consumer_pid: Some(input.consumer_pid.clone()),
        })?;

    // Update CN process state and create message and offer
    let cn_process = CONTRACT_PROVIDER_REPO
        .get_cn_processes_by_provider_id(&provider_pid)
        .await
        .map_err(IdsaCNError::DbErr)?
        .ok_or(IdsaCNError::ProcessNotFound {
            provider_pid: Some(provider_pid),
            consumer_pid: Some(input.consumer_pid.clone()),
        })?;
    let _ = CONTRACT_PROVIDER_REPO
        .put_cn_process(
            get_urn_from_string(&cn_process.cn_process_id)?,
            EditContractNegotiationProcess {
                provider_id: None, // no need to change
                consumer_id: None,
                state: Option::from(ContractNegotiationState::Requested),
            },
        )
        .await
        .map_err(IdsaCNError::DbErr)?;

    let cn_message = CONTRACT_PROVIDER_REPO
        .create_cn_message(
            get_urn_from_string(&cn_process.cn_process_id)?,
            NewContractNegotiationMessage {
                _type: input._type.to_string(),
                from: "Consumer".to_string(),
                to: "Provider".to_string(),
                content: serde_json::to_value(&input).unwrap(),
            },
        )
        .await
        .map_err(IdsaCNError::DbErr)?;

    let cn_offer = CONTRACT_PROVIDER_REPO
        .create_cn_offer(
            get_urn_from_string(&cn_process.cn_process_id)?,
            get_urn_from_string(&cn_message.cn_message_id)?,
            // TODO review this...
            NewContractNegotiationOffer {
                offer_id: match input.odrl_offer {
                    OfferTypes::Offer(ref offer) => offer.id.clone(),
                    OfferTypes::MessageOffer(ref offer) => offer.id.clone(),
                    _ => bail!("Invalid offer type"),
                },
                offer_content: serde_json::to_value(&input.odrl_offer)?,
            },
        )
        .await
        .map_err(IdsaCNError::DbErr)?;

    Ok(cn_process.into())
}

///
/// # Consumer sends acceptance of negotiation
///
pub async fn post_provider_events(
    provider_pid: &Urn,
    input: &ContractNegotiationEventMessage,
) -> anyhow::Result<ContractAckMessage> {
    let ContractNegotiationEventMessage { _type, consumer_pid, event_type, .. } = input;

    // Verify whether CN process was instantiated by the consumer
    let cn_process = CONTRACT_PROVIDER_REPO
        .get_cn_processes_by_provider_id(&provider_pid)
        .await
        .map_err(IdsaCNError::DbErr)?
        .ok_or(IdsaCNError::ProcessNotFound {
            provider_pid: Some(provider_pid.clone()),
            consumer_pid: Some(input.consumer_pid.clone()),
        })?;
    let initiated_by = cn_process.initiated_by.parse::<ConfigRoles>().map_err(|e| {
        IdsaCNError::NotCheckedError {
            provider_pid: Option::from(provider_pid.clone().to_string()),
            consumer_pid: Some(input.consumer_pid.clone().to_string()),
            error: e.to_string(),
        }
    })?;
    match (initiated_by, event_type) {
        (ConfigRoles::Consumer, NegotiationEventType::Accepted) => {}
        _ => {
            return Err(IdsaCNError::NotAllowed {
                provider_pid: Option::from(provider_pid.clone()),
                consumer_pid: Some(input.consumer_pid.clone()),
                error: "This message is not allowed".to_string(),
            }
                .into())
        }
    };

    // Update CN process state
    let cn_process = CONTRACT_PROVIDER_REPO
        .put_cn_process(
            get_urn_from_string(&cn_process.cn_process_id)?,
            EditContractNegotiationProcess {
                provider_id: None, // no need to change
                consumer_id: None,
                state: Some(input.event_type.clone().into()),
            },
        )
        .await
        .map_err(IdsaCNError::DbErr)?;
    // Create message
    let cn_message = CONTRACT_PROVIDER_REPO
        .create_cn_message(
            get_urn_from_string(&cn_process.cn_process_id)?,
            NewContractNegotiationMessage {
                _type: input._type.to_string(),
                from: "Consumer".to_string(),
                to: "Provider".to_string(),
                content: serde_json::to_value(&input).unwrap(),
            },
        )
        .await
        .map_err(IdsaCNError::DbErr)?;
    
    Ok(cn_process.into())
}

///
/// # Consumer sends agreement verification
///
pub async fn post_provider_agreement_verification(
    provider_id: Urn,
    input: ContractAgreementVerificationMessage,
) -> anyhow::Result<ContractAckMessage> {
    let ContractAgreementVerificationMessage { provider_pid, consumer_pid, _type, .. } =
        input.clone();
    let cn_process = CONTRACT_PROVIDER_REPO
        .get_cn_processes_by_provider_id(&provider_id)
        .await
        .map_err(IdsaCNError::DbErr)?
        .ok_or(IdsaCNError::ProcessNotFound {
            provider_pid: Some(provider_pid.clone().parse()?),
            consumer_pid: Some(consumer_pid.clone().parse()?),
        })?;

    let cn_process = CONTRACT_PROVIDER_REPO
        .put_cn_process(
            get_urn_from_string(&cn_process.cn_process_id)?,
            EditContractNegotiationProcess {
                provider_id: None, // no need to change
                consumer_id: None,
                state: Some(ContractNegotiationState::Verified),
            },
        )
        .await
        .map_err(IdsaCNError::DbErr)?;
    // Create message
    let cn_message = CONTRACT_PROVIDER_REPO
        .create_cn_message(
            get_urn_from_string(&cn_process.cn_process_id)?,
            NewContractNegotiationMessage {
                _type: _type.to_string(),
                from: "Consumer".to_string(),
                to: "Provider".to_string(),
                content: serde_json::to_value(&input).unwrap(),
            },
        )
        .await
        .map_err(IdsaCNError::DbErr)?;

    Ok(cn_process.into())
}

///
/// # Consumer sends termination
///
pub async fn post_provider_termination(
    provider_id: Urn,
    input: ContractTerminationMessage,
) -> anyhow::Result<ContractAckMessage> {
    let ContractTerminationMessage { provider_pid, consumer_pid, _type, .. } = input.clone();
    let cn_process = CONTRACT_PROVIDER_REPO
        .get_cn_processes_by_provider_id(&provider_id)
        .await
        .map_err(IdsaCNError::DbErr)?
        .ok_or(IdsaCNError::ProcessNotFound {
            provider_pid: Some(provider_pid.clone()),
            consumer_pid: Some(consumer_pid.clone()),
        })?;

    let cn_process = CONTRACT_PROVIDER_REPO
        .put_cn_process(
            get_urn_from_string(&cn_process.cn_process_id)?,
            EditContractNegotiationProcess {
                provider_id: None, // no need to change
                consumer_id: None,
                state: Some(ContractNegotiationState::Terminated),
            },
        )
        .await
        .map_err(IdsaCNError::DbErr)?;
    // Create message
    let cn_message = CONTRACT_PROVIDER_REPO
        .create_cn_message(
            get_urn_from_string(&cn_process.cn_process_id)?,
            NewContractNegotiationMessage {
                _type: _type.to_string(),
                from: "Consumer".to_string(),
                to: "Provider".to_string(),
                content: serde_json::to_value(&input).unwrap(),
            },
        )
        .await
        .map_err(IdsaCNError::DbErr)?;

    Ok(cn_process.into())
}
