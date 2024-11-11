use crate::transfer::common::err::TransferErrorType;
use crate::transfer::common::utils::convert_uuid_to_uri;
use crate::transfer::common::utils::{has_data_address_in_push, is_agreement_valid};
use crate::transfer::protocol::formats::FormatAction;
use crate::transfer::protocol::messages::TransferMessageTypes;
use crate::transfer::protocol::messages::{
    TransferCompletionMessage, TransferProcessMessage, TransferRequestMessage,
    TransferStartMessage, TransferState, TransferSuspensionMessage, TransferTerminationMessage,
    TRANSFER_CONTEXT,
};
use crate::transfer::provider::data::models::{TransferMessageModel, TransferProcessModel};
use crate::transfer::provider::data::repo::TransferProviderDataRepo;
use crate::transfer::provider::data::repo::TRANSFER_PROVIDER_REPO;
use crate::transfer::provider::lib::data_plane::data_plane_start;
use anyhow::{bail, Error};
use std::future::{Future, IntoFuture};
use uuid::Uuid;

pub async fn get_transfer_requests_by_provider(
    provider_pid: Uuid,
) -> anyhow::Result<Option<TransferProcessModel>> {
    // access info
    let transaction = TRANSFER_PROVIDER_REPO.get_transfer_process_by_provider_pid(provider_pid)?;
    Ok(transaction)
}


pub async fn transfer_request<F, Fut, M>(
    input: TransferRequestMessage,
    callback: F,
) -> anyhow::Result<TransferProcessMessage>
where
    F: Fn(M, Uuid, Uuid) -> Fut + Send + Sync + 'static,
    Fut: Future<Output=Result<(), Error>> + Send,
    M: From<TransferRequestMessage> + Send + 'static,
{

    // agreement validation - validate
    if is_agreement_valid(&input.agreement_id)? == false {
        bail!(TransferErrorType::AgreementError);
    }
    
    // dct:format is push, dataAdress must be
    if has_data_address_in_push(&input.data_address, &input.format)? == false {
        bail!(TransferErrorType::DataAddressCannotBeNullOnPushError);
    }

    // REQUEST PART
    // persist information
    let provider_pid = Uuid::new_v4();
    let created_at = chrono::Utc::now().naive_utc();
    let message_type = input._type.clone();

    TRANSFER_PROVIDER_REPO.create_transfer_process(TransferProcessModel {
        provider_pid,
        consumer_pid: input.consumer_pid.parse()?,
        agreement_id: input.agreement_id.parse()?,
        data_plane_id: None,
        state: TransferState::REQUESTED.to_string(),
        created_at,
        updated_at: None,
    })?;

    TRANSFER_PROVIDER_REPO.create_transfer_message(TransferMessageModel {
        id: Uuid::new_v4(),
        transfer_process_id: provider_pid,
        created_at,
        message_type,
        from: "consumer".to_string(),
        to: "provider".to_string(),
        content: serde_json::to_value(&input)?,
    })?;

    let tp = TransferProcessMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferProcessMessage.to_string(),
        provider_pid: convert_uuid_to_uri(&provider_pid)?,
        consumer_pid: (&input.consumer_pid).to_owned(),
        state: TransferState::REQUESTED,
    };


    // Connect to data streaming in case push
    // if input.format.action == FormatAction::Push {
    //     // TODO here streaming connection
    //     // connect_to_streaming_service(input, provider_pid).await?;
    // }

    data_plane_start(input, provider_pid.clone(), callback).await?;

    Ok(tp)
}

pub async fn transfer_start(input: &TransferStartMessage) -> anyhow::Result<TransferProcessMessage> {
    // persist information
    let transaction = TRANSFER_PROVIDER_REPO.update_transfer_process_by_provider_pid(
        &input.provider_pid.parse()?,
        TransferState::STARTED,
        None,
    )?;

    TRANSFER_PROVIDER_REPO.create_transfer_message(TransferMessageModel {
        id: Uuid::new_v4(),
        transfer_process_id: input.provider_pid.parse()?,
        created_at: chrono::Utc::now().naive_utc(),
        message_type: input._type.clone(),
        from: "consumer".to_string(),
        to: "provider".to_string(),
        content: serde_json::to_value(input)?,
    })?;

    let tp = TransferProcessMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferProcessMessage.to_string(),
        provider_pid: convert_uuid_to_uri(&input.provider_pid.clone().parse()?)?,
        consumer_pid: (&input.consumer_pid).to_owned(),
        state: TransferState::STARTED,
    };

    Ok(tp)
}

pub async fn transfer_suspension(
    input: &TransferSuspensionMessage,
) -> anyhow::Result<TransferProcessMessage> {
    let transaction = TRANSFER_PROVIDER_REPO.update_transfer_process_by_provider_pid(
        &input.provider_pid.parse()?,
        TransferState::SUSPENDED,
        None,
    )?;

    TRANSFER_PROVIDER_REPO.create_transfer_message(TransferMessageModel {
        id: Uuid::new_v4(),
        transfer_process_id: input.provider_pid.parse()?,
        created_at: chrono::Utc::now().naive_utc(),
        message_type: input._type.clone(),
        from: "consumer".to_string(),
        to: "provider".to_string(),
        content: serde_json::to_value(input.clone())?,
    })?;

    let tp = TransferProcessMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferProcessMessage.to_string(),
        provider_pid: convert_uuid_to_uri(&input.provider_pid.clone().parse()?)?,
        consumer_pid: (&input.consumer_pid).to_owned(),
        state: TransferState::SUSPENDED,
    };

    Ok(tp)
}

pub async fn transfer_completion(
    input: &TransferCompletionMessage,
) -> anyhow::Result<TransferProcessMessage> {
    let transaction = TRANSFER_PROVIDER_REPO.update_transfer_process_by_provider_pid(
        &input.provider_pid.parse()?,
        TransferState::COMPLETED,
        None,
    )?;

    TRANSFER_PROVIDER_REPO.create_transfer_message(TransferMessageModel {
        id: Uuid::new_v4(),
        transfer_process_id: input.provider_pid.parse()?,
        created_at: chrono::Utc::now().naive_utc(),
        message_type: input._type.clone(),
        from: "consumer".to_string(),
        to: "provider".to_string(),
        content: serde_json::to_value(input)?,
    })?;

    let tp = TransferProcessMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferProcessMessage.to_string(),
        provider_pid: convert_uuid_to_uri(&input.provider_pid.clone().parse()?)?,
        consumer_pid: (&input.consumer_pid).to_owned(),
        state: TransferState::COMPLETED,
    };

    Ok(tp)
}

pub async fn transfer_termination(
    input: &TransferTerminationMessage,
) -> anyhow::Result<TransferProcessMessage> {
    let transaction = TRANSFER_PROVIDER_REPO.update_transfer_process_by_provider_pid(
        &input.provider_pid.parse()?,
        TransferState::TERMINATED,
        None,
    )?;

    TRANSFER_PROVIDER_REPO.create_transfer_message(TransferMessageModel {
        id: Uuid::new_v4(),
        transfer_process_id: input.provider_pid.parse()?,
        created_at: chrono::Utc::now().naive_utc(),
        message_type: input._type.clone(),
        from: "consumer".to_string(),
        to: "provider".to_string(),
        content: serde_json::to_value(input)?,
    })?;

    let tp = TransferProcessMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferProcessMessage.to_string(),
        provider_pid: convert_uuid_to_uri(&input.provider_pid.clone().parse()?)?,
        consumer_pid: (&input.consumer_pid).to_owned(),
        state: TransferState::TERMINATED,
    };

    Ok(tp)
}
