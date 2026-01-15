use crate::data::factory_trait::ConnectorRepoTrait;
use crate::data::repo_sql::connector_instance_repo::ConnectorInstanceRepoForSql;
use crate::data::repo_sql::connector_template_repo::ConnectorTemplateRepoForSql;
use crate::data::repo_traits::connector_instance_repo::ConnectorInstanceRepoTrait;
use crate::data::repo_traits::connector_template_repo::ConnectorTemplateRepoTrait;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct ConnectorRepoForSql {
    templates_repo: Arc<dyn ConnectorTemplateRepoTrait>,
    instances_repo: Arc<dyn ConnectorInstanceRepoTrait>,
}

impl ConnectorRepoForSql {
    pub fn create_repo(db_connection: DatabaseConnection) -> Self {
        Self {
            templates_repo: Arc::new(ConnectorTemplateRepoForSql::new(db_connection.clone())),
            instances_repo: Arc::new(ConnectorInstanceRepoForSql::new(db_connection.clone())),
        }
    }
}

impl ConnectorRepoTrait for ConnectorRepoForSql {
    fn get_templates_repo(&self) -> Arc<dyn ConnectorTemplateRepoTrait> {
        self.templates_repo.clone()
    }

    fn get_instances_repo(&self) -> Arc<dyn ConnectorInstanceRepoTrait> {
        self.instances_repo.clone()
    }
}
