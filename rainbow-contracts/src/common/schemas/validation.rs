use crate::common::schemas::{CONTRACT_AGREEMENT_MESSAGE_SCHEMA, CONTRACT_AGREEMENT_VERIFICATION_MESSAGE_SCHEMA, CONTRACT_NEGOTIATION_EVENT_MESSAGE_SCHEMA, CONTRACT_OFFER_MESSAGE_SCHEMA, CONTRACT_REQUEST_MESSAGE_SCHEMA, CONTRACT_TERMINATION_MESSAGE_SCHEMA};
use anyhow::bail;
use jsonschema::BasicOutput;
use log::error;
use rainbow_common::protocol::contract::contract_protocol_trait::DSProtocolContractNegotiationMessageTrait;
use rainbow_common::protocol::contract::ContractNegotiationMessages;
use serde_json::to_value;
use tracing::debug;

pub fn validate_payload_schema<'a, M: DSProtocolContractNegotiationMessageTrait<'a>>(message: &M) -> anyhow::Result<()> {
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