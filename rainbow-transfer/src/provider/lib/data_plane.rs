use rainbow_catalog::core::ll_api::dataservices_request_by_id;
use rainbow_catalog::protocol::dataservice_definition::DataService;
use rainbow_common::config::database::get_db_connection;
use rainbow_db::transfer_provider::entities::agreements;
use sea_orm::EntityTrait;
use uuid::Uuid;

pub async fn resolve_endpoint_from_agreement(agreement_id: Uuid) -> anyhow::Result<DataService> {
    // Resolve endpoint
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::find_by_id(agreement_id).one(db_connection).await?;
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
        return Err(anyhow::anyhow!("data service not found"));
    }
    // let endpoint = data_service.unwrap().dcat.endpoint_url;
    Ok(data_service.unwrap())
}


// pub async fn reconnect_to_streaming_service_on_start(
//     input: TransferStartMessage,
// ) -> anyhow::Result<()> {
//     let provider_pid = convert_uri_to_uuid(&input.provider_pid)?;
//     let db_connection = get_db_connection().await;
//     let transfer = transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
//     if transfer.is_none() {
//         bail!("provider not found for agreement id");
//     }
//     let transfer = transfer.unwrap();
//
//     // resolve agreement
//     let agreement_id = transfer.agreement_id;
//
//     // resolve data service
//     let data_service = resolve_endpoint_from_agreement(agreement_id).await?;
//     let endpoint_url = data_service.dcat.endpoint_url;
//     let endpoint_description = data_service.dcat.endpoint_description;
//
//     // create payload to cb
//     // here is the crux....
//     let cb_suscription_payload = endpoint_description
//         .replace("$data_url", transfer.data_plane_address.unwrap().as_str())
//         .replace("$data_description", "My description");
//     let cb_suscription_payload =
//         serde_json::from_str::<serde_json::Value>(&cb_suscription_payload)?;
//
//     // suscribe to cb
//     let res =
//         DATA_PLANE_HTTP_CLIENT.post(endpoint_url).json(&cb_suscription_payload).send().await?;
//
//     // persist suscription identifier
//     let suscription_id = res.headers().get("location");
//     if suscription_id.is_none() {
//         // TODO error
//         bail!("not able to connect to streaming service")
//     }
//     let suscription_id = suscription_id.unwrap().to_str()?;
//     let suscription_id = suscription_id.replace("/v2/subscriptions/", "");
//     println!("Suscription: {}", suscription_id);
//
//     let old_tp = transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
//     if old_tp.is_none() {
//         bail!("provider not found for agreement id");
//     }
//     let old_tp = old_tp.unwrap();
//     let tp = transfer_process::Entity::update(transfer_process::ActiveModel {
//         provider_pid: ActiveValue::Set(old_tp.provider_pid),
//         consumer_pid: ActiveValue::Set(old_tp.consumer_pid),
//         agreement_id: ActiveValue::Set(old_tp.agreement_id),
//         data_plane_id: ActiveValue::Set(old_tp.data_plane_id),
//         subscription_id: ActiveValue::Set(Some(suscription_id)),
//         state: ActiveValue::Set(old_tp.state),
//         created_at: ActiveValue::Set(old_tp.created_at),
//         updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
//         data_plane_address: ActiveValue::Set(old_tp.data_plane_address),
//         next_hop_address: ActiveValue::Set(old_tp.next_hop_address),
//     })
//         .exec(db_connection)
//         .await?;
//
//     match res.status() {
//         reqwest::StatusCode::CREATED => {}
//         // TODO error
//         _ => bail!("not able to connect to streaming service"),
//     }
//     Ok(())
// }
//
// pub async fn disconnect_from_streaming_service_on_suspension(
//     input: TransferSuspensionMessage,
// ) -> anyhow::Result<()> {
//     let provider_pid = convert_uri_to_uuid(&input.provider_pid)?;
//     let db_connection = get_db_connection().await;
//     let transfer_process =
//         transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
//     if transfer_process.clone().unwrap().subscription_id.is_none() {
//         bail!("provider has not suscription opened");
//     }
//
//     let data_service =
//         resolve_endpoint_from_agreement(transfer_process.clone().unwrap().agreement_id).await?;
//     let data_service_url = data_service.dcat.endpoint_url;
//     let endpoint_delete_url = format!(
//         "{}/{}",
//         data_service_url,
//         transfer_process.clone().unwrap().subscription_id.unwrap()
//     );
//     let res = DATA_PLANE_HTTP_CLIENT.delete(endpoint_delete_url).send().await?;
//
//     let old_tp = transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
//     if old_tp.is_none() {
//         bail!("provider not found for agreement id");
//     }
//     let old_tp = old_tp.unwrap();
//
//     let tp = transfer_process::Entity::update(transfer_process::ActiveModel {
//         provider_pid: ActiveValue::Set(old_tp.provider_pid),
//         consumer_pid: ActiveValue::Set(old_tp.consumer_pid),
//         agreement_id: ActiveValue::Set(old_tp.agreement_id),
//         data_plane_id: ActiveValue::Set(old_tp.data_plane_id),
//         subscription_id: ActiveValue::Set(None),
//         state: ActiveValue::Set(old_tp.state),
//         created_at: ActiveValue::Set(old_tp.created_at),
//         updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
//         data_plane_address: ActiveValue::Set(old_tp.data_plane_address),
//         next_hop_address: ActiveValue::Set(old_tp.next_hop_address),
//     })
//         .exec(db_connection)
//         .await?;
//
//     Ok(())
// }
//
// pub async fn disconnect_from_streaming_service_on_completion(
//     input: TransferCompletionMessage,
// ) -> anyhow::Result<()> {
//     let provider_pid = convert_uri_to_uuid(&input.provider_pid)?;
//     let db_connection = get_db_connection().await;
//     let transfer_process =
//         transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
//     if transfer_process.clone().unwrap().subscription_id.is_none() {
//         bail!("provider has not suscription opened");
//     }
//
//     let data_service =
//         resolve_endpoint_from_agreement(transfer_process.clone().unwrap().agreement_id).await?;
//     let data_service_url = data_service.dcat.endpoint_url;
//     let endpoint_delete_url = format!(
//         "{}/{}",
//         data_service_url,
//         transfer_process.clone().unwrap().subscription_id.unwrap()
//     );
//     let res = DATA_PLANE_HTTP_CLIENT.delete(endpoint_delete_url).send().await?;
//
//     let db_connection = get_db_connection().await;
//     let old_tp = transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
//     if old_tp.is_none() {
//         bail!("provider not found for agreement id");
//     }
//     let old_tp = old_tp.unwrap();
//
//     let tp = transfer_process::Entity::update(transfer_process::ActiveModel {
//         provider_pid: ActiveValue::Set(old_tp.provider_pid),
//         consumer_pid: ActiveValue::Set(old_tp.consumer_pid),
//         agreement_id: ActiveValue::Set(old_tp.agreement_id),
//         data_plane_id: ActiveValue::Set(old_tp.data_plane_id),
//         subscription_id: ActiveValue::Set(None),
//         state: ActiveValue::Set(old_tp.state),
//         created_at: ActiveValue::Set(old_tp.created_at),
//         updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
//         data_plane_address: ActiveValue::Set(old_tp.data_plane_address),
//         next_hop_address: ActiveValue::Set(old_tp.next_hop_address),
//     })
//         .exec(db_connection)
//         .await?;
//
//
//     Ok(())
// }
//
// pub async fn disconnect_from_streaming_service_on_termination(
//     input: TransferTerminationMessage,
// ) -> anyhow::Result<()> {
//     let provider_pid = convert_uri_to_uuid(&input.provider_pid)?;
//     let db_connection = get_db_connection().await;
//     let transfer_process =
//         transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
//     if transfer_process.clone().unwrap().subscription_id.is_none() {
//         bail!("provider has not suscription opened");
//     }
//
//     let data_service =
//         resolve_endpoint_from_agreement(transfer_process.clone().unwrap().agreement_id).await?;
//     let data_service_url = data_service.dcat.endpoint_url;
//     let endpoint_delete_url = format!(
//         "{}/{}",
//         data_service_url,
//         transfer_process.clone().unwrap().subscription_id.unwrap()
//     );
//     let res = DATA_PLANE_HTTP_CLIENT.delete(endpoint_delete_url).send().await?;
//
//     let db_connection = get_db_connection().await;
//     let old_tp = transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
//     if old_tp.is_none() {
//         bail!("provider not found for agreement id");
//     }
//     let old_tp = old_tp.unwrap();
//
//     let tp = transfer_process::Entity::update(transfer_process::ActiveModel {
//         provider_pid: ActiveValue::Set(old_tp.provider_pid),
//         consumer_pid: ActiveValue::Set(old_tp.consumer_pid),
//         agreement_id: ActiveValue::Set(old_tp.agreement_id),
//         data_plane_id: ActiveValue::Set(old_tp.data_plane_id),
//         subscription_id: ActiveValue::Set(None),
//         state: ActiveValue::Set(old_tp.state),
//         created_at: ActiveValue::Set(old_tp.created_at),
//         updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
//         data_plane_address: ActiveValue::Set(old_tp.data_plane_address),
//         next_hop_address: ActiveValue::Set(old_tp.next_hop_address),
//     })
//         .exec(db_connection)
//         .await?;
//
//     Ok(())
// }
