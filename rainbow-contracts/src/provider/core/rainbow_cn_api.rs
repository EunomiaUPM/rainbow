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

use crate::provider::core::rainbow_cn_errors::CnErrorProvider;
use crate::provider::core::rainbow_cn_types::{
    EditAgreementRequest, EditContractNegotiationMessageRequest,
    EditContractNegotiationOfferRequest, EditContractNegotiationRequest, EditParticipantRequest,
    NewAgreementRequest, NewContractNegotiationMessageRequest, NewContractNegotiationOfferRequest,
    NewContractNegotiationRequest, NewParticipantRequest,
};
use rainbow_db::contracts_provider::entities::{
    agreement, cn_message, cn_offer, cn_process, participant,
};
use rainbow_db::contracts_provider::repo::{CnErrors, CONTRACT_PROVIDER_REPO};
use std::fmt::{Display, Formatter};
use urn::Urn;

pub enum CNControllerTypes {
    Process,
    Message,
    Offer,
    Agreement,
    Participant,
}
impl Display for CNControllerTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CNControllerTypes::Process => write!(f, "Contract Negotiation Process"),
            CNControllerTypes::Message => write!(f, "Contract Negotiation Message"),
            CNControllerTypes::Offer => write!(f, "Contract Negotiation Offer"),
            CNControllerTypes::Agreement => write!(f, "Contract Negotiation Agreement"),
            CNControllerTypes::Participant => write!(f, "Contract Negotiation Participant"),
        }
    }
}

///
/// CNP Processes Controllers
///
pub async fn get_cn_processes() -> anyhow::Result<Vec<cn_process::Model>> {
    let processes = CONTRACT_PROVIDER_REPO
        .get_all_cn_processes(None, None)
        .await
        .map_err(CnErrorProvider::DbErr)?;
    Ok(processes)
}

pub async fn get_cn_process_by_id(process_id: Urn) -> anyhow::Result<cn_process::Model> {
    let process = CONTRACT_PROVIDER_REPO
        .get_cn_process_by_cn_id(process_id.clone())
        .await
        .map_err(CnErrorProvider::DbErr)?
        .ok_or(CnErrorProvider::NotFound {
            id: process_id,
            entity: CNControllerTypes::Process.to_string(),
        })?;
    Ok(process)
}

pub async fn get_cn_process_by_provider(provider_id: Urn) -> anyhow::Result<cn_process::Model> {
    let process = CONTRACT_PROVIDER_REPO
        .get_cn_processes_by_provider_id(&provider_id)
        .await
        .map_err(CnErrorProvider::DbErr)?
        .ok_or(CnErrorProvider::ProviderNotFound {
            provider_id,
            entity: CNControllerTypes::Process.to_string(),
        })?;
    Ok(process)
}

pub async fn get_cn_process_by_consumer(consumer_id: Urn) -> anyhow::Result<cn_process::Model> {
    let process = CONTRACT_PROVIDER_REPO
        .get_cn_processes_by_consumer_id(consumer_id.clone())
        .await
        .map_err(CnErrorProvider::DbErr)?
        .ok_or(CnErrorProvider::ConsumerNotFound {
            consumer_id,
            entity: CNControllerTypes::Process.to_string(),
        })?;
    Ok(process)
}

pub async fn post_cn_process(
    input: NewContractNegotiationRequest,
) -> anyhow::Result<cn_process::Model> {
    let process = CONTRACT_PROVIDER_REPO
        .create_cn_process(input.into())
        .await
        .map_err(CnErrorProvider::DbErr)?;
    Ok(process)
}

pub async fn put_cn_process(
    process_id: Urn,
    input: EditContractNegotiationRequest,
) -> anyhow::Result<cn_process::Model> {
    let process = CONTRACT_PROVIDER_REPO
        .put_cn_process(process_id.clone(), input.into())
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::NotFound {
                id: process_id,
                entity: CNControllerTypes::Process.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(process)
}

pub async fn delete_cn_process_by_id(process_id: Urn) -> anyhow::Result<()> {
    let _ =
        CONTRACT_PROVIDER_REPO.delete_cn_process(process_id.clone()).await.map_err(
            |err| match err {
                CnErrors::CNProcessNotFound => CnErrorProvider::NotFound {
                    id: process_id,
                    entity: CNControllerTypes::Process.to_string(),
                },
                _ => CnErrorProvider::DbErr(err),
            },
        )?;
    Ok(())
}

///
/// CNMessage
///
pub async fn get_cn_messages() -> anyhow::Result<Vec<cn_message::Model>> {
    let cn_messages = CONTRACT_PROVIDER_REPO
        .get_all_cn_messages(None, None)
        .await
        .map_err(CnErrorProvider::DbErr)?;
    Ok(cn_messages)
}

pub async fn get_cn_messages_by_cn_process(
    process_id: Urn,
) -> anyhow::Result<Vec<cn_message::Model>> {
    let cn_messages = CONTRACT_PROVIDER_REPO
        .get_cn_messages_by_cn_process_id(process_id.clone())
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::NotFound {
                id: process_id,
                entity: CNControllerTypes::Process.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(cn_messages)
}

pub async fn get_cn_messages_by_cn_message_id(
    message_id: Urn,
) -> anyhow::Result<cn_message::Model> {
    let cn_message = CONTRACT_PROVIDER_REPO
        .get_cn_messages_by_cn_message_id(message_id.clone())
        .await
        .map_err(CnErrorProvider::DbErr)?
        .ok_or(CnErrorProvider::NotFound {
            id: message_id,
            entity: CNControllerTypes::Message.to_string(),
        })?;
    Ok(cn_message)
}

pub async fn get_cn_messages_by_cn_provider_id(
    provider_id: Urn,
) -> anyhow::Result<Vec<cn_message::Model>> {
    let cn_messages = CONTRACT_PROVIDER_REPO
        .get_cn_messages_by_provider_id(provider_id.clone())
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::ProviderNotFound {
                provider_id,
                entity: CNControllerTypes::Message.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(cn_messages)
}

pub async fn get_cn_messages_by_cn_consumer_id(
    consumer_id: Urn,
) -> anyhow::Result<Vec<cn_message::Model>> {
    let cn_messages = CONTRACT_PROVIDER_REPO
        .get_cn_messages_by_consumer_id(consumer_id.clone())
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::ConsumerNotFound {
                consumer_id,
                entity: CNControllerTypes::Message.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(cn_messages)
}

pub async fn post_cn_message_by_cn_process(
    process_id: Urn,
    input: NewContractNegotiationMessageRequest,
) -> anyhow::Result<cn_message::Model> {
    let cn_message = CONTRACT_PROVIDER_REPO
        .create_cn_message(process_id.clone(), input.into())
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::ProcessNotFound {
                process_id,
                entity: CNControllerTypes::Message.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(cn_message)
}

pub async fn put_cn_message_by_cn_process(
    process_id: Urn,
    message_id: Urn,
    input: EditContractNegotiationMessageRequest,
) -> anyhow::Result<cn_message::Model> {
    let cn_message = CONTRACT_PROVIDER_REPO
        .put_cn_message(process_id.clone(), message_id.clone(), input.into())
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::NotFound {
                id: process_id,
                entity: CNControllerTypes::Process.to_string(),
            },
            CnErrors::CNMessageNotFound => CnErrorProvider::NotFound {
                id: message_id,
                entity: CNControllerTypes::Message.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(cn_message)
}

pub async fn delete_cn_message_by_cn_process(
    process_id: Urn,
    message_id: Urn,
) -> anyhow::Result<()> {
    let _ = CONTRACT_PROVIDER_REPO
        .delete_cn_message(process_id.clone(), message_id.clone())
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::NotFound {
                id: process_id,
                entity: CNControllerTypes::Process.to_string(),
            },
            CnErrors::CNMessageNotFound => CnErrorProvider::NotFound {
                id: message_id,
                entity: CNControllerTypes::Message.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(())
}

///
/// CNOffers Rainbow API
///
pub async fn get_cn_offers_by_cn_process_id(
    process_id: Urn,
) -> anyhow::Result<Vec<cn_offer::Model>> {
    let offers = CONTRACT_PROVIDER_REPO
        .get_all_cn_offers_by_cn_process(process_id.clone())
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::ProcessNotFound {
                process_id,
                entity: CNControllerTypes::Offer.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(offers)
}

pub async fn get_last_cn_offers_by_cn_process_id(
    process_id: Urn,
) -> anyhow::Result<cn_offer::Model> {
    let offer = CONTRACT_PROVIDER_REPO
        .get_last_cn_offers_by_cn_process(process_id.clone())
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::ProcessNotFound {
                process_id: process_id.clone(),
                entity: CNControllerTypes::Offer.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?
        .ok_or(CnErrorProvider::LastNotFound {
            id: process_id,
            entity: CNControllerTypes::Offer.to_string(),
        })?;
    Ok(offer)
}

pub async fn get_cn_offer_by_cn_message_id(message_id: Urn) -> anyhow::Result<cn_offer::Model> {
    let offer = CONTRACT_PROVIDER_REPO
        .get_all_cn_offers_by_message_id(message_id.clone())
        .await
        .map_err(|err| match err {
            CnErrors::CNMessageNotFound => CnErrorProvider::NotFound {
                id: message_id.clone(),
                entity: CNControllerTypes::Offer.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?
        .ok_or(CnErrorProvider::NotFound {
            id: message_id.clone(),
            entity: CNControllerTypes::Message.to_string(),
        })?;
    Ok(offer)
}

pub async fn get_cn_offer_by_offer_id(offer_id: Urn) -> anyhow::Result<cn_offer::Model> {
    let offer = CONTRACT_PROVIDER_REPO
        .get_cn_offer_by_id(offer_id.clone())
        .await
        .map_err(CnErrorProvider::DbErr)?
        .ok_or(CnErrorProvider::NotFound {
            id: offer_id,
            entity: CNControllerTypes::Offer.to_string(),
        })?;
    Ok(offer)
}

pub async fn post_cn_offer_by_cn_process_id_and_message_id(
    process_id: Urn,
    message_id: Urn,
    input: NewContractNegotiationOfferRequest,
) -> anyhow::Result<cn_offer::Model> {
    let offer = CONTRACT_PROVIDER_REPO
        .create_cn_offer(process_id.clone(), message_id.clone(), input.into())
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::ProcessNotFound {
                process_id,
                entity: CNControllerTypes::Process.to_string(),
            },
            CnErrors::CNMessageNotFound => CnErrorProvider::ProcessNotFound {
                process_id,
                entity: CNControllerTypes::Message.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(offer)
}

pub async fn put_cn_offer_by_cn_process_id_and_message_id(
    process_id: Urn,
    message_id: Urn,
    offer_id: Urn,
    input: EditContractNegotiationOfferRequest,
) -> anyhow::Result<cn_offer::Model> {
    let offer = CONTRACT_PROVIDER_REPO
        .put_cn_offer(
            process_id.clone(),
            message_id.clone(),
            offer_id,
            input.into(),
        )
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::NotFound {
                id: process_id,
                entity: CNControllerTypes::Process.to_string(),
            },
            CnErrors::CNMessageNotFound => CnErrorProvider::NotFound {
                id: message_id,
                entity: CNControllerTypes::Message.to_string(),
            },
            CnErrors::CNOfferNotFound => CnErrorProvider::NotFound {
                id: message_id,
                entity: CNControllerTypes::Offer.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(offer)
}

pub async fn delete_cn_offer_by_cn_process_id_and_message_id(
    process_id: Urn,
    message_id: Urn,
    offer_id: Urn,
) -> anyhow::Result<()> {
    let _ = CONTRACT_PROVIDER_REPO
        .delete_cn_offer(process_id.clone(), message_id.clone(), offer_id)
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::NotFound {
                id: process_id,
                entity: CNControllerTypes::Process.to_string(),
            },
            CnErrors::CNMessageNotFound => CnErrorProvider::NotFound {
                id: message_id,
                entity: CNControllerTypes::Message.to_string(),
            },
            CnErrors::CNOfferNotFound => CnErrorProvider::NotFound {
                id: message_id,
                entity: CNControllerTypes::Offer.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(())
}

///
/// Agreements
///

pub async fn get_agreement_by_cn_process_id(process_id: Urn) -> anyhow::Result<agreement::Model> {
    let agreement = CONTRACT_PROVIDER_REPO
        .get_agreement_by_process_id(process_id.clone())
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::NotFound {
                id: process_id.clone(),
                entity: CNControllerTypes::Process.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?
        .ok_or(CnErrorProvider::ProcessNotFound {
            process_id,
            entity: CNControllerTypes::Agreement.to_string(),
        })?;
    Ok(agreement)
}

pub async fn get_agreement_by_cn_message_id(message_id: Urn) -> anyhow::Result<agreement::Model> {
    let agreement = CONTRACT_PROVIDER_REPO
        .get_agreement_by_message_id(message_id.clone())
        .await
        .map_err(|err| match err {
            CnErrors::CNMessageNotFound => CnErrorProvider::NotFound {
                id: message_id.clone(),
                entity: CNControllerTypes::Message.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?
        .ok_or(CnErrorProvider::MessageNotFound {
            message_id,
            entity: CNControllerTypes::Agreement.to_string(),
        })?;
    Ok(agreement)
}

pub async fn get_agreements() -> anyhow::Result<Vec<agreement::Model>> {
    let agreements = CONTRACT_PROVIDER_REPO
        .get_all_agreements(None, None)
        .await
        .map_err(CnErrorProvider::DbErr)?;
    Ok(agreements)
}

pub async fn get_agreement_by_agreement_id(agreement_id: Urn) -> anyhow::Result<agreement::Model> {
    let agreement = CONTRACT_PROVIDER_REPO
        .get_agreement_by_ag_id(agreement_id.clone())
        .await
        .map_err(CnErrorProvider::DbErr)?
        .ok_or(CnErrorProvider::NotFound {
            id: agreement_id,
            entity: CNControllerTypes::Agreement.to_string(),
        })?;
    Ok(agreement)
}

pub async fn post_agreement(
    process_id: Urn,
    message_id: Urn,
    input: NewAgreementRequest,
) -> anyhow::Result<agreement::Model> {
    let agreement = CONTRACT_PROVIDER_REPO
        .create_agreement(process_id.clone(), message_id.clone(), input.into())
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::NotFound {
                id: process_id,
                entity: CNControllerTypes::Process.to_string(),
            },
            CnErrors::CNMessageNotFound => CnErrorProvider::NotFound {
                id: message_id,
                entity: CNControllerTypes::Message.to_string(),
            },
            CnErrors::ParticipantNotFound(which, urn) => CnErrorProvider::ParticipantNotFound {
                participant_id: urn,
                which,
                entity: CNControllerTypes::Agreement.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(agreement)
}

pub async fn put_agreement(
    process_id: Urn,
    message_id: Urn,
    agreement_id: Urn,
    input: EditAgreementRequest,
) -> anyhow::Result<agreement::Model> {
    let agreement = CONTRACT_PROVIDER_REPO
        .put_agreement(
            process_id.clone(),
            message_id.clone(),
            agreement_id.clone(),
            input.into(),
        )
        .await
        .map_err(|err| match err {
            CnErrors::CNProcessNotFound => CnErrorProvider::ProcessNotFound {
                process_id,
                entity: CNControllerTypes::Agreement.to_string(),
            },
            CnErrors::CNMessageNotFound => CnErrorProvider::MessageNotFound {
                message_id,
                entity: CNControllerTypes::Agreement.to_string(),
            },
            CnErrors::AgreementNotFound => CnErrorProvider::NotFound {
                id: agreement_id,
                entity: CNControllerTypes::Agreement.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(agreement)
}

pub async fn delete_agreement(
    process_id: Urn,
    message_id: Urn,
    agreement_id: Urn,
) -> anyhow::Result<()> {
    let _ =
        CONTRACT_PROVIDER_REPO.delete_agreement(process_id.clone(), message_id.clone(), agreement_id.clone()).await.map_err(
            |err| match err {
                CnErrors::CNProcessNotFound => CnErrorProvider::ProcessNotFound {
                    process_id,
                    entity: CNControllerTypes::Agreement.to_string(),
                },
                CnErrors::CNMessageNotFound => CnErrorProvider::MessageNotFound {
                    message_id,
                    entity: CNControllerTypes::Agreement.to_string(),
                },
                CnErrors::AgreementNotFound => CnErrorProvider::NotFound {
                    id: agreement_id,
                    entity: CNControllerTypes::Agreement.to_string(),
                },
                _ => CnErrorProvider::DbErr(err),
            },
        )?;
    Ok(())
}

///
/// Participants
///
pub async fn get_participants() -> anyhow::Result<Vec<participant::Model>> {
    let participants = CONTRACT_PROVIDER_REPO
        .get_all_participants(None, None)
        .await
        .map_err(CnErrorProvider::DbErr)?;
    Ok(participants)
}

pub async fn get_participant_by_id(participant_id: Urn) -> anyhow::Result<participant::Model> {
    let participant = CONTRACT_PROVIDER_REPO
        .get_participant_by_p_id(participant_id.clone())
        .await
        .map_err(CnErrorProvider::DbErr)?
        .ok_or(CnErrorProvider::NotFound {
            id: participant_id,
            entity: CNControllerTypes::Participant.to_string(),
        })?;
    Ok(participant)
}

pub async fn get_participant_agreements(
    participant_id: Urn,
) -> anyhow::Result<Vec<agreement::Model>> {
    let agreements = CONTRACT_PROVIDER_REPO
        .get_agreements_by_participant_id(participant_id.clone())
        .await
        .map_err(|err| match err {
            CnErrors::ParticipantNotFound(_, _) => CnErrorProvider::NotFound {
                id: participant_id,
                entity: CNControllerTypes::Participant.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(agreements)
}

pub async fn post_participant(input: NewParticipantRequest) -> anyhow::Result<participant::Model> {
    let participant = CONTRACT_PROVIDER_REPO
        .create_participant(input.into())
        .await
        .map_err(CnErrorProvider::DbErr)?;
    Ok(participant)
}

pub async fn put_participant(
    participant_id: Urn,
    input: EditParticipantRequest,
) -> anyhow::Result<participant::Model> {
    let participant = CONTRACT_PROVIDER_REPO
        .put_participant(participant_id.clone(), input.into())
        .await
        .map_err(|err| match err {
            CnErrors::ParticipantNotFound(_, _) => CnErrorProvider::NotFound {
                id: participant_id,
                entity: CNControllerTypes::Participant.to_string(),
            },
            _ => CnErrorProvider::DbErr(err),
        })?;
    Ok(participant)
}

pub async fn delete_participant(participant_id: Urn) -> anyhow::Result<()> {
    let _ =
        CONTRACT_PROVIDER_REPO.delete_participant(participant_id.clone()).await.map_err(|err| {
            match err {
                CnErrors::ParticipantNotFound(_, _) => CnErrorProvider::NotFound {
                    id: participant_id,
                    entity: CNControllerTypes::Participant.to_string(),
                },
                _ => CnErrorProvider::DbErr(err),
            }
        })?;
    Ok(())
}
