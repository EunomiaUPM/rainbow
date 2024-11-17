use std::ops::Deref;
use crate::db::get_db_relational_connection_r2d2;
use crate::fake_catalog::data::models::DatasetsCatalogModel;
use crate::fake_catalog::data::schema::dataset_catalogs::dsl::dataset_catalogs;
use crate::fake_catalog::data::schema::dataset_catalogs::{dataset_endpoint, dataset_id};
use diesel::prelude::*;
use diesel::{OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use serde_json::Value;
use uuid::Uuid;

pub fn create_dataset_repo(endpoint: String, endpoint_properties: Option<&Value>) -> anyhow::Result<DatasetsCatalogModel> {
    let connection = &mut get_db_relational_connection_r2d2().get()?;
    let endpoint_properties = match endpoint_properties {
        Some(ep) => Some(ep.clone()),
        None => None
    };
    let transaction = diesel::insert_into(dataset_catalogs)
        .values(DatasetsCatalogModel {
            dataset_id: Uuid::new_v4(),
            dataset_endpoint: endpoint,
            dataset_endpoint_properties: endpoint_properties,
        })
        .returning(DatasetsCatalogModel::as_returning())
        .get_result(connection)?;

    Ok(transaction)
}

pub fn get_dataset_by_id_repo(id: Uuid) -> anyhow::Result<Option<DatasetsCatalogModel>> {
    let connection = &mut get_db_relational_connection_r2d2().get()?;
    let transaction = dataset_catalogs
        .filter(dataset_id.eq(id))
        .select(DatasetsCatalogModel::as_returning())
        .first(connection)
        .optional()?;

    Ok(transaction)
}

pub fn get_datasets_by_endpoint_repo(
    endpoint: String,
) -> anyhow::Result<Vec<DatasetsCatalogModel>> {
    let connection = &mut get_db_relational_connection_r2d2().get()?;
    let transaction = dataset_catalogs
        .filter(dataset_endpoint.eq(endpoint))
        .select(DatasetsCatalogModel::as_returning())
        .get_results(connection)?;

    Ok(transaction)
}

pub fn delete_dataset_repo(id: Uuid) -> anyhow::Result<()> {
    let connection = &mut get_db_relational_connection_r2d2().get()?;
    let _ = diesel::delete(dataset_catalogs.filter(dataset_id.eq(id))).execute(connection)?;

    Ok(())
}
