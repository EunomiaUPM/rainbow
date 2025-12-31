use crate::cache::factory_trait::CatalogAgentCacheTrait;
use crate::data::factory_trait::CatalogAgentRepoTrait;
use crate::entities::odrl_policies::{NewOdrlPolicyDto, OdrlPolicyDto, OdrlPolicyEntityTrait};
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::sync::Arc;
use tracing::error;
use urn::Urn;

pub struct OdrlPolicyEntities {
    repo: Arc<dyn CatalogAgentRepoTrait>,
    cache: Arc<dyn CatalogAgentCacheTrait>,
}

impl OdrlPolicyEntities {
    pub fn new(repo: Arc<dyn CatalogAgentRepoTrait>, cache: Arc<dyn CatalogAgentCacheTrait>) -> Self {
        Self { repo, cache }
    }
}

#[async_trait::async_trait]
impl OdrlPolicyEntityTrait for OdrlPolicyEntities {
    async fn get_all_odrl_offers(&self, limit: Option<u64>, page: Option<u64>) -> anyhow::Result<Vec<OdrlPolicyDto>> {
        let odrl_policies = self.repo.get_odrl_offer_repo().get_all_odrl_offers(limit, page).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let mut dtos = Vec::with_capacity(odrl_policies.len());
        for c in odrl_policies {
            let dto: OdrlPolicyDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_batch_odrl_offers(&self, ids: &Vec<Urn>) -> anyhow::Result<Vec<OdrlPolicyDto>> {
        let odrl_policies = self.repo.get_odrl_offer_repo().get_batch_odrl_offers(ids).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let mut dtos = Vec::with_capacity(odrl_policies.len());
        for c in odrl_policies {
            let dto: OdrlPolicyDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_all_odrl_offers_by_entity(&self, entity: &Urn) -> anyhow::Result<Vec<OdrlPolicyDto>> {
        let odrl_policies =
            self.repo.get_odrl_offer_repo().get_all_odrl_offers_by_entity(entity).await.map_err(|e| {
                let err = CommonErrors::database_new(&e.to_string());
                error!("{}", err.log());
                err
            })?;
        let mut dtos = Vec::with_capacity(odrl_policies.len());
        for c in odrl_policies {
            let dto: OdrlPolicyDto = c.into();
            dtos.push(dto);
        }
        Ok(dtos)
    }

    async fn get_odrl_offer_by_id(&self, odrl_offer_id: &Urn) -> anyhow::Result<Option<OdrlPolicyDto>> {
        let odrl_policy = self.repo.get_odrl_offer_repo().get_odrl_offer_by_id(odrl_offer_id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let dto = odrl_policy.map(|o| o.into());
        Ok(dto)
    }

    async fn create_odrl_offer(&self, new_odrl_offer_model: &NewOdrlPolicyDto) -> anyhow::Result<OdrlPolicyDto> {
        let new_model = new_odrl_offer_model.clone().into();
        let odrl_policy = self.repo.get_odrl_offer_repo().create_odrl_offer(&new_model).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        let dto: OdrlPolicyDto = odrl_policy.into();
        Ok(dto)
    }

    async fn delete_odrl_offer_by_id(&self, odrl_offer_id: &Urn) -> anyhow::Result<()> {
        self.repo.get_odrl_offer_repo().delete_odrl_offer_by_id(odrl_offer_id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        Ok(())
    }

    async fn delete_odrl_offers_by_entity(&self, entity_id: &Urn) -> anyhow::Result<()> {
        self.repo.get_odrl_offer_repo().delete_odrl_offers_by_entity(entity_id).await.map_err(|e| {
            let err = CommonErrors::database_new(&e.to_string());
            error!("{}", err.log());
            err
        })?;
        Ok(())
    }
}
