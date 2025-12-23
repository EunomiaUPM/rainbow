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
use crate::setup::CoreHttpWorker;
use rainbow_auth::mates;
use rainbow_catalog_agent::{CatalogDto, DataServiceDto, NewCatalogDto, NewDataServiceDto};
use rainbow_common::boot::BootstrapServiceTrait;
use rainbow_common::config::traits::{ApiConfigTrait, HostConfigTrait};
use rainbow_common::config::types::roles::RoleConfig;
use rainbow_common::config::types::HostType;
use rainbow_common::config::ApplicationConfig;
use rainbow_common::http_client::{HttpClient, HttpClientError};
use std::str::FromStr;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;
use urn::Urn;

pub struct CoreBoot;

#[async_trait::async_trait]
impl BootstrapServiceTrait for CoreBoot {
    type Config = ApplicationConfig;
    async fn load_config(role_config: RoleConfig, env_file: Option<String>) -> anyhow::Result<Self::Config> {
        let config = Self::Config::load(role_config, env_file)?;
        let table = json_to_table::json_to_table(&serde_json::to_value(&config.monolith())?).collapse().to_string();
        tracing::info!("Current Monolith Dataspace Agent Config:\n{}", table);
        Ok(config)
    }

    async fn create_participant(config: &Self::Config) -> anyhow::Result<String> {
        let client = HttpClient::new(1, 30);
        let base_url = config.ssi_auth().get_host(HostType::Http);
        let api = config.ssi_auth().get_api_version();

        // attempt first
        let url = format!("{}{}/mates/myself", base_url, api);
        let participant = client.get_json::<mates::Model>(url.as_str()).await;

        // catch error
        if let Err(err) = participant {
            match err {
                // if mate not found
                HttpClientError::HttpError { status, .. } if status.as_u16() == 404 => {
                    // onboard mate with wallet
                    let url = format!("{}{}/wallet/onboard", base_url, api);
                    client.post_void::<()>(url.as_str()).await?;
                }
                _ => anyhow::bail!(err),
            }
            // attempt again
            let url = format!("{}{}/mates/myself", base_url, api);
            let participant = client.get_json::<mates::Model>(url.as_str()).await?;
            // and return id
            Ok(participant.participant_id)
        } else {
            // if mate exists, just return id
            let participant = participant?;
            Ok(participant.participant_id)
        }
    }

    async fn load_catalog(participant_id: &Option<String>, config: &Self::Config) -> anyhow::Result<String> {
        let participant_id = participant_id.clone().unwrap_or_default();
        let client = HttpClient::new(1, 3);
        let base_url = config.catalog().get_host(HostType::Http);
        let api = config.catalog().get_api_version();
        let url = format!("{}{}/catalog-agent/catalogs/main", base_url, api);
        let catalog = client
            .post_json::<NewCatalogDto, CatalogDto>(
                url.as_str(),
                &NewCatalogDto { dspace_participant_id: Some(participant_id), ..NewCatalogDto::default() },
            )
            .await?;
        Ok(catalog.inner.id)
    }

    async fn load_dataservice(catalog_id: &Option<String>, config: &Self::Config) -> anyhow::Result<String> {
        let catalog_id = catalog_id.clone().unwrap_or_default();
        let client = HttpClient::new(1, 3);
        let base_url = config.catalog().get_host(HostType::Http);
        let negotiation_url = config.contracts().get_host(HostType::Http);

        let api = config.catalog().get_api_version();
        let url = format!("{}{}/catalog-agent/data-services/main", base_url, api);
        let catalog = client
            .post_json::<NewDataServiceDto, DataServiceDto>(
                url.as_str(),
                &NewDataServiceDto {
                    dcat_endpoint_url: format!("{}/dsp/current", negotiation_url),
                    catalog_id: Urn::from_str(catalog_id.as_str())?,
                    ..Default::default()
                },
            )
            .await?;
        Ok(catalog.inner.id)
    }

    async fn start_services_background(config: &Self::Config) -> anyhow::Result<Sender<()>> {
        // thread control
        let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);
        let cancel_token = CancellationToken::new();

        // workers
        tracing::info!("Spawning HTTP subsystem...");
        let http_handle = CoreHttpWorker::spawn(config, &cancel_token).await?;

        // todo set grpc

        // non-blocking thread
        let token_clone = cancel_token.clone();
        tokio::spawn(async move {
            tokio::select! {
                // ctrl+c
                _ = shutdown_rx.recv() => {
                    tracing::info!("Shutdown command received from Main Pipeline.");
                }
                _ = async { http_handle.await } => {
                    tracing::error!("HTTP subsystem failed or stopped unexpectedly!");
                }
            }

            tracing::info!("Initiating internal graceful shutdown sequence...");
            token_clone.cancel();
            tracing::info!("Background services stopped.");
        });

        Ok(shutdown_tx)
    }
}
