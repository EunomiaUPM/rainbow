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
use rainbow_common::config::database::get_db_connection;
use rainbow_db::contracts::entities::agreements;
use rainbow_common::utils::get_urn;
use sea_orm::{ActiveValue, EntityTrait};
use serde::{Deserialize, Serialize};
use urn::Urn;

pub async fn create_agreement(data_service_id: Urn) -> anyhow::Result<agreements::Model> {
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::insert(agreements::ActiveModel {
        agreement_id: ActiveValue::Set(get_urn(None).to_string()),
        data_service_id: ActiveValue::Set(data_service_id.to_string()),
        identity: ActiveValue::Set(None),
        identity_token: ActiveValue::Set(None),
    })
        .exec_with_returning(db_connection)
        .await?;
    Ok(agreement)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgreementIdentity {
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    identity: Option<String>,
    #[serde(rename = "identityToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_token: Option<String>,
}
pub async fn edit_agreement(
    agreement_id: String,
    agreement_data: AgreementIdentity,
) -> anyhow::Result<Option<agreements::Model>> {
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::find_by_id(agreement_id).one(db_connection).await?;
    if agreement.is_none() {
        return Ok(None);
    }
    let agreement = agreement.unwrap();
    let agreement = agreements::Entity::update(agreements::ActiveModel {
        agreement_id: ActiveValue::Set(agreement.agreement_id),
        data_service_id: ActiveValue::Set(agreement.data_service_id),
        identity: ActiveValue::Set(agreement_data.identity),
        identity_token: ActiveValue::Set(agreement_data.identity_token),
    })
        .exec(db_connection)
        .await?;

    Ok(Some(agreement))
}

pub async fn delete_agreement(agreement_id: String) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::delete_by_id(agreement_id).exec(db_connection).await?;
    if agreement.rows_affected == 0 {
        bail!("Agreement does not exist");
    }
    Ok(())
}

pub async fn get_agreement_by_id(agreement_id: String) -> anyhow::Result<Option<agreements::Model>> {
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::find_by_id(agreement_id).one(db_connection).await?;
    // let agreement = get_agreement_by_id_repo(agreement_id)?;
    Ok(agreement)
}

pub async fn get_agreements() -> anyhow::Result<Vec<agreements::Model>> {
    let db_connection = get_db_connection().await;
    let agreements = agreements::Entity::find().all(db_connection).await?;
    // let agreement = get_agreement_by_id_repo(agreement_id)?;
    Ok(agreements)
}
