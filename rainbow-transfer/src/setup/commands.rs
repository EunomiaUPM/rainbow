use crate::provider::setup::application::TransferProviderApplication;
use crate::provider::setup::config::TransferProviderApplicationConfig;
use crate::provider::setup::db_migrations::TransferProviderMigration;
use clap::{Parser, Subcommand};
use rainbow_db::transfer_provider::repo::sql::TransferProviderRepoForSql;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Transfer Provider Server")]
#[command(version = "0.2")]
struct TransferProviderCli {
    #[command(subcommand)]
    role: TransferCliRoles,
}

#[derive(Subcommand, Debug)]
pub enum TransferCliRoles {
    #[command(subcommand)]
    Provider(TransferCliCommands),
    #[command(subcommand)]
    Consumer(TransferCliCommands),
}

#[derive(Subcommand, Debug)]
pub enum TransferCliCommands {
    Start,
    Setup,
}

pub struct TransferProviderCommands;

impl TransferProviderCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        info!("Init the command line application");
        let cli = TransferProviderCli::parse();

        // parse config
        let config = match cli.role {
            TransferCliRoles::Provider(_) => {
                let config = TransferProviderApplicationConfig::default();
                let config = config.merge_dotenv_configuration().expect("Error with env file");
                config
            }
            TransferCliRoles::Consumer(_) => todo!(),
        };

        let table =
            json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
        info!("Current config:\n{}", table);

        // run scripts
        match cli.role {
            TransferCliRoles::Provider(cmd) => match cmd {
                TransferCliCommands::Start => TransferProviderApplication::run(&config).await?,
                TransferCliCommands::Setup => TransferProviderMigration::run(&config).await?,
            },
            TransferCliRoles::Consumer(cmd) => match cmd {
                TransferCliCommands::Start => {}
                TransferCliCommands::Setup => {}
            },
        }

        Ok(())
    }
}
