/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
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

use crate::entities::negotiation_process::NegotiationProcessDto;
use crate::protocols::dsp::protocol_types::{
    NegotiationAckMessageDto, NegotiationAgreementMessageDto, NegotiationErrorMessageDto, NegotiationEventMessageDto,
    NegotiationEventType, NegotiationOfferMessageDto, NegotiationProcessMessageType, NegotiationProcessMessageWrapper,
    NegotiationRequestMessageDto, NegotiationTerminationMessageDto, NegotiationVerificationMessageDto,
};
use rainbow_common::protocol::context_field::ContextField;
use rainbow_common::utils::get_urn;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::str::FromStr;
use urn::Urn;

#[allow(unused)]
pub trait RpcNegotiationProcessMessageTrait: Debug + Send + Sync {
    fn get_consumer_pid(&self) -> Option<Urn>;
    fn get_provider_pid(&self) -> Option<Urn>;
    fn get_associated_agent_peer(&self) -> Option<String>;
    fn get_agreement_id(&self) -> Option<Urn>;
    fn get_format(&self) -> Option<String>;
    fn get_provider_address(&self) -> Option<String>;
    fn get_callback_address(&self) -> Option<String>;
    fn get_error_code(&self) -> Option<String>;
    fn get_error_reason(&self) -> Option<Vec<String>>;
    fn get_message(&self) -> NegotiationProcessMessageType;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RpcNegotiationRequestMessageDto {}

impl Into<NegotiationProcessMessageWrapper<NegotiationRequestMessageDto>> for RpcNegotiationRequestMessageDto {
    fn into(self) -> NegotiationProcessMessageWrapper<NegotiationRequestMessageDto> {
        let consumer_pid = format!("urn:consumer-pid:{}", uuid::Uuid::new_v4());
        let consumer_pid_urn = Urn::from_str(consumer_pid.as_str()).unwrap();
        NegotiationProcessMessageWrapper {
            context: ContextField::default(),
            _type: NegotiationProcessMessageType::NegotiationRequestMessage,
            dto: NegotiationRequestMessageDto {
                callback_address: None,
                consumer_pid: consumer_pid_urn,
                provider_pid: None,
                offer: Default::default(),
            },
        }
    }
}

impl RpcNegotiationProcessMessageTrait for RpcNegotiationRequestMessageDto {
    fn get_consumer_pid(&self) -> Option<Urn> {
        None
    }

    fn get_provider_pid(&self) -> Option<Urn> {
        None
    }

    fn get_associated_agent_peer(&self) -> Option<String> {
        None
    }

    fn get_agreement_id(&self) -> Option<Urn> {
        None
    }

    fn get_format(&self) -> Option<String> {
        None
    }

    fn get_provider_address(&self) -> Option<String> {
        None
    }

    fn get_callback_address(&self) -> Option<String> {
        None
    }

    fn get_error_code(&self) -> Option<String> {
        None
    }

    fn get_error_reason(&self) -> Option<Vec<String>> {
        None
    }

    fn get_message(&self) -> NegotiationProcessMessageType {
        NegotiationProcessMessageType::NegotiationRequestMessage
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RpcNegotiationOfferMessageDto {}

impl Into<NegotiationProcessMessageWrapper<NegotiationOfferMessageDto>> for RpcNegotiationOfferMessageDto {
    fn into(self) -> NegotiationProcessMessageWrapper<NegotiationOfferMessageDto> {
        NegotiationProcessMessageWrapper {
            context: ContextField::default(),
            _type: NegotiationProcessMessageType::NegotiationOfferMessage,
            dto: NegotiationOfferMessageDto {
                provider_pid: get_urn(None),
                offer: Default::default(),
                consumer_pid: None,
                callback_address: None,
            },
        }
    }
}

impl RpcNegotiationProcessMessageTrait for RpcNegotiationOfferMessageDto {
    fn get_consumer_pid(&self) -> Option<Urn> {
        None
    }

    fn get_provider_pid(&self) -> Option<Urn> {
        None
    }

    fn get_associated_agent_peer(&self) -> Option<String> {
        None
    }

    fn get_agreement_id(&self) -> Option<Urn> {
        None
    }

    fn get_format(&self) -> Option<String> {
        None
    }

    fn get_provider_address(&self) -> Option<String> {
        None
    }

    fn get_callback_address(&self) -> Option<String> {
        None
    }

    fn get_error_code(&self) -> Option<String> {
        None
    }

    fn get_error_reason(&self) -> Option<Vec<String>> {
        None
    }

    fn get_message(&self) -> NegotiationProcessMessageType {
        NegotiationProcessMessageType::NegotiationOfferMessage
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RpcNegotiationAgreementMessageDto {}

impl Into<NegotiationProcessMessageWrapper<NegotiationAgreementMessageDto>> for RpcNegotiationAgreementMessageDto {
    fn into(self) -> NegotiationProcessMessageWrapper<NegotiationAgreementMessageDto> {
        NegotiationProcessMessageWrapper {
            context: ContextField::default(),
            _type: NegotiationProcessMessageType::NegotiationAgreementMessage,
            dto: NegotiationAgreementMessageDto {
                provider_pid: get_urn(None),
                consumer_pid: get_urn(None),
                agreement: Default::default(),
            },
        }
    }
}

impl RpcNegotiationProcessMessageTrait for RpcNegotiationAgreementMessageDto {
    fn get_consumer_pid(&self) -> Option<Urn> {
        None
    }

    fn get_provider_pid(&self) -> Option<Urn> {
        None
    }

    fn get_associated_agent_peer(&self) -> Option<String> {
        None
    }

    fn get_agreement_id(&self) -> Option<Urn> {
        None
    }

    fn get_format(&self) -> Option<String> {
        None
    }

    fn get_provider_address(&self) -> Option<String> {
        None
    }

    fn get_callback_address(&self) -> Option<String> {
        None
    }

    fn get_error_code(&self) -> Option<String> {
        None
    }

    fn get_error_reason(&self) -> Option<Vec<String>> {
        None
    }

    fn get_message(&self) -> NegotiationProcessMessageType {
        NegotiationProcessMessageType::NegotiationAgreementMessage
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RpcNegotiationVerificationMessageDto {}

impl Into<NegotiationProcessMessageWrapper<NegotiationVerificationMessageDto>>
    for RpcNegotiationVerificationMessageDto
{
    fn into(self) -> NegotiationProcessMessageWrapper<NegotiationVerificationMessageDto> {
        NegotiationProcessMessageWrapper {
            context: ContextField::default(),
            _type: NegotiationProcessMessageType::NegotiationAgreementVerificationMessage,
            dto: NegotiationVerificationMessageDto { consumer_pid: get_urn(None), provider_pid: get_urn(None) },
        }
    }
}

impl RpcNegotiationProcessMessageTrait for RpcNegotiationVerificationMessageDto {
    fn get_consumer_pid(&self) -> Option<Urn> {
        None
    }

    fn get_provider_pid(&self) -> Option<Urn> {
        None
    }

    fn get_associated_agent_peer(&self) -> Option<String> {
        None
    }

    fn get_agreement_id(&self) -> Option<Urn> {
        None
    }

    fn get_format(&self) -> Option<String> {
        None
    }

    fn get_provider_address(&self) -> Option<String> {
        None
    }

    fn get_callback_address(&self) -> Option<String> {
        None
    }

    fn get_error_code(&self) -> Option<String> {
        None
    }

    fn get_error_reason(&self) -> Option<Vec<String>> {
        None
    }

    fn get_message(&self) -> NegotiationProcessMessageType {
        NegotiationProcessMessageType::NegotiationAgreementVerificationMessage
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RpcNegotiationEventMessageDto {}

impl Into<NegotiationProcessMessageWrapper<NegotiationEventMessageDto>> for RpcNegotiationEventMessageDto {
    fn into(self) -> NegotiationProcessMessageWrapper<NegotiationEventMessageDto> {
        NegotiationProcessMessageWrapper {
            context: ContextField::default(),
            _type: NegotiationProcessMessageType::NegotiationEventMessage(NegotiationEventType::ACCEPTED),
            dto: NegotiationEventMessageDto {
                consumer_pid: get_urn(None),
                provider_pid: get_urn(None),
                event_type: NegotiationEventType::ACCEPTED,
            },
        }
    }
}

impl RpcNegotiationProcessMessageTrait for RpcNegotiationEventMessageDto {
    fn get_consumer_pid(&self) -> Option<Urn> {
        None
    }

    fn get_provider_pid(&self) -> Option<Urn> {
        None
    }

    fn get_associated_agent_peer(&self) -> Option<String> {
        None
    }

    fn get_agreement_id(&self) -> Option<Urn> {
        None
    }

    fn get_format(&self) -> Option<String> {
        None
    }

    fn get_provider_address(&self) -> Option<String> {
        None
    }

    fn get_callback_address(&self) -> Option<String> {
        None
    }

    fn get_error_code(&self) -> Option<String> {
        None
    }

    fn get_error_reason(&self) -> Option<Vec<String>> {
        None
    }

    fn get_message(&self) -> NegotiationProcessMessageType {
        NegotiationProcessMessageType::NegotiationEventMessage(NegotiationEventType::ACCEPTED)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RpcNegotiationTerminationMessageDto {
    pub consumer_pid: Urn,
    pub provider_pid: Urn,
    pub code: Option<String>,
    pub reason: Option<Vec<String>>,
}

impl Into<NegotiationProcessMessageWrapper<NegotiationTerminationMessageDto>> for RpcNegotiationTerminationMessageDto {
    fn into(self) -> NegotiationProcessMessageWrapper<NegotiationTerminationMessageDto> {
        NegotiationProcessMessageWrapper {
            context: ContextField::default(),
            _type: NegotiationProcessMessageType::NegotiationTerminationMessage,
            dto: NegotiationTerminationMessageDto {
                consumer_pid: get_urn(None),
                provider_pid: get_urn(None),
                code: None,
                reason: None,
            },
        }
    }
}

impl RpcNegotiationProcessMessageTrait for RpcNegotiationTerminationMessageDto {
    fn get_consumer_pid(&self) -> Option<Urn> {
        None
    }

    fn get_provider_pid(&self) -> Option<Urn> {
        None
    }

    fn get_associated_agent_peer(&self) -> Option<String> {
        None
    }

    fn get_agreement_id(&self) -> Option<Urn> {
        None
    }

    fn get_format(&self) -> Option<String> {
        None
    }

    fn get_provider_address(&self) -> Option<String> {
        None
    }

    fn get_callback_address(&self) -> Option<String> {
        None
    }

    fn get_error_code(&self) -> Option<String> {
        None
    }

    fn get_error_reason(&self) -> Option<Vec<String>> {
        None
    }

    fn get_message(&self) -> NegotiationProcessMessageType {
        NegotiationProcessMessageType::NegotiationTerminationMessage
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RpcTransferMessageDto<T> {
    pub request: T,
    pub response: NegotiationProcessMessageWrapper<NegotiationAckMessageDto>,
    pub transfer_agent_model: NegotiationProcessDto,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RpcTransferErrorDto<T> {
    pub request: T,
    pub error: NegotiationProcessMessageWrapper<NegotiationErrorMessageDto>,
}
