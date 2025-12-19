use rainbow_common::dcat_formats::{DctFormats, FormatAction};
use rainbow_common::protocol::transfer::TransferRoles;
use crate::common::core::data_plane_facade::dataplane_strategies::consumer_pull_strategy::ConsumerPullDataplaneStrategy;
use crate::common::core::data_plane_facade::dataplane_strategies::consumer_push_strategy::ConsumerPushDataplaneStrategy;
use crate::common::core::data_plane_facade::dataplane_strategies::provider_pull_strategy::ProviderPullDataplaneStrategy;
use crate::common::core::data_plane_facade::dataplane_strategies::provider_push_strategy::ProviderPushDataplaneStrategy;
use crate::common::core::data_plane_facade::DataPlaneFacadeTrait;

pub struct DataPlaneStrategyFactory;

impl DataPlaneStrategyFactory {
    pub fn get_strategy(&self, role: &TransferRoles, format: &DctFormats) -> Box<dyn DataPlaneFacadeTrait> {
        match (role, format.action) {
            (TransferRoles::Provider, FormatAction::Pull) => Box::new(ProviderPullDataplaneStrategy),
            (TransferRoles::Provider, FormatAction::Push) => Box::new(ProviderPushDataplaneStrategy),
            (TransferRoles::Consumer, FormatAction::Pull) => Box::new(ConsumerPullDataplaneStrategy),
            (TransferRoles::Consumer, FormatAction::Push) => Box::new(ConsumerPushDataplaneStrategy)
        }
    }
}
