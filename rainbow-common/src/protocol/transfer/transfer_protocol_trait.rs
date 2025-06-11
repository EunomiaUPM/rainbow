use crate::protocol::transfer::TransferMessageTypes;
use serde::{Deserialize, Serialize};
use urn::Urn;

pub trait DSProtocolTransferMessageTrait<'a>: Serialize + Deserialize<'a> + Clone {
    fn get_message_type(&self) -> anyhow::Result<TransferMessageTypes>;
    fn get_consumer_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(None)
    }
    fn get_provider_pid(&self) -> anyhow::Result<Option<&Urn>> {
        Ok(None)
    }
}
