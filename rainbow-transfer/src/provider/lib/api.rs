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
use rainbow_db::transfer_provider::entities::transfer_process;
use rainbow_db::transfer_provider::entities::{agreements, transfer_message};

use rainbow_common::config::database::get_db_connection;
use rainbow_common::protocol::transfer::TransferProcessMessage;
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn get_all_transfers() -> anyhow::Result<Vec<TransferProcessMessage>> {
    let db_connection = get_db_connection().await;
    let transfer_processes_from_db = transfer_process::Entity::find().all(db_connection).await?;

    let mut transfer_processes = vec![];
    for tp in transfer_processes_from_db {
        transfer_processes.push(TransferProcessMessage::from(tp));
    }
    Ok(transfer_processes)
}

pub async fn get_messages_by_transfer(
    transfer_id: Uuid,
) -> anyhow::Result<Vec<transfer_message::Model>> {
    let db_connection = get_db_connection().await;
    println!("{}", transfer_id);
    let messages = transfer_message::Entity::find()
        .filter(transfer_message::Column::TransferProcessId.eq(transfer_id))
        .all(db_connection)
        .await?;
    Ok(messages)
}

pub async fn get_messages_by_id(
    transfer_id: Uuid,
    message_id: Uuid,
) -> anyhow::Result<transfer_message::Model> {
    let db_connection = get_db_connection().await;
    let messages = transfer_message::Entity::find()
        .filter(
            transfer_message::Column::TransferProcessId
                .eq(transfer_id)
                .and(transfer_message::Column::Id.eq(message_id)),
        )
        .one(db_connection)
        .await?;
    if messages.is_none() {
        bail!("transfer message with id {} not found", message_id);
    }
    Ok(messages.unwrap())
}

pub async fn get_all_agreements() -> anyhow::Result<Vec<agreements::Model>> {
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::find().all(db_connection).await?;
    Ok(agreement)
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
    let db_connection = get_db_connection().await;
    let agreement_in_db = agreements::Entity::insert(agreements::ActiveModel {
        agreement_id: ActiveValue::Set(Uuid::new_v4()),
        data_service_id: ActiveValue::Set(new_agreement.data_service_id),
        identity: ActiveValue::Set(new_agreement.identity),
        identity_token: ActiveValue::Set(new_agreement.identity_token),
    })
        .exec_with_returning(db_connection)
        .await?;
    Ok(agreement_in_db)
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
    let db_connection = get_db_connection().await;
    let old_agreement = agreements::Entity::find_by_id(agreement_id).one(db_connection).await?;
    if old_agreement.is_none() {
        bail!("Agreement not found");
    }
    let mut agreement_in_db: agreements::ActiveModel = old_agreement.unwrap().into();
    if let Some(data_service_id) = new_agreement.data_service_id {
        agreement_in_db.data_service_id = ActiveValue::Set(data_service_id);
    }
    if let Some(identity) = new_agreement.identity {
        agreement_in_db.identity = ActiveValue::Set(Some(identity));
    }
    if let Some(identity_token) = new_agreement.identity_token {
        agreement_in_db.identity_token = ActiveValue::Set(Some(identity_token));
    }
    let agreement = agreements::Entity::update(agreement_in_db).exec(db_connection).await?;
    Ok(agreement)
}
pub async fn delete_agreement(agreement_id: Uuid) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    let old_agreement = agreements::Entity::delete_by_id(agreement_id).exec(db_connection).await?;
    if old_agreement.rows_affected == 0 {
        bail!("Agreement not found");
    }
    Ok(())
}
