use crate::core::subscription::subscription_types::{RainbowEventsSubscriptionCreationRequest, RainbowEventsSubscriptionCreationResponse, SubscriptionEntities};
use axum::async_trait;
use urn::Urn;

pub mod subscription;
pub mod subscription_err;
pub mod subscription_types;

#[mockall::automock]
#[async_trait]
pub trait RainbowEventsSubscriptionTrait: Send + Sync {
    async fn get_all_subscriptions(&self) -> anyhow::Result<Vec<RainbowEventsSubscriptionCreationResponse>>;
    async fn get_subscription_by_id(
        &self,
        subscription_id: Urn,
    ) -> anyhow::Result<RainbowEventsSubscriptionCreationResponse>;
    async fn put_subscription_by_id(
        &self,
        subscription_id: Urn,
        input: RainbowEventsSubscriptionCreationRequest,
    ) -> anyhow::Result<RainbowEventsSubscriptionCreationResponse>;
    async fn create_subscription(
        &self,
        input: RainbowEventsSubscriptionCreationRequest,
        subscription_type: SubscriptionEntities,
    ) -> anyhow::Result<RainbowEventsSubscriptionCreationResponse>;
    async fn delete_subscription_by_id(&self, subscription_id: Urn) -> anyhow::Result<()>;
}
