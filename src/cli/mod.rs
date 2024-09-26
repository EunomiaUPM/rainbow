use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use tracing::{debug, info};

use crate::auth::start_provider_auth_server;
use crate::cli::DataSpaceTransferRoles::Consumer;
use crate::transfer::consumer::http::server::start_consumer_server;
use crate::transfer::consumer::*;
use crate::transfer::provider::http::server::start_provider_server;

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
pub struct ConsumerArgs {
    #[arg(long)]
    provider_url: Option<String>,
    #[arg(long)]
    provider_port: Option<String>,
    #[arg(long)]
    host_url: Option<String>,
    #[arg(long)]
    host_port: Option<String>,
    #[command(subcommand)]
    command: ConsumerCommands,
}

#[derive(Debug, Args)]
pub struct ProviderArgs {
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
    Auth {},
}

#[derive(Subcommand, Debug)]
pub enum ConsumerCommands {
    Test,
    TransferRequest {},
    TransferStart {},
    TransferSuspension {},
    TransferCompletion {},
    TransferTermination {},
    Start {},
}

pub async fn init_command_line() -> Result<()> {
    info!("Init the command line application");
    let cli = Cli::parse();
    debug!("{:?}", cli);

    match &cli.role {
        DataSpaceTransferRoles::Consumer(args) => match &args.command {
            ConsumerCommands::Test {} => {
                start_test(&args.provider_url, &args.provider_port).await?;
            }
            ConsumerCommands::TransferRequest { .. } => {
                start_transfer_request(&args.provider_url, &args.provider_port).await?;
            }
            ConsumerCommands::TransferStart { .. } => {
                start_transfer_start(&args.provider_url, &args.provider_port).await?;
            }
            ConsumerCommands::TransferSuspension { .. } => {
                start_transfer_suspension(&args.provider_url, &args.provider_port).await?;
            }
            ConsumerCommands::TransferCompletion { .. } => {
                start_transfer_completion(&args.provider_url, &args.provider_port).await?;
            }
            ConsumerCommands::TransferTermination { .. } => {
                start_transfer_termination(&args.provider_url, &args.provider_port).await?;
            }
            ConsumerCommands::Start { .. } => {
                start_consumer_server(&args.host_url, &args.host_port).await?;
            }
        },
        DataSpaceTransferRoles::Provider(args) => match &args.command {
            ProviderCommands::Start { .. } => {
                start_provider_server(&args.host_url, &args.host_port).await?;
            }
            ProviderCommands::Auth { .. } => {
                start_provider_auth_server(&args.host_url, &args.host_port).await?;
            }
        },
    }

    Ok(())
}
