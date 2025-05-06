use axum::async_trait;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use urn::Urn;

pub mod catalog_odrl_facade;

#[mockall::automock]
#[async_trait]
pub trait CatalogOdrlFacadeTrait: Send + Sync {
    async fn resolve_odrl_offers(&self, offer_id: Urn) -> anyhow::Result<OdrlOffer>;
}