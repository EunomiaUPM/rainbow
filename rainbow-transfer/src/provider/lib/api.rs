/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */

use anyhow::bail;
use rainbow_db::transfer_provider::entities::{agreements, transfer_message};

use rainbow_common::config::database::get_db_connection;
use rainbow_common::protocol::transfer::TransferProcessMessage;
use rainbow_db::transfer_provider::repo::{
    EditAgreementModel, NewAgreementModel, TRANSFER_PROVIDER_REPO,
};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn get_all_transfers() -> anyhow::Result<Vec<TransferProcessMessage>> {
    let transfer_processes_from_db =
        TRANSFER_PROVIDER_REPO.get_all_transfer_processes(None, None).await?;
    let transfer_processes = transfer_processes_from_db
        .iter()
        .map(|t| TransferProcessMessage::from(t.clone()))
        .collect();
    Ok(transfer_processes)
}

pub async fn get_messages_by_transfer(
    transfer_id: Uuid,
) -> anyhow::Result<Vec<transfer_message::Model>> {
    let messages =
        TRANSFER_PROVIDER_REPO.get_all_transfer_messages_by_provider(transfer_id).await?;
    Ok(messages)
}

pub async fn get_messages_by_id(
    transfer_id: Uuid,
    message_id: Uuid,
) -> anyhow::Result<transfer_message::Model> {
    let message = TRANSFER_PROVIDER_REPO.get_transfer_message_by_id(message_id).await?;
    let message = match message {
        Some(message) => message,
        None => bail!("Message {} not found", message_id),
    };
    Ok(message)
}

pub async fn get_all_agreements() -> anyhow::Result<Vec<agreements::Model>> {
    let agreements = TRANSFER_PROVIDER_REPO.get_all_agreements(None, None).await?;
    Ok(agreements)
}

pub async fn get_agreement_by_id(agreement_id: Uuid) -> anyhow::Result<agreements::Model> {
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::find_by_id(agreement_id).one(db_connection).await?;
    if agreement.is_none() {
        bail!("Agreement not found");
    }
    Ok(agreement.unwrap())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewAgreement {
    #[serde(rename = "dataServiceId")]
    data_service_id: Uuid,
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    identity: Option<String>,
    #[serde(rename = "identityToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_token: Option<String>,
}

pub async fn post_agreement(new_agreement: NewAgreement) -> anyhow::Result<agreements::Model> {
    let agreement = TRANSFER_PROVIDER_REPO
        .create_agreement(NewAgreementModel {
            data_service_id: new_agreement.data_service_id,
            identity: new_agreement.identity,
            identity_token: new_agreement.identity_token,
        })
        .await?;
    Ok(agreement)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EditAgreement {
    #[serde(rename = "dataServiceId")]
    data_service_id: Option<Uuid>,
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    identity: Option<String>,
    #[serde(rename = "identityToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_token: Option<String>,
}

pub async fn put_agreement(
    agreement_id: Uuid,
    new_agreement: EditAgreement,
) -> anyhow::Result<agreements::Model> {
    let agreement = TRANSFER_PROVIDER_REPO
        .put_agreement(
            agreement_id,
            EditAgreementModel {
                data_service_id: new_agreement.data_service_id,
                identity: new_agreement.identity,
                identity_token: new_agreement.identity_token,
            },
        )
        .await?;
    Ok(agreement)
}

pub async fn delete_agreement(agreement_id: Uuid) -> anyhow::Result<()> {
    TRANSFER_PROVIDER_REPO.delete_agreement(agreement_id).await?;
    Ok(())
}
