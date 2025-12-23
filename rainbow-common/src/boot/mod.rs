pub mod shutdown;

use crate::config::types::roles::RoleConfig;
use std::fmt::Debug;
use std::marker::PhantomData;
use tokio::sync::broadcast;

#[async_trait::async_trait]
pub trait BootstrapServiceTrait: Send + Sync {
    type Config: Debug + Clone + Send + Sync;

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
    async fn load_catalog(_config: &Self::Config) -> anyhow::Result<String> {
        anyhow::bail!("This service does not support creation of catalogs.");
    }

    fn enable_dataservice() -> bool {
        true
    }
    async fn load_dataservice(_config: &Self::Config) -> anyhow::Result<String> {
        anyhow::bail!("This service does not support creation of data services.");
    }

    async fn start_services_background(config: &Self::Config) -> anyhow::Result<broadcast::Sender<()>>;
}

#[async_trait::async_trait]
pub trait BootstrapStepTrait: Send + Sync {
    type NextState;
    async fn next_step(self) -> anyhow::Result<Self::NextState>;
}

pub struct BootstrapCurrentState<S: BootstrapStepTrait>(pub S);

// --- STRUCTS (Vagones del tren) ---
// Todos, desde ServicesStarted, deben llevar el shutdown_tx

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
    pub _marker: PhantomData<S>,
    pub env_file: Option<String>,
    pub role: RoleConfig,
}

pub struct BootstrapServicesStarted<S: BootstrapServiceTrait> {
    pub config: S::Config,
    pub shutdown_tx: broadcast::Sender<()>,
}

pub struct BootstrapSelfParticipantOnBoarded<S: BootstrapServiceTrait> {
    pub config: S::Config,
    pub shutdown_tx: broadcast::Sender<()>, // <--- IMPORTANTE
    pub participant_id: Option<String>,
}

pub struct BootstrapCatalogLoaded<S: BootstrapServiceTrait> {
    pub config: S::Config,
    pub shutdown_tx: broadcast::Sender<()>, // <--- IMPORTANTE
    pub participant_id: Option<String>,
    pub catalog_id: Option<String>,
}

pub struct BootstrapDataServiceLoaded<S: BootstrapServiceTrait> {
    pub config: S::Config,
    pub shutdown_tx: broadcast::Sender<()>, // <--- IMPORTANTE
    pub participant_id: Option<String>,
    pub catalog_id: Option<String>,
    pub dataservice_id: Option<String>,
}

pub struct BootstrapFinalized<S: BootstrapServiceTrait> {
    pub _marker: PhantomData<S>,
    pub shutdown_tx: broadcast::Sender<()>, // <--- IMPORTANTE
}

pub struct BootstrapTerminated<S: BootstrapServiceTrait>(PhantomData<S>);

// --- IMPLEMENTACIONES ---

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapInit<S> {
    type NextState = BootstrapCurrentState<BootstrapConfigLoaded<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Step [1/7]: Init bootstrap configuration");
        Ok(BootstrapCurrentState(BootstrapConfigLoaded {
            _marker: PhantomData,
            env_file: self.env_file,
            role: self.role,
        }))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapConfigLoaded<S> {
    type NextState = BootstrapCurrentState<BootstrapServicesStarted<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Step [2/7]: Configuration loading");
        let config = S::load_config(self.role, self.env_file).await?;

        tracing::info!("Step [3/7]: Starting Services in Background");
        let shutdown_tx = S::start_services_background(&config).await?;

        // Espera de seguridad para arranque de puertos
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        Ok(BootstrapCurrentState(BootstrapServicesStarted {
            config,
            shutdown_tx,
        }))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapServicesStarted<S> {
    type NextState = BootstrapCurrentState<BootstrapSelfParticipantOnBoarded<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Step [4/7]: Creating self participant");

        let participant_id =
            if S::enable_participant() { Some(S::create_participant(&self.config).await?) } else { None };

        // OJO AQUÍ: Pasamos self.shutdown_tx en AMBOS casos.
        // Si no lo pasamos aquí, se dropea y el servidor se apaga.
        Ok(BootstrapCurrentState(BootstrapSelfParticipantOnBoarded {
            config: self.config,
            shutdown_tx: self.shutdown_tx,
            participant_id,
        }))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapSelfParticipantOnBoarded<S> {
    type NextState = BootstrapCurrentState<BootstrapCatalogLoaded<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Step [5/7]: Loading main catalog");

        let catalog_id = if S::enable_catalog() { Some(S::load_catalog(&self.config).await?) } else { None };

        Ok(BootstrapCurrentState(BootstrapCatalogLoaded {
            config: self.config,
            shutdown_tx: self.shutdown_tx, // Pasando el testigo...
            participant_id: self.participant_id,
            catalog_id,
        }))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapCatalogLoaded<S> {
    type NextState = BootstrapCurrentState<BootstrapDataServiceLoaded<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Step [6/7]: Loading main dataservice");

        let dataservice_id =
            if S::enable_dataservice() { Some(S::load_dataservice(&self.config).await?) } else { None };

        Ok(BootstrapCurrentState(BootstrapDataServiceLoaded {
            config: self.config,
            shutdown_tx: self.shutdown_tx, // Pasando el testigo...
            participant_id: self.participant_id,
            catalog_id: self.catalog_id,
            dataservice_id,
        }))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapDataServiceLoaded<S> {
    type NextState = BootstrapCurrentState<BootstrapFinalized<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Step [7/7]: Bootstrap sequence completed. Services UP.");

        Ok(BootstrapCurrentState(BootstrapFinalized {
            _marker: PhantomData,
            shutdown_tx: self.shutdown_tx, // El testigo llega a la meta
        }))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapFinalized<S> {
    type NextState = BootstrapCurrentState<BootstrapTerminated<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        //
        tracing::info!("System is RUNNING. Waiting for termination signal (Ctrl+C)...");
        match tokio::signal::ctrl_c().await {
            Ok(()) => tracing::info!("Shutdown signal received."),
            Err(err) => tracing::error!("Unable to listen for shutdown signal: {}", err),
        }
        tracing::info!("Sending shutdown signal to background services...");
        let _ = self.shutdown_tx.send(()); // Disparamos el cierre

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        Ok(BootstrapCurrentState(BootstrapTerminated(PhantomData)))
    }
}

#[async_trait::async_trait]
impl<S: BootstrapServiceTrait> BootstrapStepTrait for BootstrapTerminated<S> {
    type NextState = BootstrapCurrentState<BootstrapTerminated<S>>;

    async fn next_step(self) -> anyhow::Result<Self::NextState> {
        tracing::info!("Terminating process.");
        Ok(BootstrapCurrentState(BootstrapTerminated(PhantomData)))
    }
}
