use clap::{Parser, Subcommand};
use tracing::debug;
use rainbow_common::config::env_extraction::EnvExtraction;
use crate::setup::application::TransferApplication;
use crate::setup::db_migrations::TransferAgentMigration;

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Transfer Agent")]
#[command(version = "0.2")]
struct TransferCli {
    #[clap(subcommand)]
    command: TransferCliCommands
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum TransferCliCommands {
    Start(TransferCliArgs),
    Setup(TransferCliArgs),
}

#[derive(Parser, Debug, PartialEq)]
pub struct TransferCliArgs {
    #[arg(short, long)]
    env_file: Option<String>,
}

pub struct TransferCommands {}
impl EnvExtraction for TransferCommands {}

impl TransferCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        debug!("init_command_line - Initialize transfer commands");
        let cli = TransferCli::parse();
        match cli.command {
            TransferCliCommands::Start(args) => {
                let config = Self::extract_provider_config(args.env_file)?;
                TransferApplication::run(&config).await?;
            }
            TransferCliCommands::Setup(args) => {
                let config = Self::extract_provider_config(args.env_file)?;
                TransferAgentMigration::run(&config).await?;
            }
        }
        Ok(())
    }
}