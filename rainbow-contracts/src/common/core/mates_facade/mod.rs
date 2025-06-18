use axum::async_trait;
use rainbow_common::mates::Mates;
use urn::Urn;

pub mod mates_facade;

#[mockall::automock]
#[async_trait]
pub trait MatesFacadeTrait: Send + Sync {
    async fn get_mate_by_id(&self, mate_id: Urn) -> anyhow::Result<Mates>;
    async fn get_mate_by_slug(&self, mate_slug: String) -> anyhow::Result<Mates>;
    async fn get_me_mate(&self) -> anyhow::Result<Mates>;
}