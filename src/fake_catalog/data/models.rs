use crate::fake_catalog::data::schema::dataset_catalogs;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Queryable,
    Identifiable,
    Selectable,
    Debug,
    PartialEq,
    Insertable,
    Clone,
    Serialize,
    Deserialize
)]
#[diesel(table_name=dataset_catalogs)]
#[diesel(primary_key(dataset_id))]
pub struct DatasetsCatalogModel {
    pub dataset_id: Uuid,
    pub dataset_endpoint: String,
}
