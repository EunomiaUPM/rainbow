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

use crate::common::schemas::{
    CONTRACT_AGREEMENT_MESSAGE_SCHEMA, CONTRACT_AGREEMENT_VERIFICATION_MESSAGE_SCHEMA,
    CONTRACT_NEGOTIATION_EVENT_MESSAGE_SCHEMA, CONTRACT_OFFER_MESSAGE_SCHEMA, CONTRACT_REQUEST_MESSAGE_SCHEMA,
    CONTRACT_TERMINATION_MESSAGE_SCHEMA,
};
use anyhow::bail;
use jsonschema::BasicOutput;
use log::error;
use rainbow_common::protocol::contract::contract_protocol_trait::DSProtocolContractNegotiationMessageTrait;
use rainbow_common::protocol::contract::ContractNegotiationMessages;
use serde_json::to_value;

pub fn validate_payload_schema<'a, M: DSProtocolContractNegotiationMessageTrait<'a>>(
    message: &M,
) -> anyhow::Result<()> {
    let validation = match message.get_message_type()? {
        ContractNegotiationMessages::ContractRequestMessage => {
            CONTRACT_REQUEST_MESSAGE_SCHEMA.apply(&to_value(message)?).basic()
        }
        ContractNegotiationMessages::ContractOfferMessage => {
            CONTRACT_OFFER_MESSAGE_SCHEMA.apply(&to_value(message)?).basic()
        }
        ContractNegotiationMessages::ContractAgreementMessage => {
            CONTRACT_AGREEMENT_MESSAGE_SCHEMA.apply(&to_value(message)?).basic()
        }
        ContractNegotiationMessages::ContractAgreementVerificationMessage => {
            CONTRACT_AGREEMENT_VERIFICATION_MESSAGE_SCHEMA.apply(&to_value(message)?).basic()
        }
        ContractNegotiationMessages::ContractNegotiationEventMessage => {
            CONTRACT_NEGOTIATION_EVENT_MESSAGE_SCHEMA.apply(&to_value(message)?).basic()
        }
        ContractNegotiationMessages::ContractNegotiationTerminationMessage => {
            CONTRACT_TERMINATION_MESSAGE_SCHEMA.apply(&to_value(message)?).basic()
        }
        _ => bail!("Message malformed"),
    };
    if let BasicOutput::Invalid(errors) = validation {
        for error in errors {
            error!("{}", error.instance_location());
            error!("{}", error.error_description());
        }
        bail!("Message malformed in JSON Data Validation");
    }
    Ok(())
}
