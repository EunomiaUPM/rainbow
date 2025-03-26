use crate::consumer::setup::application::TransferConsumerApplication;
use crate::consumer::setup::config::TransferConsumerApplicationConfig;
use crate::consumer::setup::db_migrations::TransferConsumerMigration;
use crate::provider::setup::application::TransferProviderApplication;
use crate::provider::setup::config::TransferProviderApplicationConfig;
use crate::provider::setup::db_migrations::TransferProviderMigration;
use clap::{Parser, Subcommand};
use std::cmp::PartialEq;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Transfer Provider Server")]
#[command(version = "0.2")]
struct TransferCli {
    #[command(subcommand)]
    role: TransferCliRoles,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum TransferCliRoles {
    #[command(subcommand)]
    Provider(TransferCliCommands),
    #[command(subcommand)]
    Consumer(TransferCliCommands),
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum TransferCliCommands {
    Start,
    Setup,
}

pub struct TransferCommands;

impl TransferCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = TransferCli::parse();

        // run scripts
        match cli.role {
            TransferCliRoles::Provider(cmd) => {
                let config = TransferProviderApplicationConfig::default();
                let config = match config.merge_dotenv_configuration() {
                    Ok(config) => config,
                    Err(_) => config
                };
                let table =
                    json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                info!("Current config:\n{}", table);
                match cmd {
                    TransferCliCommands::Start => TransferProviderApplication::run(&config).await?,
                    TransferCliCommands::Setup => TransferProviderMigration::run(&config).await?,
                }
            }
            TransferCliRoles::Consumer(cmd) => {
                let config = TransferConsumerApplicationConfig::default();
                let config = match config.merge_dotenv_configuration() {
                    Ok(config) => config,
                    Err(_) => config
                };
                let table =
                    json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                info!("Current config:\n{}", table);
                match cmd {
                    TransferCliCommands::Start => TransferConsumerApplication::run(config.clone()).await?,
                    TransferCliCommands::Setup => TransferConsumerMigration::run(config.clone()).await?
                }
            }
        };

        Ok(())
    }
}
