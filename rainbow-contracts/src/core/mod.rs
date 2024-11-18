use crate::data::entities::agreements;
use rainbow_common::config::database::get_db_connection;
use anyhow::bail;
use sea_orm::{ActiveValue, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn create_agreement(data_service_id: Uuid) -> anyhow::Result<agreements::Model> {
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::insert(agreements::ActiveModel {
        agreement_id: ActiveValue::Set(Uuid::new_v4()),
        data_service_id: ActiveValue::Set(data_service_id),
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
pub async fn edit_agreement(agreement_id: Uuid, agreement_data: AgreementIdentity) -> anyhow::Result<Option<agreements::Model>> {
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::find_by_id(agreement_id)
        .one(db_connection)
        .await?;
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

pub async fn delete_agreement(agreement_id: Uuid) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::delete_by_id(agreement_id)
        .exec(db_connection)
        .await?;
    if agreement.rows_affected == 0 {
        bail!("Agreement does not exist");
    }
    Ok(())
}

pub async fn get_agreement_by_id(agreement_id: Uuid) -> anyhow::Result<Option<agreements::Model>> {
    let db_connection = get_db_connection().await;
    let agreement = agreements::Entity::find_by_id(agreement_id)
        .one(db_connection)
        .await?;
    // let agreement = get_agreement_by_id_repo(agreement_id)?;
    Ok(agreement)
}
