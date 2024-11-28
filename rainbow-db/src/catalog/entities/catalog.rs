use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "catalog")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub foaf_home_page: Option<String>,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_identifier: Option<String>,
    pub dct_issued: chrono::NaiveDateTime,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub dct_title: Option<String>,
    pub dspace_participant_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::dataset::Entity")]
    Dataset,
    #[sea_orm(has_many = "super::dataservice::Entity")]
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
