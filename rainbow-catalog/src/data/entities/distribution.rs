use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "distribution")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub dct_issued: chrono::NaiveDateTime,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
    pub dcat_access_service: Uuid,
    pub dataset_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::dataset::Entity",
        from = "Column::DatasetId",
        to = "super::dataset::Column::Id"
    )]
    Dataset,
    #[sea_orm(
        belongs_to = "super::dataservice::Entity",
        from = "Column::DcatAccessService",
        to = "super::dataservice::Column::Id"
    )]
    DataService,
    #[sea_orm(has_many = "super::odrl_offer::Entity")]
    OdrlOffer,
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

impl Related<super::odrl_offer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OdrlOffer.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}