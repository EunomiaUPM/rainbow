use crate::data::entities::policy_template;
use crate::data::entities::policy_template::NewPolicyTemplateModel;
use crate::data::repo_traits::policy_template_repo::{PolicyTemplatesRepoErrors, PolicyTemplatesRepositoryTrait};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use urn::Urn;

pub struct PolicyTemplatesRepositoryForSql {
    db_connection: DatabaseConnection,
}

impl PolicyTemplatesRepositoryForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl PolicyTemplatesRepositoryTrait for PolicyTemplatesRepositoryForSql {
    async fn get_all_policy_templates(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<policy_template::Model>, PolicyTemplatesRepoErrors> {
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);
        let offset = (page - 1) * limit;

        // Construir la consulta
        match policy_template::Entity::find()
            .order_by_desc(policy_template::Column::CreatedAt)
            .limit(limit)
            .offset(offset)
            .all(&self.db_connection)
            .await
        {
            Ok(templates) => Ok(templates),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorFetchingPolicyTemplate(
                err.into(),
            )),
        }
    }

    async fn get_batch_policy_templates(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<policy_template::Model>, PolicyTemplatesRepoErrors> {
        let policy_ids = ids.iter().map(|t| t.to_string()).collect::<Vec<_>>();
        let policy_process = policy_template::Entity::find()
            .filter(policy_template::Column::Id.is_in(policy_ids))
            .all(&self.db_connection)
            .await;
        match policy_process {
            Ok(odrl_process) => Ok(odrl_process),
            Err(e) => Err(PolicyTemplatesRepoErrors::ErrorFetchingPolicyTemplate(
                e.into(),
            )),
        }
    }

    async fn get_policy_template_by_id(
        &self,
        template_id: &Urn,
    ) -> anyhow::Result<Option<policy_template::Model>, PolicyTemplatesRepoErrors> {
        let template_id = template_id.to_string();
        match policy_template::Entity::find_by_id(template_id).one(&self.db_connection).await {
            Ok(template) => Ok(template),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorFetchingPolicyTemplate(
                err.into(),
            )),
        }
    }

    async fn create_policy_template(
        &self,
        new_policy_template: &NewPolicyTemplateModel,
    ) -> anyhow::Result<policy_template::Model, PolicyTemplatesRepoErrors> {
        let model: policy_template::ActiveModel = new_policy_template.into();
        match policy_template::Entity::insert(model).exec_with_returning(&self.db_connection).await {
            Ok(template) => Ok(template),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorCreatingPolicyTemplate(
                err.into(),
            )),
        }
    }

    async fn delete_policy_template_by_id(&self, template_id: &Urn) -> anyhow::Result<(), PolicyTemplatesRepoErrors> {
        let template_id = template_id.to_string();
        match policy_template::Entity::delete_by_id(template_id).exec(&self.db_connection).await {
            Ok(_) => Ok(()),
            Err(err) => Err(PolicyTemplatesRepoErrors::ErrorDeletingPolicyTemplate(
                err.into(),
            )),
        }
    }
}
