use crate::data::entities::vc_requests;
use crate::data::repo::{
    EditVCRequestModel, NewVCRequestModel, VCRequestsFactory, VCRequestsRepo, VCRequestsRepoErrors,
};
use axum::async_trait;
use rainbow_common::protocol::transfer::TransferState;
use rainbow_common::utils::get_urn;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};
use urn::Urn;

pub struct VCRequestsRepoForSql {
    db_connection: DatabaseConnection,
}

impl VCRequestsRepoForSql {
    fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

impl VCRequestsFactory for VCRequestsRepoForSql {
    fn create_repo(database_connection: DatabaseConnection) -> Self {
        Self::new(database_connection)
    }
}

#[async_trait]
impl VCRequestsRepo for VCRequestsRepoForSql {
    async fn get_all_vc_requests(
        &self,
        limit: Option<u64>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<vc_requests::Model>, VCRequestsRepoErrors> {
        let vc_requests = vc_requests::Entity::find()
            .all(&self.db_connection)
            .await
            .map_err(|e| VCRequestsRepoErrors::ErrorFetchingVCRequest(e.into()))?;
        Ok(vc_requests)
    }

    async fn get_all_vc_request_by_id(&self, id: Urn) -> anyhow::Result<vc_requests::Model, VCRequestsRepoErrors> {
        let vc_requests = vc_requests::Entity::find_by_id(id.to_string())
            .one(&self.db_connection)
            .await
            .map_err(|e| VCRequestsRepoErrors::ErrorFetchingVCRequest(e.into()))?
            .ok_or(VCRequestsRepoErrors::VCRequestNotFound)?;
        Ok(vc_requests)
    }

    async fn put_vc_request(
        &self,
        pid: Urn,
        edit_vc_request: EditVCRequestModel,
    ) -> anyhow::Result<vc_requests::Model, VCRequestsRepoErrors> {
        let vc_request = self.get_all_vc_request_by_id(pid).await?;

        let mut old_active_model: vc_requests::ActiveModel = vc_request.into();
        if let Some(state) = edit_vc_request.state {
            old_active_model.state = ActiveValue::Set(state);
        }
        let model = old_active_model.update(&self.db_connection).await;
        match model {
            Ok(model) => Ok(model),
            Err(e) => return Err(VCRequestsRepoErrors::ErrorUpdatingVCRequest(e.into())),
        }
    }

    async fn create_vc_request(
        &self,
        new_vc_request: NewVCRequestModel,
    ) -> anyhow::Result<vc_requests::Model, VCRequestsRepoErrors> {
        let model = vc_requests::ActiveModel {
            id: ActiveValue::Set(get_urn(None).to_string()),
            content: Default::default(),
            state: ActiveValue::Set(TransferState::REQUESTED.to_string()),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };
        let transfer_process = vc_requests::Entity::insert(model).exec_with_returning(&self.db_connection).await;
        match transfer_process {
            Ok(transfer_process) => Ok(transfer_process),
            Err(e) => return Err(VCRequestsRepoErrors::ErrorCreatingVCRequest(e.into())),
        }
    }

    async fn delete_vc_request(&self, pid: Urn) -> anyhow::Result<(), VCRequestsRepoErrors> {
        let transfer_process = vc_requests::Entity::delete_by_id(pid.to_string()).exec(&self.db_connection).await;
        match transfer_process {
            Ok(delete_result) => match delete_result.rows_affected {
                0 => Err(VCRequestsRepoErrors::VCRequestNotFound),
                _ => Ok(()),
            },
            Err(e) => Err(VCRequestsRepoErrors::ErrorDeletingVCRequest(e.into())),
        }
    }
}
