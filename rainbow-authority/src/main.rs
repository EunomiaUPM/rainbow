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

use rainbow_authority::setup::commands::cmd::AuthorityCommands;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

const INFO: &str = r"
----------
 ____    __    ____  _  _  ____  _____  _    _
(  _ \  /__\  (_  _)( \( )(  _ \(  _  )( \/\/ )
 )   / /(__)\  _)(_  )  (  ) _ < )(_)(  )    (
(_)\_)(__)(__)(____)(_)\_)(____/(_____)(__/\__)

Starting Rainbow Authority Server 🌈🌈
UPM Dataspace protocol implementation
Show some love on https://github.com/ging/rainbow
----------

";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter =
        EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).parse("debug,sqlx::query=off")?;
    tracing_subscriber::fmt()
        .event_format(tracing_subscriber::fmt::format().with_line_number(true))
        .with_env_filter(filter)
        .init();
    info!("{}", INFO);
    AuthorityCommands::init_command_line().await?;
    Ok(())
}
