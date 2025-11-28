use crate::protocols::dsp::facades::data_plane_facade::DataPlaneProviderFacadeTrait;
use crate::protocols::dsp::facades::data_service_resolver_facade::DataServiceFacadeTrait;
use std::sync::Arc;

pub mod data_plane_facade;
pub mod data_service_resolver_facade;

#[async_trait::async_trait]
pub trait FacadeTrait: Send + Sync {
    async fn get_data_service_facade(&self) -> Arc<dyn DataServiceFacadeTrait>;
    async fn get_data_plane_facade(&self) -> Arc<dyn DataPlaneProviderFacadeTrait>;
}

pub struct FacadeService {
    data_service_resolver_facade: Arc<dyn DataServiceFacadeTrait>,
    data_plane_facade: Arc<dyn DataPlaneProviderFacadeTrait>,
}

impl FacadeService {
    pub fn new(
        data_service_resolver_facade: Arc<dyn DataServiceFacadeTrait>,
        data_plane_facade: Arc<dyn DataPlaneProviderFacadeTrait>,
    ) -> FacadeService {
        Self { data_service_resolver_facade, data_plane_facade }
    }
}

#[async_trait::async_trait]
impl FacadeTrait for FacadeService {
    async fn get_data_service_facade(&self) -> Arc<dyn DataServiceFacadeTrait> {
        self.data_service_resolver_facade.clone()
    }

    async fn get_data_plane_facade(&self) -> Arc<dyn DataPlaneProviderFacadeTrait> {
        self.data_plane_facade.clone()
    }
}
