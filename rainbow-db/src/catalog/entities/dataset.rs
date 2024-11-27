use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dataset")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub dct_conforms_to: Option<String>,
    pub dct_creator: Option<String>,
    pub dct_identifier: Option<String>,
    pub dct_issued: chrono::NaiveDateTime,
    pub dct_modified: Option<chrono::NaiveDateTime>,
    pub dct_title: Option<String>,
    pub dct_description: Option<String>,
    pub catalog_id: Uuid
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::catalog::Entity",
        from = "Column::CatalogId",
        to = "super::catalog::Column::Id"
    )]
    Catalog,
    #[sea_orm(has_many = "super::distribution::Entity")]
    Distribution,
    #[sea_orm(has_many = "super::odrl_offer::Entity")]
    OdrlOffer,
}

impl Related<super::catalog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Catalog.def()
    }
}


impl Related<super::distribution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Distribution.def()
    }
}

impl Related<super::odrl_offer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OdrlOffer.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}