use anyhow::bail;
use rainbow_catalog::core::ll_api::dataservices_request_by_id;
use uuid::Uuid;

pub async fn get_dataservice_url_by_id(dataservice_id: Uuid) -> anyhow::Result<String> {
    let dataservice = dataservices_request_by_id(dataservice_id).await?;
    match dataservice {
        Some(ds) => Ok(ds.dcat.endpoint_url),
        None => bail!("No Dataservice found"),
    }
}