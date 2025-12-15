pub(crate) mod policy_templates;

use crate::data::entities::odrl_offer::NewOdrlOfferModel;
use crate::data::entities::policy_template::NewPolicyTemplateModel;
use crate::data::entities::{odrl_offer, policy_template};
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PolicyTemplateDto {
    #[serde(flatten)]
    pub inner: PolicyTemplateDto,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NewPolicyTemplateDto {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: serde_json::Value,
    pub operand_options: Option<serde_json::Value>,
}

impl From<NewPolicyTemplateDto> for NewPolicyTemplateModel {
    fn from(dto: NewPolicyTemplateDto) -> Self {
        Self {
            id: dto.id,
            title: dto.title,
            description: dto.description,
            content: dto.content,
            operand_options: dto.operand_options,
        }
    }
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait PolicyTemplateEntityTrait {
    async fn get_all_policy_templates(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<PolicyTemplateDto>>;
    async fn get_batch_policy_templates(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<PolicyTemplateDto>>;
    async fn get_policy_template_by_id(&self, template_id: &Urn) -> anyhow::Result<Option<PolicyTemplateDto>>;
    async fn create_policy_template(
        &self,
        new_policy_template: &NewPolicyTemplateDto,
    ) -> anyhow::Result<PolicyTemplateDto>;
    async fn delete_policy_template_by_id(&self, template_id: &Urn) -> anyhow::Result<()>;
}
