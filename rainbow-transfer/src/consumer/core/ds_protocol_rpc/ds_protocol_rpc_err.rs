use crate::consumer::core::ds_protocol::ds_protocol_err::DSProtocolTransferConsumerErrors;
use thiserror::Error;
use urn::Urn;

#[derive(Debug, Error)]
pub enum DSRPCTransferConsumerErrors {
    #[error("Provider not reachable")]
    ProviderNotReachable {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
    },
    #[error("Provider internal error")]
    ProviderInternalError {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
        error: Option<serde_json::Value>,
    },
    #[error("Provider response is not protocol compliant")]
    ProviderResponseNotSerializable {
        provider_pid: Option<Urn>,
        consumer_pid: Option<Urn>,
    },
    #[error("Dataspace protocol error")]
    DSProtocolTransferConsumerError(DSProtocolTransferConsumerErrors),
}