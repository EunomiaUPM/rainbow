use crate::provider::core::data_service_resolver::DataServiceFacadeTrait;
// use crate::common::utils::{has_data_address_in_push, is_agreement_valid};
use crate::provider::core::ds_protocol::DSProtocolTransferProviderTrait;
use anyhow::anyhow;
use axum::async_trait;
use rainbow_common::dcat_formats::FormatAction;
use rainbow_common::protocol::transfer::{
    DataAddress, TransferCompletionMessage, TransferProcessMessage, TransferRequestMessage, TransferRoles,
    TransferStartMessage, TransferStateForDb, TransferSuspensionMessage, TransferTerminationMessage,
};
use rainbow_common::utils::{get_urn, get_urn_from_string};
use rainbow_dataplane::facade::DataPlaneFacade;
use rainbow_db::transfer_provider::repo::{EditTransferProcessModel, NewTransferMessageModel, NewTransferProcessModel, TransferProviderRepoFactory};
use std::sync::Arc;
use urn::Urn;

pub struct DSProtocolTransferProviderImpl<T, U, V>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: DataServiceFacadeTrait + Send + Sync,
    V: DataPlaneFacade + Send + Sync,
{
    transfer_repo: Arc<T>,
    data_service_facade: Arc<U>,
    data_plane: Arc<V>,
}

impl<T, U, V> DSProtocolTransferProviderImpl<T, U, V>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: DataServiceFacadeTrait + Send + Sync,
    V: DataPlaneFacade + Send + Sync,
{
    pub fn new(
        transfer_repo: Arc<T>,
        data_service_facade: Arc<U>,
        data_plane: Arc<V>,
    ) -> Self {
        Self { transfer_repo, data_service_facade, data_plane }
    }
}

#[async_trait]
impl<T, U, V> DSProtocolTransferProviderTrait for DSProtocolTransferProviderImpl<T, U, V>
where
    T: TransferProviderRepoFactory + Send + Sync,
    U: DataServiceFacadeTrait + Send + Sync,
    V: DataPlaneFacade + Send + Sync,
{
    async fn get_transfer_requests_by_provider(
        &self,
        provider_pid: Urn,
    ) -> anyhow::Result<TransferProcessMessage> {
        // TODO THIS
        let transfers = self.transfer_repo.get_transfer_process_by_provider(provider_pid).await?.ok_or(anyhow!("nope"))?;
        let transfers = TransferProcessMessage::from(transfers);
        Ok(transfers)
    }

    async fn transfer_request(&self, input: TransferRequestMessage) -> anyhow::Result<TransferProcessMessage> {
        // // agreement validation - validate
        // if is_agreement_valid(&input.agreement_id)? == false {
        //     bail!(TransferErrorType::AgreementError);
        // }
        //
        // // dct:format is push, dataAdress must be
        // if has_data_address_in_push(&input.data_address, &input.format)? == false {
        //     bail!(TransferErrorType::DataAddressCannotBeNullOnPushError);
        // }

        let provider_pid = get_urn(None);
        let consumer_pid = get_urn_from_string(&input.consumer_pid)?;
        let created_at = chrono::Utc::now().naive_utc();
        let message_type = input._type.clone();

        // data plane provision
        let agreement_id = get_urn_from_string(&input.agreement_id)?;
        let data_service = self.data_service_facade.resolve_data_service_by_agreement_id(agreement_id.clone()).await?;
        // let data_plane_peer = self
        //     .data_plane
        //     .bootstrap_data_plane_in_provider(input.clone(), provider_pid.clone())
        //     .await?
        //     .add_attribute(
        //         "endpointUrl".to_string(),
        //         data_service.clone().dcat.clone().endpoint_url,
        //     )
        //     .add_attribute(
        //         "endpointDescription".to_string(),
        //         data_service.clone().dcat.clone().endpoint_description,
        //     );
        // println!("{:?}", data_plane_peer);
        //
        // let data_plane_peer = self
        //     .data_plane
        //     .set_data_plane_next_hop(data_plane_peer, provider_pid.clone(), consumer_pid.clone())
        //     .await?;
        // let data_plane_id = data_plane_peer.id.clone();
        // self.data_plane.connect_to_streaming_service(data_plane_id.clone()).await?;


        // db persist
        let transfer_process_db = self
            .transfer_repo
            .create_transfer_process(NewTransferProcessModel {
                provider_pid: provider_pid.clone(),
                consumer_pid,
                agreement_id,
                // data_plane_id,
                data_plane_id: get_urn(None),
            })
            .await?;

        let _ = self
            .transfer_repo
            .create_transfer_message(
                provider_pid.clone(),
                NewTransferMessageModel {
                    message_type,
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: serde_json::to_value(&input)?,
                },
            )
            .await?;

        // prepare data address for transfer start message
        let data_address = match input.clone().format.action {
            FormatAction::Push => None,
            FormatAction::Pull => Some(DataAddress {
                _type: "dspace:DataAddress".to_string(),
                endpoint_type: "HTTP".to_string(),
                endpoint: data_service.clone().dcat.endpoint_description.to_string(),
                endpoint_properties: vec![],
            }),
        };

        // // callback for sending after a transfer start
        // callback(input.into(), provider_pid, data_address).await?;

        // return
        let tp = TransferProcessMessage::from(transfer_process_db);
        Ok(tp)
    }

    async fn transfer_start(
        &self,
        provider_pid: Urn,
        input: TransferStartMessage,
    ) -> anyhow::Result<TransferProcessMessage> {
        // persist process
        let transfer_process_db = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel { state: Option::from(TransferStateForDb::STARTED), ..Default::default() },
            )
            .await?;

        // create message
        let _ = self
            .transfer_repo
            .create_transfer_message(
                provider_pid,
                NewTransferMessageModel {
                    message_type: input._type.clone(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: serde_json::to_value(&input)?,
                },
            )
            .await?;

        let tp = TransferProcessMessage::from(transfer_process_db.clone());
        // data plane
        let data_plane_id = get_urn_from_string(&transfer_process_db.data_plane_id.unwrap())?;
        self.data_plane.connect_to_streaming_service(data_plane_id).await?;

        Ok(tp)
    }

    async fn transfer_suspension(
        &self,
        provider_pid: Urn,
        input: TransferSuspensionMessage,
    ) -> anyhow::Result<TransferProcessMessage> {
        // persist process

        let transfer_process_db = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel { state: Option::from(TransferStateForDb::COMPLETED), ..Default::default() },
            )
            .await?;

        // create message
        let _ = self
            .transfer_repo
            .create_transfer_message(
                provider_pid,
                NewTransferMessageModel {
                    message_type: input._type.clone(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: serde_json::to_value(&input)?,
                },
            )
            .await?;

        let tp = TransferProcessMessage::from(transfer_process_db.clone());

        // data plane
        let data_plane_id = get_urn_from_string(&transfer_process_db.data_plane_id.unwrap())?;
        self.data_plane.disconnect_from_streaming_service(data_plane_id).await?;
        Ok(tp)
    }

    async fn transfer_completion(
        &self,
        provider_pid: Urn,
        input: TransferCompletionMessage,
    ) -> anyhow::Result<TransferProcessMessage> {
        // persist process

        let transfer_process_db = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel { state: Option::from(TransferStateForDb::COMPLETED), ..Default::default() },
            )
            .await?;

        // create message
        let _ = self
            .transfer_repo
            .create_transfer_message(
                provider_pid,
                NewTransferMessageModel {
                    message_type: input._type.clone(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: serde_json::to_value(&input)?,
                },
            )
            .await?;

        let tp = TransferProcessMessage::from(transfer_process_db.clone());

        // data plane
        let data_plane_id = get_urn_from_string(&transfer_process_db.data_plane_id.unwrap())?;
        self.data_plane.disconnect_from_streaming_service(data_plane_id).await?;
        Ok(tp)
    }

    async fn transfer_termination(
        &self,
        provider_pid: Urn,
        input: TransferTerminationMessage,
    ) -> anyhow::Result<TransferProcessMessage> {
        // persist process

        let transfer_process_db = self
            .transfer_repo
            .put_transfer_process(
                provider_pid.clone(),
                EditTransferProcessModel { state: Option::from(TransferStateForDb::TERMINATED), ..Default::default() },
            )
            .await?;

        // create message
        let _ = self
            .transfer_repo
            .create_transfer_message(
                provider_pid,
                NewTransferMessageModel {
                    message_type: input._type.clone(),
                    from: TransferRoles::Consumer,
                    to: TransferRoles::Provider,
                    content: serde_json::to_value(&input)?,
                },
            )
            .await?;

        let tp = TransferProcessMessage::from(transfer_process_db.clone());

        // data plane
        let data_plane_id = get_urn_from_string(&transfer_process_db.data_plane_id.unwrap())?;
        self.data_plane.disconnect_from_streaming_service(data_plane_id).await?;
        Ok(tp)
    }
}
