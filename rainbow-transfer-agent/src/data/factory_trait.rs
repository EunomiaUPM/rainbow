use crate::data::repo_traits::transfer_message_repo::TransferMessageRepoTrait;
use crate::data::repo_traits::transfer_process_identifier_repo::TransferIdentifierRepoTrait;
use crate::data::repo_traits::transfer_process_repo::TransferProcessRepoTrait;
use std::sync::Arc;

#[mockall::automock]
pub trait TransferAgentRepoTrait: Send + Sync + 'static {
    fn get_transfer_process_repo(&self) -> Arc<dyn TransferProcessRepoTrait>;
    fn get_transfer_message_repo(&self) -> Arc<dyn TransferMessageRepoTrait>;
    fn get_transfer_process_identifiers_repo(&self) -> Arc<dyn TransferIdentifierRepoTrait>;
}
