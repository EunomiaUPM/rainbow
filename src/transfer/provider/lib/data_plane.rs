use crate::fake_catalog::lib::get_dataset_by_id;
use crate::fake_contracts::lib::get_agreement_by_id;
use crate::transfer::protocol::messages::{
    DataAddress, TransferRequestMessage, TransferSuspensionMessage,
};
use crate::transfer::provider::data::models::DataPlaneProcessModel;
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
    let dataset = get_dataset_by_id(dataset_id).unwrap();
    if dataset.is_none() {
        // TODO create error
        return Err(anyhow::anyhow!("dataset not found"));
    }
    let endpoint = dataset.unwrap().dataset_endpoint;
    Ok(endpoint)
}

pub async fn provision_data_plane<F, Fut, M>(
    mut input: TransferRequestMessage,
    provider_pid: Uuid,
    cb: F,
) -> anyhow::Result<()>
where
    F: Fn(M, Uuid) -> Fut + Send + Sync + 'static,
    Fut: Future<Output=Result<(), Error>> + Send,
    M: From<TransferRequestMessage> + Send + 'static,
{
    let agreement = Uuid::parse_str(&input.agreement_id)?;

    // Stuff should happen here....
    // TODO define what... related with PXP
    //

    // persist
    let data_plane_process =
        TRANSFER_PROVIDER_REPO.create_data_plane_process(DataPlaneProcessModel {
            data_plane_id: Uuid::new_v4(),
            transfer_process_id: provider_pid.to_owned(),
            agreement_id: agreement,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: None,
            state: true,
        })?;

    let endpoint = format!(
        "http://localhost:1234/data/{}",
        data_plane_process.data_plane_id.to_string()
    );

    input.data_address = Some(DataAddress {
        _type: "dspace:DataAddress".to_string(),
        endpoint_type: "HTTP".to_string(), // TODO hacer esto (desde agreements ODRL y DCAT)
        endpoint,
        endpoint_properties: vec![], // TODO hacer esto (desde agreements ODRL y DCAT)
    });

    cb(input.into(), provider_pid).await?;
    Ok(())
}

pub async fn unprovision_data_plane<F, Fut, M>(
    input: TransferSuspensionMessage,
    provider_pid: Uuid,
    cb: F,
) -> anyhow::Result<()>
where
    F: Fn(M, Uuid) -> Fut + Send + Sync + 'static,
    Fut: Future<Output=Result<(), Error>> + Send,
    M: From<TransferSuspensionMessage> + Send + 'static,
{
    // Stuff should happen here....
    // TODO define what... related with PXP
    //

    let data_plane =
        TRANSFER_PROVIDER_REPO.get_data_plane_process_by_transfer_process_id(
            provider_pid,
        )?;
    if let None = data_plane {
        return Err(anyhow::anyhow!("not found...")); // TODO error
    } else {
        TRANSFER_PROVIDER_REPO.update_data_plane_process(
            data_plane.unwrap().data_plane_id,
            false,
        )?;
        cb(input.into(), provider_pid).await?;
        Ok(())
    }
}
