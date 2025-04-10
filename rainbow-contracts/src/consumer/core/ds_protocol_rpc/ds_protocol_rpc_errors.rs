use crate::consumer::core::ds_protocol::ds_protocol_errors::IdsaCNError;
use thiserror::Error;
use urn::Urn;

#[derive(Debug, Error)]
pub enum DSRPCContractNegotiationConsumerErrors {
    #[error("Consumer not reachable")]
    ConsumerNotReachable {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
    },
    #[error("Consumer internal error")]
    ConsumerInternalError {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
    },
    #[error("Consumer response is not protocol compliant")]
    ConsumerResponseNotSerializable {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
    },
    #[error("Dataspace protocol error")]
    DSProtocolContractNegotiationError(IdsaCNError),
    #[error("Consumer and Provider not coincide")]
    ConsumerAndProviderCorrelationError {
        provider_pid: Urn,
        consumer_pid: Urn,
    },
    #[error("Consumer and Provider not coincide")]
    OdrlValidationError,
}