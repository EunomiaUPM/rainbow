// use super::data_plane::{disconnect_from_streaming_service_on_completion, disconnect_from_streaming_service_on_suspension, disconnect_from_streaming_service_on_termination, reconnect_to_streaming_service_on_start, resolve_endpoint_from_agreement};
use crate::common::utils::{has_data_address_in_push, is_agreement_valid};
use crate::provider::lib::data_plane::resolve_endpoint_from_agreement;
use anyhow::{bail, Error};
use rainbow_common::config::database::get_db_connection;
use rainbow_common::dcat_formats::FormatAction;
use rainbow_common::err::transfer_err::TransferErrorType;
use rainbow_common::err::transfer_err::TransferErrorType::TransferProcessNotFound;
use rainbow_common::protocol::transfer::{
    DataAddress, TransferCompletionMessage, TransferMessageTypesForDb, TransferProcessMessage,
    TransferRequestMessage, TransferRoles, TransferStartMessage, TransferStateForDb,
    TransferSuspensionMessage, TransferTerminationMessage,
};
use rainbow_common::utils::convert_uri_to_uuid;
use rainbow_dataplane::core::DataPlanePeerCreationBehavior;
use rainbow_dataplane::{
    bootstrap_data_plane_in_provider, connect_to_streaming_service,
    disconnect_from_streaming_service, set_data_plane_next_hop,
};
use rainbow_db::transfer_provider::entities::transfer_message;
use rainbow_db::transfer_provider::entities::transfer_process;
use sea_orm::{ActiveValue, EntityTrait};
use std::future::{Future, IntoFuture};
use std::str::FromStr;
use uuid::Uuid;

pub async fn get_transfer_requests_by_provider(
    provider_pid: Uuid,
) -> anyhow::Result<Option<TransferProcessMessage>> {
    let db_connection = get_db_connection().await;
    let transaction_from_db =
        transfer_process::Entity::find_by_id(provider_pid).one(db_connection).await?;
    match transaction_from_db {
        Some(t) => {
            let transaction = TransferProcessMessage::from(t);
            Ok(Some(transaction))
        }
        None => Ok(None),
    }
}

pub async fn transfer_request<F, Fut, M>(
    input: TransferRequestMessage,
    callback: F,
) -> anyhow::Result<TransferProcessMessage>
where
    F: Fn(M, Uuid, Option<DataAddress>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), Error>> + Send,
    M: From<TransferRequestMessage> + Send + 'static,
{
    let db_connection = get_db_connection().await;

    // agreement validation - validate
    if is_agreement_valid(&input.agreement_id)? == false {
        bail!(TransferErrorType::AgreementError);
    }

    // dct:format is push, dataAdress must be
    if has_data_address_in_push(&input.data_address, &input.format)? == false {
        bail!(TransferErrorType::DataAddressCannotBeNullOnPushError);
    }

    let provider_pid = Uuid::new_v4();
    let consumer_pid = convert_uri_to_uuid(&input.consumer_pid)?;
    let created_at = chrono::Utc::now().naive_utc();
    let message_type = input._type.clone();

    // data plane provision
    let agreement = convert_uri_to_uuid(&input.agreement_id)?;
    let data_service = resolve_endpoint_from_agreement(agreement).await?;

    let data_plane_peer = bootstrap_data_plane_in_provider(input.clone(), provider_pid.clone())
        .await?
        .add_attribute(
            "endpointUrl".to_string(),
            data_service.dcat.clone().endpoint_url,
        )
        .add_attribute(
            "endpointDescription".to_string(),
            data_service.dcat.clone().endpoint_description,
        );
    let data_plane_peer =
        set_data_plane_next_hop(data_plane_peer, provider_pid.clone(), consumer_pid).await?;
    let data_plane_id = data_plane_peer.id.clone();
    connect_to_streaming_service(data_plane_id).await?;

    // db persist
    let transfer_process_db = transfer_process::Entity::insert(transfer_process::ActiveModel {
        provider_pid: ActiveValue::Set(provider_pid),
        consumer_pid: ActiveValue::Set(Some(input.consumer_pid.parse().unwrap())),
        agreement_id: ActiveValue::Set(input.agreement_id.parse()?),
        data_plane_id: ActiveValue::Set(Some(data_plane_id)),
        state: ActiveValue::Set(TransferStateForDb::REQUESTED),
        created_at: ActiveValue::Set(created_at),
        updated_at: ActiveValue::Set(None),
    })
    .exec_with_returning(db_connection)
    .await?;

    let transfer_message_db = transfer_message::Entity::insert(transfer_message::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        transfer_process_id: ActiveValue::Set(provider_pid),
        created_at: ActiveValue::Set(created_at),
        message_type: ActiveValue::Set(TransferMessageTypesForDb::try_from(message_type)?),
        from: ActiveValue::Set(TransferRoles::Consumer),
        to: ActiveValue::Set(TransferRoles::Provider),
        content: ActiveValue::Set(serde_json::to_value(&input)?),
    })
    .exec_with_returning(db_connection)
    .await?;

    // prepare data address for transfer start message
    let data_address = match input.clone().format.action {
        FormatAction::Push => None,
        FormatAction::Pull => Some(DataAddress {
            _type: "dspace:DataAddress".to_string(),
            endpoint_type: "HTTP".to_string(),
            endpoint: data_service.dcat.endpoint_description.to_string(),
            endpoint_properties: vec![],
        }),
    };

    // callback for sending after a transfer start
    callback(input.into(), provider_pid, data_address).await?;

    // return
    let tp = TransferProcessMessage::from(transfer_process_db);
    Ok(tp)
}

pub async fn transfer_start(input: TransferStartMessage) -> anyhow::Result<TransferProcessMessage> {
    let db_connection = get_db_connection().await;
    // persist information
    let old_process =
        transfer_process::Entity::find_by_id(Uuid::from_str(input.provider_pid.as_str())?)
            .one(db_connection)
            .await?;
    if old_process.is_none() {
        bail!(TransferProcessNotFound)
    }
    let old_process = old_process.unwrap();

    let transfer_process_db = transfer_process::Entity::update(transfer_process::ActiveModel {
        provider_pid: ActiveValue::Set(old_process.provider_pid),
        consumer_pid: ActiveValue::Set(old_process.consumer_pid),
        agreement_id: ActiveValue::Set(old_process.agreement_id),
        data_plane_id: ActiveValue::Set(old_process.data_plane_id),
        state: ActiveValue::Set(TransferStateForDb::STARTED),
        created_at: ActiveValue::Set(old_process.created_at),
        updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
    })
    .exec(db_connection)
    .await?;

    let transfer_message_db = transfer_message::Entity::insert(transfer_message::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        transfer_process_id: ActiveValue::Set(input.provider_pid.parse()?),
        created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        message_type: ActiveValue::Set(TransferMessageTypesForDb::try_from(input._type.clone())?),
        from: ActiveValue::Set(TransferRoles::Consumer),
        to: ActiveValue::Set(TransferRoles::Provider),
        content: ActiveValue::Set(serde_json::to_value(&input)?),
    })
    .exec_with_returning(db_connection)
    .await?;

    let tp = TransferProcessMessage::from(transfer_process_db.clone());

    connect_to_streaming_service(transfer_process_db.data_plane_id.unwrap()).await?;

    Ok(tp)
}

pub async fn transfer_suspension(
    input: TransferSuspensionMessage,
) -> anyhow::Result<TransferProcessMessage> {
    let db_connection = get_db_connection().await;
    // persist information
    let old_process =
        transfer_process::Entity::find_by_id(Uuid::from_str(input.provider_pid.as_str())?)
            .one(db_connection)
            .await?;
    if old_process.is_none() {
        bail!(TransferProcessNotFound)
    }
    let old_process = old_process.unwrap();
    let transfer_process_db = transfer_process::Entity::update(transfer_process::ActiveModel {
        provider_pid: ActiveValue::Set(old_process.provider_pid),
        consumer_pid: ActiveValue::Set(old_process.consumer_pid),
        agreement_id: ActiveValue::Set(old_process.agreement_id),
        data_plane_id: ActiveValue::Set(old_process.data_plane_id),
        state: ActiveValue::Set(TransferStateForDb::SUSPENDED),
        created_at: ActiveValue::Set(old_process.created_at),
        updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
    })
    .exec(db_connection)
    .await?;

    let transfer_message_db = transfer_message::Entity::insert(transfer_message::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        transfer_process_id: ActiveValue::Set(input.provider_pid.parse()?),
        created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        message_type: ActiveValue::Set(TransferMessageTypesForDb::try_from(input._type.clone())?),
        from: ActiveValue::Set(TransferRoles::Consumer),
        to: ActiveValue::Set(TransferRoles::Provider),
        content: ActiveValue::Set(serde_json::to_value(&input)?),
    })
    .exec_with_returning(db_connection)
    .await?;

    let tp = TransferProcessMessage::from(transfer_process_db.clone());

    disconnect_from_streaming_service(transfer_process_db.data_plane_id.unwrap()).await?;

    Ok(tp)
}

pub async fn transfer_completion(
    input: TransferCompletionMessage,
) -> anyhow::Result<TransferProcessMessage> {
    let db_connection = get_db_connection().await;
    // persist information
    let old_process =
        transfer_process::Entity::find_by_id(Uuid::from_str(input.provider_pid.as_str())?)
            .one(db_connection)
            .await?;
    if old_process.is_none() {
        bail!(TransferProcessNotFound)
    }
    let old_process = old_process.unwrap();
    let transfer_process_db = transfer_process::Entity::update(transfer_process::ActiveModel {
        provider_pid: ActiveValue::Set(old_process.provider_pid),
        consumer_pid: ActiveValue::Set(old_process.consumer_pid),
        agreement_id: ActiveValue::Set(old_process.agreement_id),
        data_plane_id: ActiveValue::Set(old_process.data_plane_id),
        state: ActiveValue::Set(TransferStateForDb::COMPLETED),
        created_at: ActiveValue::Set(old_process.created_at),
        updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
    })
    .exec(db_connection)
    .await?;

    let transfer_message_db = transfer_message::Entity::insert(transfer_message::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        transfer_process_id: ActiveValue::Set(input.provider_pid.parse()?),
        created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        message_type: ActiveValue::Set(TransferMessageTypesForDb::try_from(input._type.clone())?),
        from: ActiveValue::Set(TransferRoles::Consumer),
        to: ActiveValue::Set(TransferRoles::Provider),
        content: ActiveValue::Set(serde_json::to_value(&input)?),
    })
    .exec_with_returning(db_connection)
    .await?;

    let tp = TransferProcessMessage::from(transfer_process_db.clone());

    disconnect_from_streaming_service(transfer_process_db.data_plane_id.unwrap()).await?;

    Ok(tp)
}

pub async fn transfer_termination(
    input: TransferTerminationMessage,
) -> anyhow::Result<TransferProcessMessage> {
    let db_connection = get_db_connection().await;
    // persist information
    let old_process =
        transfer_process::Entity::find_by_id(Uuid::from_str(input.provider_pid.as_str())?)
            .one(db_connection)
            .await?;
    if old_process.is_none() {
        bail!(TransferProcessNotFound)
    }
    let old_process = old_process.unwrap();
    let transfer_process_db = transfer_process::Entity::update(transfer_process::ActiveModel {
        provider_pid: ActiveValue::Set(old_process.provider_pid),
        consumer_pid: ActiveValue::Set(old_process.consumer_pid),
        agreement_id: ActiveValue::Set(old_process.agreement_id),
        data_plane_id: ActiveValue::Set(old_process.data_plane_id),
        state: ActiveValue::Set(TransferStateForDb::TERMINATED),
        created_at: ActiveValue::Set(old_process.created_at),
        updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
    })
    .exec(db_connection)
    .await?;

    let transfer_message_db = transfer_message::Entity::insert(transfer_message::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        transfer_process_id: ActiveValue::Set(input.provider_pid.parse()?),
        created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        message_type: ActiveValue::Set(TransferMessageTypesForDb::try_from(input._type.clone())?),
        from: ActiveValue::Set(TransferRoles::Consumer),
        to: ActiveValue::Set(TransferRoles::Provider),
        content: ActiveValue::Set(serde_json::to_value(&input)?),
    })
    .exec_with_returning(db_connection)
    .await?;

    // // if suscription id cancel
    // if transfer_process_db.subscription_id.is_some() {
    //     disconnect_from_streaming_service_on_termination(input).await?;
    // }

    let tp = TransferProcessMessage::from(transfer_process_db.clone());

    disconnect_from_streaming_service(transfer_process_db.data_plane_id.unwrap()).await?;

    Ok(tp)
}
