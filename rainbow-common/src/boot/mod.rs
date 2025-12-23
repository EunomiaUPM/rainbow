pub mod shutdown;

use crate::config::types::roles::RoleConfig;
use std::fmt::Debug;
use std::marker::PhantomData;

#[async_trait::async_trait]
pub trait BootstrapServiceTrait: Send + Sync {
    type Config: Debug + Clone + Send + Sync; // whatever type - config hasn't common trait yet
    async fn load_config(role: RoleConfig, env_file: Option<String>) -> anyhow::Result<Self::Config>;
    fn enable_participant() -> bool {
        true
    }
    async fn create_participant(_config: &Self::Config) -> anyhow::Result<String> {
        anyhow::bail!("This service does not support creation of participants.");
    }
    fn enable_catalog() -> bool {
        true
    }
    async fn load_catalog(_config: &Self::Config) -> anyhow::Result<Vec<String>> {
        anyhow::bail!("This service does not support creation of catalogs.");
    }
    fn enable_dataservice() -> bool {
        true
    }
    async fn load_dataservice(_config: &Self::Config) -> anyhow::Result<Vec<String>> {
        anyhow::bail!("This service does not support creation of data services.");
    }
    async fn start_services(
        config: &Self::Config,
        participant_id: Option<String>,
        catalog_id: Option<String>,
    ) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait BootstrapStepTrait: Send + Sync {
    type NextState;
    async fn next_step(self) -> anyhow::Result<Self::NextState>;
}

pub struct BootstrapCurrentState<S: BootstrapStepTrait>(pub S);
pub struct BootstrapInit<S: BootstrapServiceTrait> {
    pub _marker: PhantomData<S>,
    pub env_file: Option<String>,
    pub role: RoleConfig,
}

impl<S: BootstrapServiceTrait> BootstrapInit<S> {
    pub fn new(role: RoleConfig, env_file: Option<String>) -> Self {
        Self { _marker: PhantomData, env_file, role }
    }
}
pub struct BootstrapConfigLoaded<S: BootstrapServiceTrait> {
    pub config: S::Config,
}
pub struct BootstrapSelfParticipantOnBoarded<S: BootstrapServiceTrait> {
    pub config: S::Config,
    pub participant_id: Option<String>,
}
pub struct BootstrapCatalogLoaded<S: BootstrapServiceTrait> {
    pub config: S::Config,
    pub catalog_id: Option<String>,
    pub participant_id: Option<String>,
}
pub struct BootstrapDataServiceLoaded<S: BootstrapServiceTrait> {
    pub config: S::Config,
    pub catalog_id: Option<String>,
    pub participant_id: Option<String>,
}
pub struct BootstrapUpAndRunning<S: BootstrapServiceTrait> {
    pub config: S::Config,
    pub catalog_id: Option<String>,
    pub participant_id: Option<String>,
}
pub struct BootstrapFinalized<S: BootstrapServiceTrait>(PhantomData<S>);
pub struct BootstrapTerminated<S: BootstrapServiceTrait>(PhantomData<S>);

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapInit<S> {
    type NextState = BootstrapCurrentState<BootstrapConfigLoaded<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Step [1/6]: Init bootstrap configuration");
        let config = S::load_config(self.role, self.env_file).await?;
        Ok(BootstrapCurrentState(BootstrapConfigLoaded { config }))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapConfigLoaded<S> {
    type NextState = BootstrapCurrentState<BootstrapSelfParticipantOnBoarded<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Step [2/6]: Configuration loading");
        // create participant....
        Ok(BootstrapCurrentState(BootstrapSelfParticipantOnBoarded {
            config: self.config,
            participant_id: None,
        }))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapSelfParticipantOnBoarded<S> {
    type NextState = BootstrapCurrentState<BootstrapCatalogLoaded<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Step [3/6]: Creating self participant");
        Ok(BootstrapCurrentState(BootstrapCatalogLoaded {
            config: self.config,
            catalog_id: None,
            participant_id: None,
        }))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapCatalogLoaded<S> {
    type NextState = BootstrapCurrentState<BootstrapDataServiceLoaded<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Step [4/6]: Loading main dataservice");
        Ok(BootstrapCurrentState(BootstrapDataServiceLoaded {
            config: self.config,
            catalog_id: None,
            participant_id: None,
        }))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapDataServiceLoaded<S> {
    type NextState = BootstrapCurrentState<BootstrapUpAndRunning<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Step [5/6]: Loading main catalog");
        Ok(BootstrapCurrentState(BootstrapUpAndRunning {
            config: self.config,
            catalog_id: None,
            participant_id: None,
        }))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapUpAndRunning<S> {
    type NextState = BootstrapCurrentState<BootstrapFinalized<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Step [6/6]: Service up and running");
        S::start_services(&self.config, self.participant_id, self.catalog_id).await?;
        Ok(BootstrapCurrentState(BootstrapFinalized(PhantomData)))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapFinalized<S> {
    type NextState = BootstrapCurrentState<BootstrapFinalized<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Finalizing");
        Ok(BootstrapCurrentState(BootstrapFinalized(PhantomData)))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapTerminated<S> {
    type NextState = BootstrapCurrentState<BootstrapTerminated<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Terminating");
        Ok(BootstrapCurrentState(BootstrapTerminated(PhantomData)))
    }
}
