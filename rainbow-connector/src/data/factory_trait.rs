use crate::data::repo_traits::connector_instance_repo::ConnectorInstanceRepoTrait;
use crate::data::repo_traits::connector_template_repo::ConnectorTemplateRepoTrait;
use std::sync::Arc;

pub trait ConnectorRepoTrait: Send + Sync {
    fn get_templates_repo(&self) -> Arc<dyn ConnectorTemplateRepoTrait>;
    fn get_instances_repo(&self) -> Arc<dyn ConnectorInstanceRepoTrait>;
}
