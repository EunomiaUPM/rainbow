#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use tracing_test::traced_test;
use uuid::Uuid;
use rainbow_common::dcat_formats::{DctFormats, FormatAction, FormatProtocol};
use rainbow_common::utils::convert_uuid_to_uri;
use rainbow_transfer::protocol::messages::{DataAddress, TransferMessageTypes, TransferProcessMessage, TransferRequestMessage, TRANSFER_CONTEXT};
use super::utils::{cleanup_test_env, setup_test_env};

#[traced_test]
#[tokio::test]
pub async fn transfer_push_full_case() -> anyhow::Result<()> {
    let (
        mut provider_server,
        mut consumer_server,
        client,
        agreements,
        _datasets,
        callback_address,
        consumer_pid,
        callback_id
    ) = setup_test_env().await?;

    // 1. Kickoff from client with DataAddress
    let data_address = DataAddress {
        _type: "dspace:DataAddress".to_string(),
        endpoint_type: "test".to_string(),
        endpoint: "test".to_string(),
        endpoint_properties: vec![],
    };
    // let consumer_pid = convert_uuid_to_uri(&consumer_pid)?;

    // 2. I create a TransferRequest
    let consumer_pid = Uuid::new_v4();
    let request_data = TransferRequestMessage {
        context: TRANSFER_CONTEXT.to_string(),
        _type: TransferMessageTypes::TransferRequestMessage.to_string(),
        consumer_pid: convert_uuid_to_uri(&consumer_pid)?,
        agreement_id: convert_uuid_to_uri(&agreements.get(0).unwrap().agreement_id)?,
        format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Push },
        callback_address,
        data_address: Some(data_address), // <- this data address should be created
    };
    println!("2.\n Creating TransferRequest: \n{:?}", request_data);


    // 3. Transfer request to provider
    let res = client
        .post("http://localhost:1234/transfers/request")
        .json(&request_data)
        .send()
        .await?;

    let res_body = res.json::<TransferProcessMessage>().await?;
    let provider_pid_ = res_body.provider_pid.clone();

    println!("3.\n Provider says: \n{:?}", res_body);

    // 4. Transfer start is happening under the hood. check logs

    // 5. begin data transfer

    // i have to create data layer in consumer first...

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    cleanup_test_env(provider_server, consumer_server).await
}
