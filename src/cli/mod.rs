use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use tracing::{debug, info};
use crate::http::consumer::{start_test, start_transfer_request};

use crate::http::provider::{start_provider_server, start_provider_auth_server};


#[derive(Parser, Debug)]
#[command(name = "Dataspace protocol")]
#[command(version = "0.1")]
#[command(about = "Dataspace protocol", long_about = "Dataspace protocol")]
struct Cli {
    #[command(subcommand)]
    role: DataSpaceTransferRoles,
}

#[derive(Subcommand, Debug)]
pub enum DataSpaceTransferRoles {
    #[command(about = "Start the consumer testing scripts")]
    Consumer(ConsumerArgs),
    #[command(about = "Start the provider server")]
    Provider(ProviderArgs),
}

#[derive(Debug, Args)]
struct ConsumerArgs {
    #[arg(long)]
    provider_url: Option<String>,
    #[arg(long)]
    provider_port: Option<String>,
    #[command(subcommand)]
    command: ConsumerCommands,
}

#[derive(Debug, Args)]
struct ProviderArgs {
    #[arg(long)]
    host_url: Option<String>,
    #[arg(long)]
    host_port: Option<String>,
    #[command(subcommand)]
    command: ProviderCommands,
}

#[derive(Subcommand, Debug)]
pub enum ProviderCommands {
    Start {},
    Auth {}
}

#[derive(Subcommand, Debug)]
pub enum ConsumerCommands {
    Test,
    TransferRequest {},
    TransferSuspension {}
}

pub async fn init_command_line() -> Result<()> {
    info!("Init the command line application");
    let cli = Cli::parse();
    debug!("{:?}", cli);

    match &cli.role {
        DataSpaceTransferRoles::Consumer(args) => {
            match &args.command {
                ConsumerCommands::Test {} => {
                    start_test(&args.provider_url, &args.provider_port).await?;
                }
                ConsumerCommands::TransferRequest { .. } => {
                    start_transfer_request(&args.provider_url, &args.provider_port).await?;
                }
                ConsumerCommands::TransferSuspension { .. } => {

                }
            }
        }
        DataSpaceTransferRoles::Provider(args) => {
            match &args.command {
                ProviderCommands::Start { .. } => {
                    start_provider_server(&args.host_url, &args.host_port).await?;
                }
                ProviderCommands::Auth { .. } => {
                    start_provider_auth_server(&args.host_url, &args.host_port).await?;
                }
            }
        }
    }


    Ok(())
}

