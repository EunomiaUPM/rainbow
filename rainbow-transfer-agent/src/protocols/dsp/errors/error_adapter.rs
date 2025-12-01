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
    TransferErrorDto, TransferProcessMessageType, TransferProcessMessageWrapper,
};
use rainbow_common::errors::CommonErrors;
use rainbow_common::protocol::context_field::ContextField;
use urn::Urn;

#[derive(Clone)]
pub struct DspTransferError {
    inner: CommonErrors,
    pub consumer_pid: Option<Urn>,
    pub provider_pid: Option<Urn>,
}

impl From<CommonErrors> for DspTransferError {
    fn from(value: CommonErrors) -> Self {
        Self { inner: value, consumer_pid: None, provider_pid: None }
    }
}

impl DspTransferError {
    pub fn set_error_consumer_pid(&mut self, consumer_pid: Option<Urn>) -> DspTransferError {
        self.consumer_pid = consumer_pid;
        self.clone()
    }
    pub fn set_error_provider_pid(&mut self, provider_pid: Option<Urn>) -> DspTransferError {
        self.provider_pid = provider_pid;
        self.clone()
    }
}

impl From<CommonErrors> for TransferProcessMessageWrapper<TransferErrorDto> {
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
        };
        TransferProcessMessageWrapper {
            context: ContextField::default(),
            _type: TransferProcessMessageType::TransferError,
            dto: TransferErrorDto {
                consumer_pid: None,
                provider_pid: None,
                code: Option::from(err_info.error_code.to_string()),
                reason: Option::from(vec![err_info.cause, err_info.message]),
            },
        }
    }
}

impl From<DspTransferError> for TransferProcessMessageWrapper<TransferErrorDto> {
    fn from(value: DspTransferError) -> Self {
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
        };
        TransferProcessMessageWrapper {
            context: ContextField::default(),
            _type: TransferProcessMessageType::TransferError,
            dto: TransferErrorDto {
                consumer_pid: value.consumer_pid,
                provider_pid: value.provider_pid,
                code: Option::from(err_info.error_code.to_string()),
                reason: Option::from(vec![err_info.cause, err_info.message]),
            },
        }
    }
}
