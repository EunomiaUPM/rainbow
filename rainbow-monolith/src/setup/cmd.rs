/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use crate::consumer::db_migrations::CoreConsumerMigration;
use crate::provider::db_migrations::CoreProviderMigration;
use crate::provider::db_seeding::CoreProviderSeeding;
use crate::setup::CoreHttpWorker;
use clap::{Parser, Subcommand};
use rainbow_common::config::traits::MonoConfigTrait;
use rainbow_common::config::types::roles::RoleConfig;
use rainbow_common::config::ApplicationConfig;
use std::cmp::PartialEq;
use tracing::{debug, info};
use rainbow_common::boot::{BootstrapInit, BootstrapStepTrait};
use crate::setup::boot::CoreBoot;

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
    Start(CoreCliArgs),
    Setup(CoreCliArgs),
}

#[derive(Parser, Debug, PartialEq)]
pub struct CoreCliArgs {
    #[arg(short, long)]
    env_file: Option<String>,
}

pub struct CoreCommands;

impl CoreCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        // parse command line
        debug!("Init the command line application");
        let cli = CoreCli::parse();

        // run scripts
        match cli.role {
            CoreCliRoles::Provider(cmd) => match cmd {
                CoreCliCommands::Start(args) => {
                    let init = BootstrapInit::<CoreBoot>::new(RoleConfig::Provider, args.env_file);
                    let step1 = init.next_step().await?; // Carga Config
                    let step2 = step1.0.next_step().await?; // Config -> Participant
                    let step3 = step2.0.next_step().await?; // Participant -> Catalog
                    let step4 = step3.0.next_step().await?; // Catalog -> DataService
                    let step5 = step4.0.next_step().await?; // -> RUN (Blocking)
                    let _final = step5.0.next_step().await?; // Finalizing log
                }
                CoreCliCommands::Setup(args) => {
                    let config = ApplicationConfig::load(RoleConfig::Provider, args.env_file)?;
                    let table = json_to_table::json_to_table(&serde_json::to_value(&config.monolith())?).collapse().to_string();
                    info!("Current Core Connector Config:\n{}", table);
                    CoreProviderMigration::run(&config).await?;
                    match config.is_mono_catalog_datahub() {
                        true => {}
                        false => {
                            CoreProviderSeeding::run(&config).await?;
                        }
                    }
                }
            },
            CoreCliRoles::Consumer(cmd) => match cmd {
                CoreCliCommands::Start(args) => {
                    let init = BootstrapInit::<CoreBoot>::new(RoleConfig::Consumer, args.env_file);
                    let step1 = init.next_step().await?; // Carga Config
                    let step2 = step1.0.next_step().await?; // Config -> Participant
                    let step3 = step2.0.next_step().await?; // Participant -> Catalog
                    let step4 = step3.0.next_step().await?; // Catalog -> DataService
                    let step5 = step4.0.next_step().await?; // -> RUN (Blocking)
                    let _final = step5.0.next_step().await?; // Finalizing log
                }
                CoreCliCommands::Setup(args) => {
                    let config = ApplicationConfig::load(RoleConfig::Consumer, args.env_file)?;
                    let table = json_to_table::json_to_table(&serde_json::to_value(&config.monolith())?).collapse().to_string();
                    info!("Current Core Connector Config:\n{}", table);
                    CoreConsumerMigration::run(&config).await?
                }
            },
        };

        Ok(())
    }
}
