use crate::protocol::messages::{
    DataAddress, TransferCompletionMessage, TransferRequestMessage, TransferSuspensionMessage,
    TransferTerminationMessage,
};
use crate::provider::data::entities::agreements;
// use crate::setup::config::get_provider_url;
use rainbow_common::config::database::get_db_connection;
use anyhow::Error;
use rainbow_common::config::config::get_provider_url;
// use rainbow_common::transfer_comm::get_dataservice_url_by_id;
use sea_orm::EntityTrait;
use std::future::Future;
use uuid::Uuid;
use rainbow_catalog::core::ll_api::dataservices_request_by_id;

pub async fn resolve_endpoint_from_agreement(agreement_id: Uuid) -> anyhow::Result<String> {
    // Resolve endpoint
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::find_by_id(agreement_id)
        .one(db_connection)
        .await?;
    if agreement.is_none() {
        // TODO create error
        return Err(anyhow::anyhow!("agreement not found"));
    }
    let agreement = agreement.unwrap();

    // TODO if is all in modules, change function
    let data_service_id = agreement.data_service_id;
    let data_service = dataservices_request_by_id(data_service_id).await?;
    if data_service.is_none() {
        // TODO create error
        return Err(anyhow::anyhow!("dataset not found"));
    }
    let endpoint = data_service.unwrap().dcat.endpoint_url;
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
    let endpoint = format!("http://{}/data/{}", get_provider_url()?, data_plane_id.to_string());

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
