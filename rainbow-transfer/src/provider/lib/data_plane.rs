use rainbow_catalog::core::ll_api::dataservices_request_by_id;
use rainbow_catalog::protocol::dataservice_definition::DataService;
use rainbow_common::config::database::get_db_connection;
use rainbow_db::transfer_provider::entities::agreements;
use sea_orm::EntityTrait;
use uuid::Uuid;

pub async fn resolve_endpoint_from_agreement(agreement_id: Uuid) -> anyhow::Result<DataService> {
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::find_by_id(agreement_id).one(db_connection).await?;
    if agreement.is_none() {
        return Err(anyhow::anyhow!("agreement not found"));
    }
    let agreement = agreement.unwrap();

    // TODO if is all in modules, change function
    let data_service_id = agreement.data_service_id;
    let data_service = dataservices_request_by_id(data_service_id).await?;
    if data_service.is_none() {
        return Err(anyhow::anyhow!("data service not found"));
    }

    Ok(data_service.unwrap())
}
