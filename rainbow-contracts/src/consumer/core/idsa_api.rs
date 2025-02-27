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
use crate::consumer::core::idsa_api_errors::IdsaCNError;
use anyhow::bail;
use rainbow_common::protocol::contract::contract_ack::ContractAckMessage;
use rainbow_common::protocol::contract::contract_agreement::ContractAgreementMessage;
use rainbow_common::protocol::contract::contract_negotiation_event::{ContractNegotiationEventMessage, NegotiationEventType};
use rainbow_common::protocol::contract::contract_negotiation_termination::ContractTerminationMessage;
use rainbow_common::protocol::contract::contract_offer::ContractOfferMessage;
use rainbow_common::protocol::contract::ContractNegotiationState;
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_db::contracts_consumer::repo::{NewContractNegotiationProcess, CONTRACT_CONSUMER_REPO};
use urn::Urn;

pub async fn post_offers(
    input: ContractOfferMessage
) -> anyhow::Result<ContractAckMessage> {
    let cn_process = CONTRACT_CONSUMER_REPO
        .create_cn_process(NewContractNegotiationProcess {
            provider_id: Some(get_urn_from_string(&input.provider_pid)?),
            consumer_id: Some(get_urn(None)),
        })
        .await
        .map_err(IdsaCNError::DbErr)?;
    let mut cn_ack: ContractAckMessage = cn_process.into();
    cn_ack.state = ContractNegotiationState::Offered;
    Ok(cn_ack)
}


pub async fn post_consumer_offers(
    consumer_pid: Urn,
    input: ContractOfferMessage,
) -> anyhow::Result<ContractAckMessage> {
    let cn_process = CONTRACT_CONSUMER_REPO
        .get_cn_process_by_consumer_id(consumer_pid.clone())
        .await
        .map_err(IdsaCNError::DbErr)?
        .ok_or(IdsaCNError::ProcessNotFound {
            provider_pid: Option::from(get_urn_from_string(&input.provider_pid)?),
            consumer_pid: Some(consumer_pid),
        })?;

    let mut cn_ack: ContractAckMessage = cn_process.into();
    cn_ack.state = ContractNegotiationState::Offered;
    Ok(cn_ack)
}

pub async fn post_agreement(
    consumer_pid: Urn,
    input: ContractAgreementMessage,
) -> anyhow::Result<ContractAckMessage> {
    let cn_process = CONTRACT_CONSUMER_REPO
        .get_cn_process_by_consumer_id(consumer_pid.clone())
        .await
        .map_err(IdsaCNError::DbErr)?
        .ok_or(IdsaCNError::ProcessNotFound {
            provider_pid: Option::from(get_urn_from_string(&input.provider_pid)?),
            consumer_pid: Some(consumer_pid),
        })?;

    let mut cn_ack: ContractAckMessage = cn_process.into();
    cn_ack.state = ContractNegotiationState::Agreed;
    Ok(cn_ack)
}

pub async fn post_events(
    consumer_pid: Urn,
    input: ContractNegotiationEventMessage,
) -> anyhow::Result<ContractAckMessage> {
    // verify finalized
    if input.event_type != NegotiationEventType::Finalized {
        bail!("Only FINALIZED event type is supported".to_string());
    }

    let cn_process = CONTRACT_CONSUMER_REPO
        .get_cn_process_by_consumer_id(consumer_pid.clone())
        .await
        .map_err(IdsaCNError::DbErr)?
        .ok_or(IdsaCNError::ProcessNotFound {
            provider_pid: Option::from(input.provider_pid),
            consumer_pid: Some(consumer_pid),
        })?;

    let mut cn_ack: ContractAckMessage = cn_process.into();
    cn_ack.state = ContractNegotiationState::Finalized;
    Ok(cn_ack)
}

pub async fn post_termination(
    consumer_pid: Urn,
    input: ContractTerminationMessage,
) -> anyhow::Result<ContractAckMessage> {
    let cn_process = CONTRACT_CONSUMER_REPO
        .get_cn_process_by_consumer_id(consumer_pid.clone())
        .await
        .map_err(IdsaCNError::DbErr)?
        .ok_or(IdsaCNError::ProcessNotFound {
            provider_pid: Option::from(input.provider_pid),
            consumer_pid: Some(consumer_pid),
        })?;

    let mut cn_ack: ContractAckMessage = cn_process.into();
    cn_ack.state = ContractNegotiationState::Terminated;
    Ok(cn_ack)
}
