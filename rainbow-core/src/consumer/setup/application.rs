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

use crate::consumer::http::router::create_core_consumer_router;
use axum::serve;
use rainbow_common::config::consumer_config::{ApplicationConsumerConfig, ApplicationConsumerConfigTrait};
use tokio::net::TcpListener;
use tracing::info;

pub struct CoreConsumerApplication;

impl CoreConsumerApplication {
    pub async fn run(config: &ApplicationConsumerConfig) -> anyhow::Result<()> {
        // db_connection
        let router = create_core_consumer_router(config).await;
        // Init server
        let server_message = format!(
            "Starting core consumer server in {}",
            config.get_transfer_host_url().unwrap(),
        );
        info!("{}", server_message);
        let listener = TcpListener::bind(format!(
            "{}:{}",
            config.get_raw_transfer_process_host().clone().unwrap().url,
            config.get_raw_transfer_process_host().clone().unwrap().port
        )).await?;
        serve(listener, router).await?;
        Ok(())
    }
}
