use crate::data::entities::data_plane_field;
use crate::data::entities::data_plane_field::{EditDataPlaneFieldModel, NewDataPlaneFieldModel};
use crate::data::repo_traits::data_plane_fields_repo::{DataPlaneFieldRepoErrors, DataPlaneFieldRepoTrait};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use urn::{Urn, UrnBuilder};

pub struct DataPlaneFieldRepoForSql {
    db_connection: DatabaseConnection,
}
impl DataPlaneFieldRepoForSql {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait::async_trait]
impl DataPlaneFieldRepoTrait for DataPlaneFieldRepoForSql {
    async fn get_all_data_plane_fields(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<data_plane_field::Model>, DataPlaneFieldRepoErrors> {
        let fields = data_plane_field::Entity::find()
            .limit(limit.unwrap_or(20))
            .offset(page.map(|p| p * limit.unwrap_or(20)).unwrap_or(0))
            .all(&self.db_connection)
            .await;
        match fields {
            Ok(fields) => Ok(fields),
            Err(e) => Err(DataPlaneFieldRepoErrors::ErrorFetchingDataplaneField(
                e.into(),
            )),
        }
    }

    async fn get_batch_data_plane_fields(
        &self,
        ids: &Vec<Urn>,
    ) -> anyhow::Result<Vec<data_plane_field::Model>, DataPlaneFieldRepoErrors> {
        let ids = ids.iter().map(|t| t.to_string()).collect::<Vec<_>>();
        let fields = data_plane_field::Entity::find()
            .filter(data_plane_field::Column::Id.is_in(ids))
            .all(&self.db_connection)
            .await;
        match fields {
            Ok(fields) => Ok(fields),
            Err(e) => Err(DataPlaneFieldRepoErrors::ErrorFetchingDataplaneField(
                e.into(),
            )),
        }
    }

    async fn get_all_data_plane_fields_by_process_id(
        &self,
        process_id: &Urn,
    ) -> anyhow::Result<Vec<data_plane_field::Model>, DataPlaneFieldRepoErrors> {
        let pid = process_id.to_string();
        let fields = data_plane_field::Entity::find()
            .filter(data_plane_field::Column::DataPlaneProcessId.eq(pid))
            .all(&self.db_connection)
            .await;

        match fields {
            Ok(fields) => Ok(fields),
            Err(e) => Err(DataPlaneFieldRepoErrors::ErrorFetchingDataplaneField(
                e.into(),
            )),
        }
    }

    async fn get_data_plane_field_by_id(
        &self,
        field_id: &Urn,
    ) -> anyhow::Result<Option<data_plane_field::Model>, DataPlaneFieldRepoErrors> {
        let pid = field_id.to_string();
        let field = data_plane_field::Entity::find_by_id(pid).one(&self.db_connection).await;
        match field {
            Ok(field) => Ok(field),
            Err(e) => Err(DataPlaneFieldRepoErrors::ErrorFetchingDataplaneField(
                e.into(),
            )),
        }
    }

    async fn create_data_plane_field(
        &self,
        process_id: &Urn,
        new_data_plane_field: &NewDataPlaneFieldModel,
    ) -> anyhow::Result<data_plane_field::Model, DataPlaneFieldRepoErrors> {
        let id = UrnBuilder::new("dataplane-field", uuid::Uuid::new_v4().to_string().as_str())
            .build()
            .expect("UrnBuilder failed");

        let model = data_plane_field::ActiveModel {
            id: ActiveValue::Set(id.to_string()),
            key: ActiveValue::Set(new_data_plane_field.key.clone()),
            value: ActiveValue::Set(new_data_plane_field.value.clone()),
            data_plane_process_id: ActiveValue::Set(process_id.to_string()),
        };

        let transfer_proces = data_plane_field::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match transfer_proces {
            Ok(transfer_process) => Ok(transfer_process),
            Err(e) => {
                return Err(DataPlaneFieldRepoErrors::ErrorFetchingDataplaneField(
                    e.into(),
                ))
            }
        }
    }

    async fn put_data_plane_field(
        &self,
        field_id: &Urn,
        edit_field: &EditDataPlaneFieldModel,
    ) -> anyhow::Result<data_plane_field::Model, DataPlaneFieldRepoErrors> {
        let old_model = self.get_data_plane_field_by_id(field_id).await;
        let old_model = match old_model {
            Ok(old_model) => match old_model {
                Some(old_model) => old_model,
                None => return Err(DataPlaneFieldRepoErrors::DataplaneFieldNotFound),
            },
            Err(e) => {
                return Err(DataPlaneFieldRepoErrors::ErrorFetchingDataplaneField(
                    e.into(),
                ))
            }
        };
        let mut old_active_model: data_plane_field::ActiveModel = old_model.into();
        old_active_model.value = ActiveValue::Set(edit_field.value.clone());
        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(e) => Err(DataPlaneFieldRepoErrors::ErrorUpdatingDataplaneField(
                e.into(),
            )),
        }
    }

    async fn delete_data_plane_field(&self, field_id: &Urn) -> anyhow::Result<(), DataPlaneFieldRepoErrors> {
        let id = field_id.to_string();
        let field = data_plane_field::Entity::delete_by_id(id).exec(&self.db_connection).await;
        match field {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(DataPlaneFieldRepoErrors::DataplaneFieldNotFound),
                _ => Ok(()),
            },
            Err(e) => Err(DataPlaneFieldRepoErrors::ErrorDeletingDataplaneField(
                e.into(),
            )),
        }
    }
}
