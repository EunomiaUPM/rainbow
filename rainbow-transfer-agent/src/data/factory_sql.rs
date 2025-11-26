use crate::data::factory_trait::TransferAgentRepoTrait;
use crate::data::repo_traits::transfer_message_repo::TransferMessageRepoTrait;
use crate::data::repo_traits::transfer_process_identifier_repo::TransferIdentifierRepoTrait;
use crate::data::repo_traits::transfer_process_repo::TransferProcessRepoTrait;
use crate::data::repos_sql::transfer_message_repo::TransferMessageRepoForSql;
use crate::data::repos_sql::transfer_process_identifier_repo::TransferIdentifierRepoForSql;
use crate::data::repos_sql::transfer_process_repo::TransferProcessRepoForSql;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct TransferAgentRepoForSql {
    transfer_process_repo: Arc<dyn TransferProcessRepoTrait>,
    transfer_process_identifier_repo: Arc<dyn TransferIdentifierRepoTrait>,
    transfer_message_repo: Arc<dyn TransferMessageRepoTrait>,
}

impl TransferAgentRepoForSql {
    pub fn create_repo(db_connection: DatabaseConnection) -> Self {
        Self {
            transfer_process_repo: Arc::new(TransferProcessRepoForSql::new(db_connection.clone())),
            transfer_process_identifier_repo: Arc::new(TransferIdentifierRepoForSql::new(db_connection.clone())),
            transfer_message_repo: Arc::new(TransferMessageRepoForSql::new(db_connection.clone())),
        }
    }
}

impl TransferAgentRepoTrait for TransferAgentRepoForSql {
    fn get_transfer_process_repo(&self) -> Arc<dyn TransferProcessRepoTrait> {
        self.transfer_process_repo.clone()
    }
    fn get_transfer_message_repo(&self) -> Arc<dyn TransferMessageRepoTrait> {
        self.transfer_message_repo.clone()
    }
    fn get_transfer_process_identifiers_repo(&self) -> Arc<dyn TransferIdentifierRepoTrait> {
        self.transfer_process_identifier_repo.clone()
    }
}
