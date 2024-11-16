use crate::data::entities::odrl_offer;
use crate::data::get_db_connection;
use crate::data::migrations::m20241111_000005_odrl_offers::EntityTypes;
use sea_orm::QueryFilter;
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait};
use serde_json::Value;
use uuid::Uuid;

pub async fn get_catalog_policies(catalog_id: Uuid) -> anyhow::Result<Value> {
    let db_connection = get_db_connection().await;

    let policies = odrl_offer::Entity::find()
        .filter(
            odrl_offer::Column::Entity.eq(catalog_id)
                .and(odrl_offer::Column::EntityType.eq(EntityTypes::Catalog)),
        )
        .all(db_connection)
        .await?;
    let policies_value = serde_json::to_value(policies)?;
    Ok(policies_value)
}
pub async fn post_catalog_policies(catalog_id: Uuid, policy: Value) -> anyhow::Result<Value> {
    let db_connection = get_db_connection().await;
    let new_policy = odrl_offer::ActiveModel {
        id: ActiveValue::Set(uuid::Uuid::new_v4()),
        odrl_offers: ActiveValue::Set(Some(policy)),
        entity: ActiveValue::Set(catalog_id),
        entity_type: ActiveValue::Set(EntityTypes::Catalog),
    };
    let policy_entity = odrl_offer::Entity::insert(new_policy)
        .exec_with_returning(db_connection)
        .await?;

    let policies_value = serde_json::to_value(policy_entity)?;
    Ok(policies_value)
}
pub async fn delete_catalog_policies(catalog_id: Uuid, policy_id: Uuid) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    odrl_offer::Entity::delete_by_id(policy_id)
        .exec(db_connection)
        .await?;
    Ok(())
}
pub async fn get_dataset_policies(dataset_id: Uuid) -> anyhow::Result<Value> {
    let db_connection = get_db_connection().await;

    let policies = odrl_offer::Entity::find()
        .filter(
            odrl_offer::Column::Entity.eq(dataset_id)
                .and(odrl_offer::Column::EntityType.eq(EntityTypes::Dataset)),
        )
        .all(db_connection)
        .await?;
    let policies_value = serde_json::to_value(policies)?;
    Ok(policies_value)
}
pub async fn post_dataset_policies(dataset_id: Uuid, policy: Value) -> anyhow::Result<Value> {
    let db_connection = get_db_connection().await;
    let new_policy = odrl_offer::ActiveModel {
        id: ActiveValue::Set(uuid::Uuid::new_v4()),
        odrl_offers: ActiveValue::Set(Some(policy)),
        entity: ActiveValue::Set(dataset_id),
        entity_type: ActiveValue::Set(EntityTypes::Dataset),
    };
    let policy_entity = odrl_offer::Entity::insert(new_policy)
        .exec_with_returning(db_connection)
        .await?;

    let policies_value = serde_json::to_value(policy_entity)?;
    Ok(policies_value)
}
pub async fn delete_dataset_policies(dataset_id: Uuid, policy_id: Uuid) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    odrl_offer::Entity::delete_by_id(policy_id)
        .exec(db_connection)
        .await?;
    Ok(())
}
pub async fn get_dataservices_policies(dataservice_id: Uuid) -> anyhow::Result<Value> {
    let db_connection = get_db_connection().await;

    let policies = odrl_offer::Entity::find()
        .filter(
            odrl_offer::Column::Entity.eq(dataservice_id)
                .and(odrl_offer::Column::EntityType.eq(EntityTypes::DataService)),
        )
        .all(db_connection)
        .await?;
    let policies_value = serde_json::to_value(policies)?;
    Ok(policies_value)
}
pub async fn post_dataservices_policies(dataservice_id: Uuid, policy: Value) -> anyhow::Result<Value> {
    let db_connection = get_db_connection().await;
    let new_policy = odrl_offer::ActiveModel {
        id: ActiveValue::Set(uuid::Uuid::new_v4()),
        odrl_offers: ActiveValue::Set(Some(policy)),
        entity: ActiveValue::Set(dataservice_id),
        entity_type: ActiveValue::Set(EntityTypes::DataService),
    };
    let policy_entity = odrl_offer::Entity::insert(new_policy)
        .exec_with_returning(db_connection)
        .await?;

    let policies_value = serde_json::to_value(policy_entity)?;
    Ok(policies_value)
}
pub async fn delete_dataservices_policies(dataservice_id: Uuid, policy_id: Uuid) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    odrl_offer::Entity::delete_by_id(policy_id)
        .exec(db_connection)
        .await?;
    Ok(())
}
pub async fn get_distributions_policies(distribution_id: Uuid) -> anyhow::Result<Value> {
    let db_connection = get_db_connection().await;

    let policies = odrl_offer::Entity::find()
        .filter(
            odrl_offer::Column::Entity.eq(distribution_id)
                .and(odrl_offer::Column::EntityType.eq(EntityTypes::Distribution)),
        )
        .all(db_connection)
        .await?;
    let policies_value = serde_json::to_value(policies)?;
    Ok(policies_value)
}
pub async fn post_distributions_policies(distribution_id: Uuid, policy: Value) -> anyhow::Result<Value> {
    let db_connection = get_db_connection().await;
    let new_policy = odrl_offer::ActiveModel {
        id: ActiveValue::Set(uuid::Uuid::new_v4()),
        odrl_offers: ActiveValue::Set(Some(policy)),
        entity: ActiveValue::Set(distribution_id),
        entity_type: ActiveValue::Set(EntityTypes::DataService),
    };
    let policy_entity = odrl_offer::Entity::insert(new_policy)
        .exec_with_returning(db_connection)
        .await?;

    let policies_value = serde_json::to_value(policy_entity)?;
    Ok(policies_value)
}
pub async fn delete_distributions_policies(distribution_id: Uuid, policy_id: Uuid) -> anyhow::Result<()> {
    let db_connection = get_db_connection().await;
    odrl_offer::Entity::delete_by_id(policy_id)
        .exec(db_connection)
        .await?;
    Ok(())
}