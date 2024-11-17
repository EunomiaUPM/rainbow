use super::super::migrations::m20241111_000005_odrl_offers::EntityTypes;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "odrl_offers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub odrl_offers: Option<serde_json::Value>,
    pub entity: Uuid,
    pub entity_type: EntityTypes,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::catalog::Entity",
        from = "Column::Entity",
        to = "super::catalog::Column::Id"
    )]
    Catalog,
    #[sea_orm(
        belongs_to = "super::dataset::Entity",
        from = "Column::Entity",
        to = "super::dataset::Column::Id"
    )]
    Dataset,
    #[sea_orm(
        belongs_to = "super::dataservice::Entity",
        from = "Column::Entity",
        to = "super::dataservice::Column::Id"
    )]
    DataService,
    #[sea_orm(
        belongs_to = "super::distribution::Entity",
        from = "Column::Entity",
        to = "super::distribution::Column::Id"
    )]
    Distribution,
}
impl Related<super::catalog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Catalog.def()
    }
}
impl Related<super::dataset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dataset.def()
    }
}
impl Related<super::dataservice::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DataService.def()
    }
}
impl Related<super::distribution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Distribution.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
