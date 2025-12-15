pub(crate) mod odrl_policies;

use crate::data::entities::odrl_offer;
use crate::data::entities::odrl_offer::NewOdrlOfferModel;
use serde::{Deserialize, Serialize};
use urn::Urn;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OdrlPolicyDto {
    #[serde(flatten)]
    pub inner: odrl_offer::Model,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NewOdrlPolicyDto {
    pub id: Option<Urn>,
    pub odrl_offers: Option<serde_json::Value>,
    pub entity_id: Urn,
    pub entity_type: String,
}

impl From<NewOdrlPolicyDto> for NewOdrlOfferModel {
    fn from(dto: NewOdrlPolicyDto) -> Self {
        Self { id: dto.id, odrl_offers: dto.odrl_offers, entity_id: dto.entity_id, entity_type: dto.entity_type }
    }
}

impl From<odrl_offer::Model> for OdrlPolicyDto {
    fn from(value: odrl_offer::Model) -> Self {
        Self { inner: value }
    }
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait OdrlPolicyEntityTrait: Sync + Send {
    async fn get_all_odrl_offers(&self, limit: Option<u64>, page: Option<u64>) -> anyhow::Result<Vec<OdrlPolicyDto>>;
    async fn get_batch_odrl_offers(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<OdrlPolicyDto>>;
    async fn get_all_odrl_offers_by_entity(&self, entity: &Urn) -> anyhow::Result<Vec<OdrlPolicyDto>>;
    async fn get_odrl_offer_by_id(&self, odrl_offer_id: &Urn) -> anyhow::Result<Option<OdrlPolicyDto>>;
    async fn create_odrl_offer(&self, new_odrl_offer_model: &NewOdrlPolicyDto) -> anyhow::Result<OdrlPolicyDto>;
    async fn delete_odrl_offer_by_id(&self, odrl_offer_id: &Urn) -> anyhow::Result<()>;
    async fn delete_odrl_offers_by_entity(&self, entity_id: &Urn) -> anyhow::Result<()>;
}
