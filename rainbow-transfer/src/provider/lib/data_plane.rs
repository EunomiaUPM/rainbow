use crate::fake_catalog::lib::get_dataset_by_id;
use crate::fake_contracts::lib::get_agreement_by_id;
use crate::protocol::messages::{
    DataAddress, TransferCompletionMessage, TransferRequestMessage, TransferSuspensionMessage,
    TransferTerminationMessage,
};
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
    F: Fn(M, Uuid, Uuid) -> Fut + Send + Sync + 'static,
    Fut: Future<Output=Result<(), Error>> + Send,
    M: From<TransferRequestMessage> + Send + 'static,
{
    let data_plane_id = Uuid::new_v4();
    // let endpoint = format!("http://{}/data/{}", get_provider_url()?, data_plane_id.to_string());
    let endpoint = format!("http://localhost:1234/data/{}", data_plane_id.to_string());

    // return data address
    input.data_address = Some(DataAddress {
        _type: "dspace:DataAddress".to_string(),
        endpoint_type: "HTTP".to_string(), // TODO hacer esto (desde agreements ODRL y DCAT)
        endpoint,
        endpoint_properties: vec![], // TODO hacer esto (desde agreements ODRL y DCAT)
    });

    cb(input.into(), provider_pid, data_plane_id).await?;
    Ok(())
}

pub async fn connect_to_streaming_service(
    input: &TransferRequestMessage,
    provider_pid: Uuid,
) -> anyhow::Result<()> {
    println!("{:?}", input);
    Ok(())
}

pub async fn disconnect_from_streaming_service_on_suspension(
    input: TransferSuspensionMessage,
    provider_pid: Uuid,
) -> anyhow::Result<()> {
    Ok(())
}

pub async fn disconnect_from_streaming_service_on_completion(
    input: TransferCompletionMessage,
    provider_pid: Uuid,
) -> anyhow::Result<()> {
    Ok(())
}

pub async fn disconnect_from_streaming_service_on_termination(
    input: TransferTerminationMessage,
    provider_pid: Uuid,
) -> anyhow::Result<()> {
    Ok(())
}
