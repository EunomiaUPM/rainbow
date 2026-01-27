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

use crate::protocols::dsp::protocol_types::{
    NegotiationErrorMessageDto, NegotiationProcessMessageType, NegotiationProcessMessageWrapper,
};
use rainbow_common::dsp_common::context_field::ContextField;
use rainbow_common::errors::{CommonErrors, ErrorInfo};
use urn::Urn;

#[derive(Clone)]
pub struct DspNegotiationError {
    inner: CommonErrors,
    pub consumer_pid: Option<Urn>,
    pub provider_pid: Option<Urn>,
}

impl From<CommonErrors> for DspNegotiationError {
    fn from(value: CommonErrors) -> Self {
        Self { inner: value, consumer_pid: None, provider_pid: None }
    }
}

impl DspNegotiationError {
    pub fn set_error_consumer_pid(&mut self, consumer_pid: Option<Urn>) -> DspNegotiationError {
        self.consumer_pid = consumer_pid;
        self.clone()
    }
    pub fn set_error_provider_pid(&mut self, provider_pid: Option<Urn>) -> DspNegotiationError {
        self.provider_pid = provider_pid;
        self.clone()
    }
}

impl From<CommonErrors> for NegotiationProcessMessageWrapper<NegotiationErrorMessageDto> {
    fn from(value: CommonErrors) -> Self {
        let err_info = match value {
            CommonErrors::PetitionError { info, .. } => info,
            CommonErrors::ProviderError { info, .. } => info,
            CommonErrors::ConsumerError { info, .. } => info,
            CommonErrors::AuthorityError { info, .. } => info,
            CommonErrors::MissingActionError { info, .. } => info,
            CommonErrors::MissingResourceError { info, .. } => info,
            CommonErrors::FormatError { info, .. } => info,
            CommonErrors::UnauthorizedError { info, .. } => info,
            CommonErrors::ForbiddenError { info, .. } => info,
            CommonErrors::DatabaseError { info, .. } => info,
            CommonErrors::FeatureNotImplError { info, .. } => info,
            CommonErrors::ReadError { info, .. } => info,
            CommonErrors::WriteError { info, .. } => info,
            CommonErrors::ParseError { info, .. } => info,
            CommonErrors::ModuleNotActiveError { info, .. } => info,
            CommonErrors::VaultError { info, .. } => info,
            CommonErrors::EnvVarError { info, .. } => info,
        };
        NegotiationProcessMessageWrapper {
            context: ContextField::default(),
            _type: NegotiationProcessMessageType::NegotiationError,
            dto: NegotiationErrorMessageDto {
                consumer_pid: None,
                provider_pid: None,
                code: Option::from(err_info.error_code.to_string()),
                reason: Option::from(vec![err_info.cause, err_info.message]),
            },
        }
    }
}

impl From<DspNegotiationError> for NegotiationProcessMessageWrapper<NegotiationErrorMessageDto> {
    fn from(value: DspNegotiationError) -> Self {
        let common_error = value.inner;
        let err_info = match common_error {
            CommonErrors::PetitionError { info, .. } => info,
            CommonErrors::ProviderError { info, .. } => info,
            CommonErrors::ConsumerError { info, .. } => info,
            CommonErrors::AuthorityError { info, .. } => info,
            CommonErrors::MissingActionError { info, .. } => info,
            CommonErrors::MissingResourceError { info, .. } => info,
            CommonErrors::FormatError { info, .. } => info,
            CommonErrors::UnauthorizedError { info, .. } => info,
            CommonErrors::ForbiddenError { info, .. } => info,
            CommonErrors::DatabaseError { info, .. } => info,
            CommonErrors::FeatureNotImplError { info, .. } => info,
            CommonErrors::ReadError { info, .. } => info,
            CommonErrors::WriteError { info, .. } => info,
            CommonErrors::ParseError { info, .. } => info,
            CommonErrors::ModuleNotActiveError { info, .. } => info,
            CommonErrors::VaultError { info, .. } => info,
            CommonErrors::EnvVarError { info, .. } => info,
        };
        NegotiationProcessMessageWrapper {
            context: ContextField::default(),
            _type: NegotiationProcessMessageType::NegotiationError,
            dto: NegotiationErrorMessageDto {
                consumer_pid: value.consumer_pid,
                provider_pid: value.provider_pid,
                code: Option::from(err_info.error_code.to_string()),
                reason: Option::from(vec![err_info.cause, err_info.message]),
            },
        }
    }
}
