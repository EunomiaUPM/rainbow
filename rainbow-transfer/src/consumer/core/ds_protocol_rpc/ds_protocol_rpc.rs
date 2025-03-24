use crate::consumer::core::data_plane_facade::DataPlaneConsumerFacadeTrait;
use crate::consumer::core::ds_protocol::ds_protocol_err::DSProtocolTransferConsumerErrors;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_err::DSRPCTransferConsumerErrors;
use crate::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::{
    DSRPCTransferConsumerCompletionRequest, DSRPCTransferConsumerCompletionResponse,
    DSRPCTransferConsumerRequestRequest, DSRPCTransferConsumerRequestResponse, DSRPCTransferConsumerStartRequest,
    DSRPCTransferConsumerStartResponse, DSRPCTransferConsumerSuspensionRequest,
    DSRPCTransferConsumerSuspensionResponse, DSRPCTransferConsumerTerminationRequest,
    DSRPCTransferConsumerTerminationResponse,
};
use crate::consumer::core::ds_protocol_rpc::DSRPCTransferConsumerTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::transfer::transfer_process::TransferProcessMessage;
use rainbow_common::protocol::transfer::transfer_request::TransferRequestMessage;
use rainbow_common::protocol::transfer::transfer_start::TransferStartMessage;
use rainbow_common::protocol::transfer::transfer_suspension::TransferSuspensionMessage;
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_db::transfer_consumer::repo::{EditTransferCallback, NewTransferCallback, TransferConsumerRepoFactory};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;

pub struct DSRPCTransferConsumerService<T, U>
where
    T: TransferConsumerRepoFactory + Send + Sync,
    U: DataPlaneConsumerFacadeTrait + Send + Sync,
{
    transfer_repo: Arc<T>,
    data_plane_facade: Arc<U>,
    client: Client,
}

impl<T, U> DSRPCTransferConsumerService<T, U>
where
    T: TransferConsumerRepoFactory + Send + Sync,
    U: DataPlaneConsumerFacadeTrait + Send + Sync,
{
    pub fn new(transfer_repo: Arc<T>, data_plane_facade: Arc<U>) -> Self {
        let client =
            Client::builder().timeout(Duration::from_secs(10)).build().expect("Failed to build reqwest client");
        Self { transfer_repo, data_plane_facade, client }
    }
}

#[async_trait]
impl<T, U> DSRPCTransferConsumerTrait for DSRPCTransferConsumerService<T, U>
where
    T: TransferConsumerRepoFactory + Send + Sync,
    U: DataPlaneConsumerFacadeTrait + Send + Sync,
{
    async fn setup_request(
        &self,
        input: DSRPCTransferConsumerRequestRequest,
    ) -> anyhow::Result<DSRPCTransferConsumerRequestResponse> {
        let DSRPCTransferConsumerRequestRequest { agreement_id, format, data_address, provider_address, .. } = input;
        let consumer_pid = get_urn(None);
        // create message
        let transfer_request = TransferRequestMessage {
            consumer_pid: consumer_pid.to_string(),
            agreement_id: agreement_id.clone(),
            format: format.clone(),
            data_address: data_address.clone(),
            ..Default::default()
        };

        // communicate to provider
        let provider_base_url = provider_address.strip_suffix('/').unwrap_or(provider_address.as_str());
        let provider_address = format!("{}/transfers/request", provider_base_url);
        let req = self
            .client
            .post(provider_address)
            .json(&transfer_request)
            .send()
            .await
            .map_err(|_e| DSRPCTransferConsumerErrors::ProviderNotReachable {
                provider_pid: None,
                consumer_pid: None,
            })?;
        if !req.status().is_success() {
            bail!(DSRPCTransferConsumerErrors::ProviderInternalError {
                provider_pid: None,
                consumer_pid: None,
                error: Some(req.json().await?)
            });
        }
        // create response
        let response = req.json::<TransferProcessMessage>().await
            .map_err(|_e| DSRPCTransferConsumerErrors::ProviderResponseNotSerializable {
                provider_pid: None,
                consumer_pid: None,
            })?;

        // persist in db
        let transfer_process = self.transfer_repo
            .create_transfer_callback(NewTransferCallback {
                consumer_pid: Some(get_urn_from_string(&response.consumer_pid)?),
                provider_pid: Some(get_urn_from_string(&response.provider_pid)?),
                data_address: None,
            })
            .await
            .map_err(|e| DSRPCTransferConsumerErrors::DSProtocolTransferConsumerError(DSProtocolTransferConsumerErrors::DbErr(e)))?;
        // data plane
        self.data_plane_facade.on_transfer_request().await?;
        // create response
        let provider_pid = Some(get_urn_from_string(&transfer_process.provider_pid.unwrap())?);
        let response = DSRPCTransferConsumerRequestResponse {
            provider_pid: provider_pid.unwrap(),
            consumer_pid,
            agreement_id,
            format,
            data_address,
            message: response,
        };
        Ok(response)
    }

    async fn setup_start(
        &self,
        input: DSRPCTransferConsumerStartRequest,
    ) -> anyhow::Result<DSRPCTransferConsumerStartResponse> {
        let DSRPCTransferConsumerStartRequest { data_address, provider_address, provider_pid, consumer_pid, .. } = input;
        // create message
        let transfer_request = TransferStartMessage {
            consumer_pid: consumer_pid.to_string(),
            provider_pid: provider_pid.to_string(),
            data_address: data_address.clone(),
            ..Default::default()
        };

        // communicate to provider
        let provider_base_url = provider_address.strip_suffix('/').unwrap_or(provider_address.as_str());
        let provider_address = format!("{}/transfers/{}/start", provider_base_url, provider_pid.to_string());
        let req = self
            .client
            .post(provider_address)
            .json(&transfer_request)
            .send()
            .await
            .map_err(|_e| DSRPCTransferConsumerErrors::ProviderNotReachable {
                provider_pid: None,
                consumer_pid: None,
            })?;
        if !req.status().is_success() {
            bail!(DSRPCTransferConsumerErrors::ProviderInternalError {
                provider_pid: None,
                consumer_pid: None,
                error: Some(req.json().await?)
            });
        }
        // create response
        let response = req.json::<TransferProcessMessage>().await
            .map_err(|_e| DSRPCTransferConsumerErrors::ProviderResponseNotSerializable {
                provider_pid: None,
                consumer_pid: None,
            })?;

        // persist in db
        let _transfer_process = self.transfer_repo
            .put_transfer_callback_by_consumer(consumer_pid.clone(), EditTransferCallback {
                consumer_pid: None,
                provider_pid: None,
                data_plane_id: None,
                data_address: None,
            })
            .await
            .map_err(|e| DSRPCTransferConsumerErrors::DSProtocolTransferConsumerError(DSProtocolTransferConsumerErrors::DbErr(e)))?;
        // data plane
        self.data_plane_facade.on_transfer_start().await?;
        // create response
        let response = DSRPCTransferConsumerStartResponse {
            provider_pid,
            consumer_pid,
            data_address,
            message: response,
        };
        Ok(response)
    }

    async fn setup_suspension(
        &self,
        input: DSRPCTransferConsumerSuspensionRequest,
    ) -> anyhow::Result<DSRPCTransferConsumerSuspensionResponse> {
        let DSRPCTransferConsumerSuspensionRequest { provider_address, provider_pid, consumer_pid, code, reason } = input;
        // create message
        let transfer_request = TransferSuspensionMessage {
            consumer_pid: consumer_pid.to_string(),
            provider_pid: provider_pid.to_string(),
            code,
            reason,
            ..Default::default()
        };

        // communicate to provider
        let provider_base_url = provider_address.strip_suffix('/').unwrap_or(provider_address.as_str());
        let provider_address = format!("{}/transfers/{}/suspension", provider_base_url, provider_pid.to_string());
        let req = self
            .client
            .post(provider_address)
            .json(&transfer_request)
            .send()
            .await
            .map_err(|_e| DSRPCTransferConsumerErrors::ProviderNotReachable {
                provider_pid: None,
                consumer_pid: None,
            })?;
        if !req.status().is_success() {
            bail!(DSRPCTransferConsumerErrors::ProviderInternalError {
                provider_pid: None,
                consumer_pid: None,
                error: Some(req.json().await?)
            });
        }
        // create response
        let response = req.json::<TransferProcessMessage>().await
            .map_err(|_e| DSRPCTransferConsumerErrors::ProviderResponseNotSerializable {
                provider_pid: None,
                consumer_pid: None,
            })?;

        // persist in db
        let _transfer_process = self.transfer_repo
            .put_transfer_callback_by_consumer(consumer_pid.clone(), EditTransferCallback {
                consumer_pid: None,
                provider_pid: None,
                data_plane_id: None,
                data_address: None,
            })
            .await
            .map_err(|e| DSRPCTransferConsumerErrors::DSProtocolTransferConsumerError(DSProtocolTransferConsumerErrors::DbErr(e)))?;
        // data plane
        self.data_plane_facade.on_transfer_start().await?;
        // create response
        let response = DSRPCTransferConsumerSuspensionResponse {
            provider_pid,
            consumer_pid,
            message: response,
        };
        Ok(response)
    }

    async fn setup_completion(
        &self,
        input: DSRPCTransferConsumerCompletionRequest,
    ) -> anyhow::Result<DSRPCTransferConsumerCompletionResponse> {
        let DSRPCTransferConsumerCompletionRequest { provider_address, provider_pid, consumer_pid, .. } = input;
        // create message
        let transfer_request = TransferStartMessage {
            consumer_pid: consumer_pid.to_string(),
            provider_pid: provider_pid.to_string(),
            ..Default::default()
        };

        // communicate to provider
        let provider_base_url = provider_address.strip_suffix('/').unwrap_or(provider_address.as_str());
        let provider_address = format!("{}/transfers/{}/completion", provider_base_url, provider_pid.to_string());
        let req = self
            .client
            .post(provider_address)
            .json(&transfer_request)
            .send()
            .await
            .map_err(|_e| DSRPCTransferConsumerErrors::ProviderNotReachable {
                provider_pid: None,
                consumer_pid: None,
            })?;
        if !req.status().is_success() {
            bail!(DSRPCTransferConsumerErrors::ProviderInternalError {
                provider_pid: None,
                consumer_pid: None,
                error: Some(req.json().await?)
            });
        }
        // create response
        let response = req.json::<TransferProcessMessage>().await
            .map_err(|_e| DSRPCTransferConsumerErrors::ProviderResponseNotSerializable {
                provider_pid: None,
                consumer_pid: None,
            })?;

        // persist in db
        let _transfer_process = self.transfer_repo
            .put_transfer_callback_by_consumer(consumer_pid.clone(), EditTransferCallback {
                consumer_pid: None,
                provider_pid: None,
                data_plane_id: None,
                data_address: None,
            })
            .await
            .map_err(|e| DSRPCTransferConsumerErrors::DSProtocolTransferConsumerError(DSProtocolTransferConsumerErrors::DbErr(e)))?;
        // data plane
        self.data_plane_facade.on_transfer_start().await?;
        // create response
        let response = DSRPCTransferConsumerCompletionResponse {
            provider_pid,
            consumer_pid,
            message: response,
        };
        Ok(response)
    }

    async fn setup_termination(
        &self,
        input: DSRPCTransferConsumerTerminationRequest,
    ) -> anyhow::Result<DSRPCTransferConsumerTerminationResponse> {
        let DSRPCTransferConsumerTerminationRequest { provider_address, provider_pid, consumer_pid, code, reason } = input;
        // create message
        let transfer_request = TransferSuspensionMessage {
            consumer_pid: consumer_pid.to_string(),
            provider_pid: provider_pid.to_string(),
            code,
            reason,
            ..Default::default()
        };

        // communicate to provider
        let provider_base_url = provider_address.strip_suffix('/').unwrap_or(provider_address.as_str());
        let provider_address = format!("{}/transfers/{}/termination", provider_base_url, provider_pid.to_string());
        let req = self
            .client
            .post(provider_address)
            .json(&transfer_request)
            .send()
            .await
            .map_err(|_e| DSRPCTransferConsumerErrors::ProviderNotReachable {
                provider_pid: None,
                consumer_pid: None,
            })?;
        if !req.status().is_success() {
            bail!(DSRPCTransferConsumerErrors::ProviderInternalError {
                provider_pid: None,
                consumer_pid: None,
                error: Some(req.json().await?)
            });
        }
        // create response
        let response = req.json::<TransferProcessMessage>().await
            .map_err(|_e| DSRPCTransferConsumerErrors::ProviderResponseNotSerializable {
                provider_pid: None,
                consumer_pid: None,
            })?;

        // persist in db
        let _transfer_process = self.transfer_repo
            .put_transfer_callback_by_consumer(consumer_pid.clone(), EditTransferCallback {
                consumer_pid: None,
                provider_pid: None,
                data_plane_id: None,
                data_address: None,
            })
            .await
            .map_err(|e| DSRPCTransferConsumerErrors::DSProtocolTransferConsumerError(DSProtocolTransferConsumerErrors::DbErr(e)))?;
        // data plane
        self.data_plane_facade.on_transfer_start().await?;
        // create response
        let response = DSRPCTransferConsumerTerminationResponse {
            provider_pid,
            consumer_pid,
            message: response,
        };
        Ok(response)
    }
}
