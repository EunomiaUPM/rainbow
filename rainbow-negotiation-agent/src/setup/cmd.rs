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

use crate::setup::boot::NegotiationAgentBoot;
use crate::setup::db_migrations::NegotiationAgentMigration;
use clap::{Parser, Subcommand};
use rainbow_common::boot::{BootstrapInit, BootstrapStepTrait};
use rainbow_common::config::services::ContractsConfig;
use rainbow_common::config::traits::ConfigLoader;
use rainbow_common::config::types::roles::RoleConfig;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "Rainbow Dataspace Connector Negotiation Agent")]
#[command(version = "0.2")]
struct NegotiationCli {
    #[clap(subcommand)]
    command: NegotiationCliCommands,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum NegotiationCliCommands {
    Start(NegotiationCliArgs),
    Setup(NegotiationCliArgs),
}

#[derive(Parser, Debug, PartialEq)]
pub struct NegotiationCliArgs {
    #[arg(short, long)]
    env_file: Option<String>,
}

pub struct NegotiationCommands {}

impl NegotiationCommands {
    pub async fn init_command_line() -> anyhow::Result<()> {
        debug!("init_command_line - Initialize negotiation commands");
        let cli = NegotiationCli::parse();
        match cli.command {
            NegotiationCliCommands::Start(args) => {
                let init = BootstrapInit::<NegotiationAgentBoot>::new(RoleConfig::NotDefined, args.env_file);
                let step1 = init.next_step().await?; // Carga Config
                let step2 = step1.0.next_step().await?; // Config -> Participant
                let step3 = step2.0.next_step().await?; // Participant -> Catalog
                let step4 = step3.0.next_step().await?; // Catalog -> DataService
                let step5 = step4.0.next_step().await?; // -> RUN (Blocking)
                let _final = step5.0.next_step().await?; // Finalizing log
            }
            NegotiationCliCommands::Setup(args) => {
                let config = ContractsConfig::load(RoleConfig::NotDefined, args.env_file);
                let table = json_to_table::json_to_table(&serde_json::to_value(&config)?).collapse().to_string();
                info!("Current Negotiations Agent Config:\n{}", table);
                NegotiationAgentMigration::run(&config).await?;
            }
        }
        Ok(())
    }
}
