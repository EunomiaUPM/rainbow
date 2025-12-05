use crate::protocols::dsp::facades::data_plane_facade::dataplane_strategies::consumer_pull_strategy::ConsumerPullDataplaneStrategy;
use crate::protocols::dsp::facades::data_plane_facade::dataplane_strategies::consumer_push_strategy::ConsumerPushDataplaneStrategy;
use crate::protocols::dsp::facades::data_plane_facade::dataplane_strategies::provider_pull_strategy::ProviderPullDataplaneStrategy;
use crate::protocols::dsp::facades::data_plane_facade::dataplane_strategies::provider_push_strategy::ProviderPushDataplaneStrategy;
use crate::protocols::dsp::facades::data_plane_facade::DataPlaneFacadeTrait;
use rainbow_common::dcat_formats::{DctFormats, FormatAction};
use rainbow_common::protocol::transfer::TransferRoles;
use rainbow_dataplane::coordinator::dataplane_access_controller::DataPlaneAccessControllerTrait;
use std::sync::Arc;

pub struct DataPlaneStrategyFactory {
    dataplane_access_controller: Arc<dyn DataPlaneAccessControllerTrait>,
}

impl DataPlaneStrategyFactory {
    pub fn new(dataplane_access_controller: Arc<dyn DataPlaneAccessControllerTrait>) -> Self {
        Self { dataplane_access_controller }
    }
    pub fn get_strategy(&self, role: &TransferRoles, format: &DctFormats) -> Box<dyn DataPlaneFacadeTrait> {
        match (role, format.action) {
            (TransferRoles::Provider, FormatAction::Pull) => Box::new(ProviderPullDataplaneStrategy::new(
                self.dataplane_access_controller.clone(),
            )),
            (TransferRoles::Provider, FormatAction::Push) => Box::new(ProviderPushDataplaneStrategy::new(
                self.dataplane_access_controller.clone(),
            )),
            (TransferRoles::Consumer, FormatAction::Pull) => Box::new(ConsumerPullDataplaneStrategy::new(
                self.dataplane_access_controller.clone(),
            )),
            (TransferRoles::Consumer, FormatAction::Push) => Box::new(ConsumerPushDataplaneStrategy::new(
                self.dataplane_access_controller.clone(),
            )),
        }
    }
}
