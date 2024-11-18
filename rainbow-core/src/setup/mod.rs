pub mod databases;

use crate::http::{get_consumer_routes, get_provider_routes};
use crate::migrations::{migrate_consumer_db, migrate_provider_db};
use crate::setup::databases::get_db_connection;
use anyhow::Result;
use axum::serve;
use clap::{Args, Parser, Subcommand};
use dotenvy::dotenv;
use rainbow_transfer::config_field;
use rainbow_transfer::setup::config::{Config, ConfigRoles, GLOBAL_CONFIG};
use sea_orm_migration::MigratorTrait;
use std::env;
use tokio::net::TcpListener;
use tracing::{debug, error, info};

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
    #[arg(long)]
    db_type: Option<String>,
    #[arg(long)]
    db_url: Option<String>,
    #[arg(long)]
    db_port: Option<String>,
    #[arg(long)]
    db_user: Option<String>,
    #[arg(long)]
    db_password: Option<String>,
    #[arg(long)]
    db_database: Option<String>,
    #[command(subcommand)]
    command: ConsumerCommands,
}

#[derive(Debug, Args)]
pub struct ProviderArgs {
    #[arg(long)]
    host_url: Option<String>,
    #[arg(long)]
    host_port: Option<String>,
    #[arg(long)]
    db_type: Option<String>,
    #[arg(long)]
    db_url: Option<String>,
    #[arg(long)]
    db_port: Option<String>,
    #[arg(long)]
    db_user: Option<String>,
    #[arg(long)]
    db_password: Option<String>,
    #[arg(long)]
    db_database: Option<String>,
    #[arg(long)]
    auth_url: Option<String>,
    #[arg(long)]
    auth_port: Option<String>,
    #[command(subcommand)]
    command: ProviderCommands,
}

#[derive(Subcommand, Debug)]
pub enum ProviderCommands {
    Start {},
    Setup,
    LoadContracts,
}

#[derive(Subcommand, Debug)]
pub enum ConsumerCommands {
    Start {},
    Setup,
}

pub async fn init_command_line() -> Result<()> {
    info!("Init the command line application");
    let cli = Cli::parse();
    if env::var_os("TEST").is_none() {
        dotenv().ok();
    }

    let config = match &cli.role {
        DataSpaceTransferRoles::Provider(args) => Config {
            host_url: config_field!(args, host_url, "HOST_URL", "localhost"),
            host_port: config_field!(args, host_port, "HOST_PORT", "1234"),
            db_type: config_field!(args, db_type, "DB_TYPE", "postgres"),
            db_url: config_field!(args, db_url, "DB_URL", "localhost"),
            db_port: config_field!(args, db_port, "DB_PORT", "5433"),
            db_user: config_field!(args, db_user, "DB_USER", "ds-protocol-provider"),
            db_password: config_field!(args, db_password, "DB_PASSWORD", "ds-protocol-provider"),
            db_database: config_field!(args, db_database, "DB_DATABASE", "ds-protocol-provider"),
            provider_url: None,
            provider_port: None,
            auth_url: Some(config_field!(
                args,
                auth_url,
                "AUTH_URL",
                "localhost"
            )),
            auth_port: Some(config_field!(args, auth_port, "AUTH_PORT", "1232")),
            role: ConfigRoles::Provider,
        },
        DataSpaceTransferRoles::Consumer(args) => Config {
            host_url: config_field!(args, host_url, "HOST_URL", "localhost"),
            host_port: config_field!(args, host_port, "HOST_PORT", "1235"),
            db_type: config_field!(args, db_type, "DB_TYPE", "postgres"),
            db_url: config_field!(args, db_url, "DB_URL", "localhost"),
            db_port: config_field!(args, db_port, "DB_PORT", "5434"),
            db_user: config_field!(args, db_user, "DB_USER", "ds-protocol-consumer"),
            db_password: config_field!(args, db_password, "DB_PASSWORD", "ds-protocol-consumer"),
            db_database: config_field!(args, db_database, "DB_DATABASE", "ds-protocol-consumer"),
            provider_url: Some(config_field!(
                args,
                provider_url,
                "PROVIDER_HOST",
                "localhost"
            )),
            provider_port: Some(config_field!(args, provider_port, "PROVIDER_PORT", "1234")),
            auth_url: None,
            auth_port: None,
            role: ConfigRoles::Consumer,
        },
    };

    GLOBAL_CONFIG
        .set(config)
        .expect("Global Config not initialized");

    info!("Config status: \n{}", serde_json::to_string_pretty(GLOBAL_CONFIG.get().unwrap())?);

    match &cli.role {
        DataSpaceTransferRoles::Consumer(args) => {
            // CONFIG FOR CONSUMER HERE
            match &args.command {
                ConsumerCommands::Start { .. } => {
                    let config = GLOBAL_CONFIG.get().unwrap();
                    let server_message = format!(
                        "Starting consumer server in http://{}:{}",
                        config.host_url, config.host_port
                    );
                    info!("{}", server_message);
                    let listener = TcpListener::bind(format!("{}:{}", config.host_url, config.host_port)).await?;
                    serve(listener, get_consumer_routes().await).await?;
                }
                ConsumerCommands::Setup => {
                    migrate_consumer_db().await?;
                }
            }
        }
        DataSpaceTransferRoles::Provider(args) => {
            // CONFIG FOR PROVIDER HERE
            match &args.command {
                ProviderCommands::Start { .. } => {
                    let config = GLOBAL_CONFIG.get().unwrap();
                    let server_message = format!(
                        "Starting provider server in http://{}:{}",
                        config.host_url, config.host_port
                    );
                    info!("{}", server_message);
                    let listener = TcpListener::bind(format!("{}:{}", config.host_url, config.host_port)).await?;
                    serve(listener, get_provider_routes().await).await?;
                }
                ProviderCommands::Setup => {
                    migrate_provider_db().await?;
                }
                ProviderCommands::LoadContracts => {}
            }
        }
    }

    Ok(())
}
