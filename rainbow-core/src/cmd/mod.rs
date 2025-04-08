use crate::provider::setup::application::CoreProviderApplication;
use crate::provider::setup::config::CoreProviderApplicationConfig;
use crate::provider::setup::db_migrations::CoreProviderMigration;
use clap::{Parser, Subcommand};
use std::cmp::PartialEq;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Core Server")]
#[command(version = "0.2")]
struct CoreCli {
    #[command(subcommand)]
    role: CoreCliRoles,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum CoreCliRoles {
    #[command(subcommand)]
    Provider(CoreCliCommands),
    #[command(subcommand)]
    Consumer(CoreCliCommands),
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum CoreCliCommands {
    Start,
    Setup,
}

pub struct CoreCommands;

impl CoreCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = CoreCli::parse();

        // run scripts
        match cli.role {
            CoreCliRoles::Provider(cmd) => {
                let config = CoreProviderApplicationConfig::default();
                let config = match config.merge_dotenv_configuration() {
                    Ok(config) => config,
                    Err(_) => config
                };
                let table =
                    json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                info!("Current config:\n{}", table);
                match cmd {
                    CoreCliCommands::Start => CoreProviderApplication::run(&config).await?,
                    CoreCliCommands::Setup => CoreProviderMigration::run(&config).await?,
                }
            }
            CoreCliRoles::Consumer(cmd) => {
                todo!()
            }
        };

        Ok(())
    }
}
