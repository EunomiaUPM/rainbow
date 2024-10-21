use crate::fake_catalog::lib::get_dataset_by_id;
use crate::fake_contracts::lib::get_agreement_by_id;
use crate::setup::config::get_provider_url;
use crate::transfer::protocol::messages::{
    DataAddress, TransferRequestMessage, TransferState, TransferSuspensionMessage,
};
use crate::transfer::provider::data::repo::{TransferProviderDataRepo, TRANSFER_PROVIDER_REPO};
use anyhow::Error;
use std::future::Future;
use uuid::Uuid;

pub async fn resolve_endpoint_from_agreement(agreement_id: Uuid) -> anyhow::Result<String> {
    // Resolve endpoint
    let agreement = get_agreement_by_id(agreement_id).unwrap();
    if agreement.is_none() {
        // TODO create error
        return Err(anyhow::anyhow!("agreement not found"));
    }

    let dataset_id = agreement.unwrap().dataset_id;
    let dataset = get_dataset_by_id(dataset_id)?;
    if dataset.is_none() {
        // TODO create error
        return Err(anyhow::anyhow!("dataset not found"));
    }
    let endpoint = dataset.unwrap().dataset_endpoint;
    Ok(endpoint)
}

pub async fn data_plane_start<F, Fut, M>(
    mut input: TransferRequestMessage,
    provider_pid: Uuid,
    cb: F,
) -> anyhow::Result<()>
where
    F: Fn(M, Uuid) -> Fut + Send + Sync + 'static,
    Fut: Future<Output=Result<(), Error>> + Send,
    M: From<TransferRequestMessage> + Send + 'static,
{
    let data_plane_id = Uuid::new_v4();
    let endpoint = format!("http://{}/data/{}", get_provider_url()?, data_plane_id.to_string());

    // persist
    TRANSFER_PROVIDER_REPO.update_transfer_process_by_provider_pid(
        &provider_pid,
        TransferState::STARTED,
        Some(data_plane_id),
    )?;

    // return data address
    input.data_address = Some(DataAddress {
        _type: "dspace:DataAddress".to_string(),
        endpoint_type: "HTTP".to_string(), // TODO hacer esto (desde agreements ODRL y DCAT)
        endpoint,
        endpoint_properties: vec![], // TODO hacer esto (desde agreements ODRL y DCAT)
    });

    cb(input.into(), provider_pid).await?;
    Ok(())
}

pub async fn suspend_data_plane(
    input: TransferSuspensionMessage,
    provider_pid: Uuid,
) -> anyhow::Result<()> {
    Ok(())
}
