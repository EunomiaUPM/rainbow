use crate::provider::setup::application::ContractNegotiationProviderApplication;
use crate::provider::setup::config::ContractNegotiationProviderApplicationConfig;
use crate::provider::setup::db_migrations::ContractNegotiationProviderMigration;
use clap::{Parser, Subcommand};
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Contract Negotiation Server")]
#[command(version = "0.2")]
struct ContractNegotiationCli {
    #[command(subcommand)]
    role: ContractNegotiationCliRoles,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum ContractNegotiationCliRoles {
    #[command(subcommand)]
    Provider(ContractNegotiationCliCommands),
    #[command(subcommand)]
    Consumer(ContractNegotiationCliCommands),
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum ContractNegotiationCliCommands {
    Start,
    Setup,
}

pub struct ContractNegotiationCommands;

impl ContractNegotiationCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = ContractNegotiationCli::parse();

        // run scripts
        match cli.role {
            ContractNegotiationCliRoles::Provider(cmd) => {
                let config = ContractNegotiationProviderApplicationConfig::default();
                let config = match config.merge_dotenv_configuration() {
                    Ok(config) => config,
                    Err(_) => config
                };
                let table =
                    json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                info!("Current config:\n{}", table);
                match cmd {
                    ContractNegotiationCliCommands::Start => ContractNegotiationProviderApplication::run(&config).await?,
                    ContractNegotiationCliCommands::Setup => ContractNegotiationProviderMigration::run(&config).await?,
                }
            }
            ContractNegotiationCliRoles::Consumer(_) => {
                // let config = TransferConsumerApplicationConfig::default();
                // let config = match config.merge_dotenv_configuration() {
                //     Ok(config) => config,
                //     Err(_) => config
                // };
                // let table =
                //     json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                // info!("Current config:\n{}", table);
                // match cmd {
                //     ContractNegotiationCliCommands::Start => TransferConsumerApplication::run(config.clone()).await?,
                //     ContractNegotiationCliCommands::Setup => TransferConsumerMigration::run(config.clone()).await?
                // }
            }
        };

        Ok(())
    }
}