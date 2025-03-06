use crate::core::DataPlanePeerDefaultBehavior;
use crate::data_plane_peer::DataPlanePeer;
use crate::facade::DataPlaneFacade;
use crate::implementations::ngsi_ld::NgsiLdDataPlane;
use crate::implementations::plain_http::HttpDataPlane;
use rainbow_common::config::config::ConfigRoles;
use rainbow_common::config::database::get_db_connection;
use rainbow_common::dcat_formats::{FormatAction, FormatProtocol};
use rainbow_common::protocol::transfer::TransferRequestMessage;
use tracing::info;
use urn::Urn;

pub struct DataPlaneFacadeImpl {}

impl DataPlaneFacadeImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[axum::async_trait]
impl DataPlaneFacade for DataPlaneFacadeImpl {
    async fn bootstrap_data_plane_in_consumer(
        &self,
        transfer_request: TransferRequestMessage,
    ) -> anyhow::Result<DataPlanePeer> {
        info!("Bootstraping consumer data plane");
        match transfer_request.format.protocol {
            FormatProtocol::NgsiLd => Ok(NgsiLdDataPlane::bootstrap_data_plane_in_consumer(transfer_request).await?),
            FormatProtocol::Http => Ok(HttpDataPlane::bootstrap_data_plane_in_consumer(transfer_request).await?),
            _ => {
                todo!("Not implemented yet...")
            }
        }
    }

    async fn bootstrap_data_plane_in_provider(
        &self,
        transfer_request: TransferRequestMessage,
        provider_pid: Urn,
    ) -> anyhow::Result<DataPlanePeer> {
        info!("Bootstraping provider data plane");

        match transfer_request.format.protocol {
            FormatProtocol::NgsiLd => {
                Ok(NgsiLdDataPlane::bootstrap_data_plane_in_provider(transfer_request, provider_pid).await?)
            }
            FormatProtocol::Http => {
                Ok(HttpDataPlane::bootstrap_data_plane_in_provider(transfer_request, provider_pid).await?)
            }
            _ => {
                todo!("Not implemented yet...")
            }
        }
    }

    // async fn get_data_plane_peer(&self, data_plane_id: Urn) -> anyhow::Result<DataPlanePeer> {
    //     let data_plane_peer = DataPlanePeer::load_model_by_id(data_plane_id).await;
    //     match data_plane_peer {
    //         Ok(dp) => Ok(*dp),
    //         Err(_) => bail!("Failed to load data plane. Data plane not found."),
    //     }
    // }

    async fn set_data_plane_next_hop(
        &self,
        data_plane_peer: DataPlanePeer,
        provider_pid: Urn,
        consumer_pid: Urn,
    ) -> anyhow::Result<DataPlanePeer> {
        info!("Setting next hop");

        match data_plane_peer.dct_formats.protocol {
            FormatProtocol::NgsiLd => {
                Ok(NgsiLdDataPlane::set_data_plane_next_hop(data_plane_peer, provider_pid, consumer_pid).await?)
            }
            FormatProtocol::Http => {
                Ok(HttpDataPlane::set_data_plane_next_hop(data_plane_peer, provider_pid, consumer_pid).await?)
            }
            _ => {
                todo!("Not implemented yet...")
            }
        }
    }

    async fn connect_to_streaming_service(&self, data_plane_id: Urn) -> anyhow::Result<()> {
        info!("Setup Connection to streaming service");
        let db_connection = get_db_connection().await;
        let data_plane_peer = DataPlanePeer::load_model_by_id(data_plane_id.clone()).await?;

        // Prevent connections on Consumer or Pull cases
        match data_plane_peer.role {
            ConfigRoles::Provider => {}
            _ => {
                return Ok(());
            }
        }
        match data_plane_peer.dct_formats.action {
            FormatAction::Push => {}
            FormatAction::Pull => {
                return Ok(());
            }
        }

        match data_plane_peer.dct_formats.protocol {
            FormatProtocol::NgsiLd => Ok(NgsiLdDataPlane::connect_to_streaming_service(data_plane_id).await?),
            FormatProtocol::Http => Ok(()),
            _ => {
                todo!("Not implemented yet...")
            }
        }
    }

    async fn disconnect_from_streaming_service(&self, data_plane_id: Urn) -> anyhow::Result<()> {
        info!("Disconnecting from streaming service");
        let db_connection = get_db_connection().await;
        let peer = DataPlanePeer::load_model_by_id(data_plane_id.clone()).await?;

        // Prevent disconnection on Consumer or Pull cases
        match peer.role {
            ConfigRoles::Provider => {}
            _ => {
                return Ok(());
            }
        }
        match peer.dct_formats.action {
            FormatAction::Push => {}
            FormatAction::Pull => {
                return Ok(());
            }
        }
        match peer.dct_formats.protocol {
            FormatProtocol::NgsiLd => Ok(NgsiLdDataPlane::disconnect_from_streaming_service(data_plane_id).await?),
            FormatProtocol::Http => Ok(()),
            _ => {
                todo!("Not implemented yet...")
            }
        }
    }
}
