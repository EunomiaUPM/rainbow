use crate::common::http::client::DATA_PLANE_HTTP_CLIENT;
use crate::protocol::messages::{DataAddress, TransferCompletionMessage, TransferRequestMessage, TransferStartMessage, TransferSuspensionMessage, TransferTerminationMessage};
use crate::provider::data::entities::agreements;
use crate::provider::data::entities::transfer_process;
use anyhow::{bail, Error};
use rainbow_catalog::core::ll_api::dataservices_request_by_id;
use rainbow_catalog::protocol::dataservice_definition::DataService;
use rainbow_common::config::config::get_provider_url;
use rainbow_common::config::database::get_db_connection;
use rainbow_common::dcat_formats::FormatAction;
use rainbow_common::utils::convert_uri_to_uuid;
use sea_orm::{ActiveValue, EntityTrait};
use std::future::Future;
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

/// Provisions data plane
/// Creates data plane address
/// Creates next_hop_addresses for provider data plane
/// Establishes connection to data streaming source
pub async fn data_plane_start<F, Fut, M>(
    input: TransferRequestMessage,
    provider_pid: Uuid,
    cb: F,
) -> anyhow::Result<()>
where
    F: Fn(M, Uuid, DataAddress, DataAddress, Uuid) -> Fut + Send + Sync + 'static,
    Fut: Future<Output=Result<(), Error>> + Send,
    M: From<TransferRequestMessage> + Send + 'static,
{
    // Data plane address
    let data_plane_id = Uuid::new_v4();
    let data_plane_address_url = match input.format.action {
        FormatAction::Push => format!(
            "http://{}/data/push/{}",
            get_provider_url()?,
            data_plane_id.to_string()
        ),
        FormatAction::Pull => format!(
            "http://{}/data/pull/{}",
            get_provider_url()?,
            data_plane_id.to_string()
        ),
    };
    let data_plane_address = DataAddress {
        _type: "dspace:DataAddress".to_string(),
        endpoint_type: "HTTP".to_string(),
        endpoint: data_plane_address_url,
        endpoint_properties: vec![],
    };

    // Next hop addresses
    let agreement_uuid = convert_uri_to_uuid(&input.agreement_id)?;
    let consumer_pid = convert_uri_to_uuid(&input.consumer_pid)?;
    let next_hop_address = match input.format.action {
        // Data address push is going to be consumer data plane
        FormatAction::Push => {
            let callback_id = input.callback_address.clone();
            let endpoint_url = format!("{}/data/push/{}", callback_id, consumer_pid);
            let next_hop_address = DataAddress {
                _type: "dspace:DataAddress".to_string(),
                endpoint: endpoint_url,
                endpoint_type: "HTTP".to_string(),
                endpoint_properties: vec![],
            };
            next_hop_address
        }
        // Data address pull is going to be final system endpoint
        FormatAction::Pull => {
            let data_service = resolve_endpoint_from_agreement(agreement_uuid).await?;
            let endpoint_url = data_service.dcat.endpoint_url;
            let next_hop_address = DataAddress {
                _type: "dspace:DataAddress".to_string(),
                endpoint: endpoint_url,
                endpoint_type: "HTTP".to_string(),
                endpoint_properties: vec![],
            };
            next_hop_address
        }
    };

    // Connect to streaming service
    match input.format.action {
        FormatAction::Push => {
            connect_to_streaming_service(&input, provider_pid, data_plane_address.clone()).await?;
        }
        FormatAction::Pull => {}
    }

    cb(
        input.into(),
        provider_pid,
        next_hop_address,
        data_plane_address,
        data_plane_id,
    )
        .await?;
    Ok(())
}

pub async fn connect_to_streaming_service(
    input: &TransferRequestMessage,
    provider_pid: Uuid,
    data_address: DataAddress,
) -> anyhow::Result<()> {
    println!("Connecting to streaming service...");
    println!("{:?}", input);

    // resolve agreement
    let agreement_id = &input.agreement_id;
    let agreement_id = convert_uri_to_uuid(agreement_id)?;

    // resolve data service
    let data_service = resolve_endpoint_from_agreement(agreement_id).await?;
    let endpoint_url = data_service.dcat.endpoint_url;
    let endpoint_description = data_service.dcat.endpoint_description;

    // create payload to cb
    // here is the crux....
    let cb_suscription_payload = endpoint_description
        .replace("$data_url", &*data_address.endpoint)
        .replace("$data_description", "My description");
    let cb_suscription_payload =
        serde_json::from_str::<serde_json::Value>(&cb_suscription_payload)?;

    // suscribe to cb
    let res =
        DATA_PLANE_HTTP_CLIENT.post(endpoint_url).json(&cb_suscription_payload).send().await?;

    // persist suscription identifier
    let suscription_id = res.headers().get("location");
    if suscription_id.is_none() {
        // TODO error
        bail!("not able to connect to streaming service")
    }
    let suscription_id = suscription_id.unwrap().to_str()?;
    let suscription_id = suscription_id.replace("/v2/subscriptions/", "");
    println!("Suscription: {}", suscription_id);

    let db_connection = get_db_connection().await;
    let old_tp = transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
    if old_tp.is_none() {
        bail!("provider not found for agreement id");
    }
    let old_tp = old_tp.unwrap();
    let tp = transfer_process::Entity::update(transfer_process::ActiveModel {
        provider_pid: ActiveValue::Set(old_tp.provider_pid),
        consumer_pid: ActiveValue::Set(old_tp.consumer_pid),
        agreement_id: ActiveValue::Set(old_tp.agreement_id),
        data_plane_id: ActiveValue::Set(old_tp.data_plane_id),
        subscription_id: ActiveValue::Set(Some(suscription_id)),
        state: ActiveValue::Set(old_tp.state),
        created_at: ActiveValue::Set(old_tp.created_at),
        updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
        data_plane_address: ActiveValue::Set(old_tp.data_plane_address),
        next_hop_address: ActiveValue::Set(old_tp.next_hop_address),
    })
        .exec(db_connection)
        .await?;

    match res.status() {
        reqwest::StatusCode::CREATED => {}
        // TODO error
        _ => bail!("not able to connect to streaming service"),
    }

    Ok(())
}

pub async fn reconnect_to_streaming_service_on_start(
    input: TransferStartMessage,
) -> anyhow::Result<()> {
    let provider_pid = convert_uri_to_uuid(&input.provider_pid)?;
    let db_connection = get_db_connection().await;
    let transfer = transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
    if transfer.is_none() {
        bail!("provider not found for agreement id");
    }
    let transfer = transfer.unwrap();

    // resolve agreement
    let agreement_id = transfer.agreement_id;

    // resolve data service
    let data_service = resolve_endpoint_from_agreement(agreement_id).await?;
    let endpoint_url = data_service.dcat.endpoint_url;
    let endpoint_description = data_service.dcat.endpoint_description;

    // create payload to cb
    // here is the crux....
    let cb_suscription_payload = endpoint_description
        .replace("$data_url", transfer.data_plane_address.unwrap().as_str())
        .replace("$data_description", "My description");
    let cb_suscription_payload =
        serde_json::from_str::<serde_json::Value>(&cb_suscription_payload)?;

    // suscribe to cb
    let res =
        DATA_PLANE_HTTP_CLIENT.post(endpoint_url).json(&cb_suscription_payload).send().await?;

    // persist suscription identifier
    let suscription_id = res.headers().get("location");
    if suscription_id.is_none() {
        // TODO error
        bail!("not able to connect to streaming service")
    }
    let suscription_id = suscription_id.unwrap().to_str()?;
    let suscription_id = suscription_id.replace("/v2/subscriptions/", "");
    println!("Suscription: {}", suscription_id);

    let old_tp = transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
    if old_tp.is_none() {
        bail!("provider not found for agreement id");
    }
    let old_tp = old_tp.unwrap();
    let tp = transfer_process::Entity::update(transfer_process::ActiveModel {
        provider_pid: ActiveValue::Set(old_tp.provider_pid),
        consumer_pid: ActiveValue::Set(old_tp.consumer_pid),
        agreement_id: ActiveValue::Set(old_tp.agreement_id),
        data_plane_id: ActiveValue::Set(old_tp.data_plane_id),
        subscription_id: ActiveValue::Set(Some(suscription_id)),
        state: ActiveValue::Set(old_tp.state),
        created_at: ActiveValue::Set(old_tp.created_at),
        updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
        data_plane_address: ActiveValue::Set(old_tp.data_plane_address),
        next_hop_address: ActiveValue::Set(old_tp.next_hop_address),
    })
        .exec(db_connection)
        .await?;

    match res.status() {
        reqwest::StatusCode::CREATED => {}
        // TODO error
        _ => bail!("not able to connect to streaming service"),
    }
    Ok(())
}

pub async fn disconnect_from_streaming_service_on_suspension(
    input: TransferSuspensionMessage,
) -> anyhow::Result<()> {
    let provider_pid = convert_uri_to_uuid(&input.provider_pid)?;
    let db_connection = get_db_connection().await;
    let transfer_process =
        transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
    if transfer_process.clone().unwrap().subscription_id.is_none() {
        bail!("provider has not suscription opened");
    }

    let data_service =
        resolve_endpoint_from_agreement(transfer_process.clone().unwrap().agreement_id).await?;
    let data_service_url = data_service.dcat.endpoint_url;
    let endpoint_delete_url = format!(
        "{}/{}",
        data_service_url,
        transfer_process.clone().unwrap().subscription_id.unwrap()
    );
    let res = DATA_PLANE_HTTP_CLIENT.delete(endpoint_delete_url).send().await?;

    let old_tp = transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
    if old_tp.is_none() {
        bail!("provider not found for agreement id");
    }
    let old_tp = old_tp.unwrap();

    let tp = transfer_process::Entity::update(transfer_process::ActiveModel {
        provider_pid: ActiveValue::Set(old_tp.provider_pid),
        consumer_pid: ActiveValue::Set(old_tp.consumer_pid),
        agreement_id: ActiveValue::Set(old_tp.agreement_id),
        data_plane_id: ActiveValue::Set(old_tp.data_plane_id),
        subscription_id: ActiveValue::Set(None),
        state: ActiveValue::Set(old_tp.state),
        created_at: ActiveValue::Set(old_tp.created_at),
        updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
        data_plane_address: ActiveValue::Set(old_tp.data_plane_address),
        next_hop_address: ActiveValue::Set(old_tp.next_hop_address),
    })
        .exec(db_connection)
        .await?;

    Ok(())
}

pub async fn disconnect_from_streaming_service_on_completion(
    input: TransferCompletionMessage,
) -> anyhow::Result<()> {
    let provider_pid = convert_uri_to_uuid(&input.provider_pid)?;
    let db_connection = get_db_connection().await;
    let transfer_process =
        transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
    if transfer_process.clone().unwrap().subscription_id.is_none() {
        bail!("provider has not suscription opened");
    }

    let data_service =
        resolve_endpoint_from_agreement(transfer_process.clone().unwrap().agreement_id).await?;
    let data_service_url = data_service.dcat.endpoint_url;
    let endpoint_delete_url = format!(
        "{}/{}",
        data_service_url,
        transfer_process.clone().unwrap().subscription_id.unwrap()
    );
    let res = DATA_PLANE_HTTP_CLIENT.delete(endpoint_delete_url).send().await?;

    let db_connection = get_db_connection().await;
    let old_tp = transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
    if old_tp.is_none() {
        bail!("provider not found for agreement id");
    }
    let old_tp = old_tp.unwrap();

    let tp = transfer_process::Entity::update(transfer_process::ActiveModel {
        provider_pid: ActiveValue::Set(old_tp.provider_pid),
        consumer_pid: ActiveValue::Set(old_tp.consumer_pid),
        agreement_id: ActiveValue::Set(old_tp.agreement_id),
        data_plane_id: ActiveValue::Set(old_tp.data_plane_id),
        subscription_id: ActiveValue::Set(None),
        state: ActiveValue::Set(old_tp.state),
        created_at: ActiveValue::Set(old_tp.created_at),
        updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
        data_plane_address: ActiveValue::Set(old_tp.data_plane_address),
        next_hop_address: ActiveValue::Set(old_tp.next_hop_address),
    })
        .exec(db_connection)
        .await?;


    Ok(())
}

pub async fn disconnect_from_streaming_service_on_termination(
    input: TransferTerminationMessage,
) -> anyhow::Result<()> {
    let provider_pid = convert_uri_to_uuid(&input.provider_pid)?;
    let db_connection = get_db_connection().await;
    let transfer_process =
        transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
    if transfer_process.clone().unwrap().subscription_id.is_none() {
        bail!("provider has not suscription opened");
    }

    let data_service =
        resolve_endpoint_from_agreement(transfer_process.clone().unwrap().agreement_id).await?;
    let data_service_url = data_service.dcat.endpoint_url;
    let endpoint_delete_url = format!(
        "{}/{}",
        data_service_url,
        transfer_process.clone().unwrap().subscription_id.unwrap()
    );
    let res = DATA_PLANE_HTTP_CLIENT.delete(endpoint_delete_url).send().await?;

    let db_connection = get_db_connection().await;
    let old_tp = transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
    if old_tp.is_none() {
        bail!("provider not found for agreement id");
    }
    let old_tp = old_tp.unwrap();

    let tp = transfer_process::Entity::update(transfer_process::ActiveModel {
        provider_pid: ActiveValue::Set(old_tp.provider_pid),
        consumer_pid: ActiveValue::Set(old_tp.consumer_pid),
        agreement_id: ActiveValue::Set(old_tp.agreement_id),
        data_plane_id: ActiveValue::Set(old_tp.data_plane_id),
        subscription_id: ActiveValue::Set(None),
        state: ActiveValue::Set(old_tp.state),
        created_at: ActiveValue::Set(old_tp.created_at),
        updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
        data_plane_address: ActiveValue::Set(old_tp.data_plane_address),
        next_hop_address: ActiveValue::Set(old_tp.next_hop_address),
    })
        .exec(db_connection)
        .await?;
    
    Ok(())
}
