/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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

use crate::consumer::http::server::create_consumer_router;
use crate::provider::http::server::create_provider_router;
use axum::serve;
use rainbow_common::config::config::GLOBAL_CONFIG;
use tokio::net::TcpListener;
use tracing::info;

pub async fn start_provider_server() -> anyhow::Result<()> {
    // config stuff
    let config = GLOBAL_CONFIG.get().unwrap();
    let server_message = format!(
        "Starting provider server in http://{}:{}",
        config.host_url, config.host_port
    );
    info!("{}", server_message);
    let url = config.host_url.clone().replace("http://", "");

    // start server
    let listener = TcpListener::bind(format!("{}:{}", url, config.host_port)).await?;
    serve(listener, create_provider_router().await).await?;

    Ok(())
}

pub async fn start_consumer_server() -> anyhow::Result<()> {
    // config stuff
    let config = GLOBAL_CONFIG.get().unwrap();
    let server_message = format!(
        "Starting consumer server in http://{}:{}",
        config.host_url, config.host_port
    );
    info!("{}", server_message);
    let url = config.host_url.clone().replace("http://", "");

    // start server
    let listener = TcpListener::bind(format!("{}:{}", config.host_url, config.host_port)).await?;
    serve(listener, create_consumer_router().await).await?;

    Ok(())
}
