use rainbow_common::dcat_formats::{DctFormats, FormatAction, FormatProtocol};
use rainbow_common::utils::get_urn;
use rainbow_transfer::consumer::core::ds_protocol_rpc::ds_protocol_rpc_types::{DSRPCTransferConsumerRequestRequest, DSRPCTransferConsumerRequestResponse, DSRPCTransferConsumerStartRequest, DSRPCTransferConsumerStartResponse, DSRPCTransferConsumerSuspensionRequest, DSRPCTransferConsumerSuspensionResponse};
use rainbow_transfer::provider::core::ds_protocol_rpc::ds_protocol_rpc_types::{DSRPCTransferProviderCompletionRequest, DSRPCTransferProviderCompletionResponse, DSRPCTransferProviderStartRequest, DSRPCTransferProviderStartResponse};
use serde_json::Value;
use std::process::Command;
use tracing_test::traced_test;

#[traced_test]
#[tokio::test]
pub async fn transfer_rpc_pull() -> anyhow::Result<()> {
    //
    // Setup servers
    //
    let cwd = "./../rainbow-transfer";
    let mut provider_server = Command::new("cargo")
        .current_dir(cwd)
        .args(&["run", "--", "provider", "start"])
        .spawn()
        .expect("Failed to start provider server");
    let cwd = "./../rainbow-transfer";
    let mut consumer_server = Command::new("cargo")
        .current_dir(cwd)
        .args(&["run", "--", "consumer", "start"])
        .spawn()
        .expect("Failed to start consumer server");

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let client = reqwest::Client::new();

    // -------------------------------
    // Consumer RPC TransferRequest
    // -------------------------------
    // Client in consumer side initiates with sending DSRPCTransferConsumerRequestRequest
    let message = DSRPCTransferConsumerRequestRequest {
        provider_address: "http://localhost:1234".to_string(),
        agreement_id: get_urn(None).to_string(),
        format: DctFormats { protocol: FormatProtocol::Http, action: FormatAction::Pull },
        data_address: None,
    };
    let url = "http://localhost:1235/api/v1/transfers/rpc/setup-request";
    println!("\n\n1. Requesting from consumer:\n{}\n{}", url, serde_json::to_string_pretty(&message)?);
    let request = client
        .post(url)
        .json(&message)
        .send()
        .await?;
    let response = request.json::<DSRPCTransferConsumerRequestResponse>().await?;
    println!("1. Ack from provider\n{}\n\n", serde_json::to_string_pretty(&response)?);

    // At this point correlation IDs are created
    let provider_pid = response.provider_pid.clone();
    let consumer_pid = response.consumer_pid.clone();
    let consumer_callback_address = response.callback_address.clone();


    // -------------------------------
    // Providers RPC TransferStart
    // -------------------------------
    // Client in consumer side initiates with sending DSRPCTransferConsumerRequestRequest
    let message = DSRPCTransferProviderStartRequest {
        consumer_callback: consumer_callback_address.clone(),
        provider_pid: provider_pid.clone(),
        consumer_pid: consumer_pid.clone(),
        data_address: None,
    };
    let url = "http://localhost:1234/api/v1/transfers/rpc/setup-start";
    println!("\n\n2. Sending Start message from provider:\n{}\n{}", url, serde_json::to_string_pretty(&message)?);
    let request = client
        .post(url)
        .json(&message)
        .send()
        .await?;
    let response = request.json::<DSRPCTransferProviderStartResponse>().await?;
    println!("2. Ack from consumer\n{}\n\n", serde_json::to_string_pretty(&response)?);


    // -------------------------------
    // Consumer RPCs TransferSuspension
    // -------------------------------
    // Client in consumer side initiates with sending DSRPCTransferConsumerRequestRequest
    let message = DSRPCTransferConsumerSuspensionRequest {
        provider_address: "http://localhost:1234".to_string(),
        provider_pid: provider_pid.clone(),
        consumer_pid: consumer_pid.clone(),
        code: Some("TEST".to_string()),
        reason: Some(vec!["TEST".to_string()]),
    };
    let url = "http://localhost:1235/api/v1/transfers/rpc/setup-suspension";
    println!("\n\n3. Sending Suspension message from consumer:\n{}\n{}", url, serde_json::to_string_pretty(&message)?);
    let request = client
        .post(url)
        .json(&message)
        .send()
        .await?;
    let response = request.json::<DSRPCTransferConsumerSuspensionResponse>().await?;
    println!("3. Ack from provider\n{}\n\n", serde_json::to_string_pretty(&response)?);


    // -------------------------------
    // Consumer RPCs TransferRestart
    // -------------------------------
    // Client in consumer side initiates with sending DSRPCTransferConsumerRequestRequest
    let message = DSRPCTransferConsumerStartRequest {
        provider_address: "http://localhost:1234".to_string(),
        provider_pid: provider_pid.clone(),
        consumer_pid: consumer_pid.clone(),
        data_address: None,
    };
    let url = "http://localhost:1235/api/v1/transfers/rpc/setup-start";
    println!("\n\n4. Sending Restart message from consumer:\n{}\n{}", url, serde_json::to_string_pretty(&message)?);
    let request = client
        .post(url)
        .json(&message)
        .send()
        .await?;
    let response = request.json::<DSRPCTransferConsumerStartResponse>().await?;
    println!("4. Ack from provider\n{}\n\n", serde_json::to_string_pretty(&response)?);


    // -------------------------------
    // Provider RPCs TransferCompletion
    // -------------------------------
    // Client in consumer side initiates with sending DSRPCTransferConsumerRequestRequest
    let message = DSRPCTransferProviderCompletionRequest {
        consumer_callback: consumer_callback_address.clone(),
        provider_pid: provider_pid.clone(),
        consumer_pid: consumer_pid.clone(),
    };
    let url = "http://localhost:1234/api/v1/transfers/rpc/setup-completion";
    println!("\n\n5. Sending Completion message from provider:\n{}\n{}", url, serde_json::to_string_pretty(&message)?);
    let request = client
        .post(url)
        .json(&message)
        .send()
        .await?;
    let response = request.json::<DSRPCTransferProviderCompletionResponse>().await?;
    println!("5. Ack from consumer\n{}\n\n", serde_json::to_string_pretty(&response)?);


    //
    // Tear down servers
    //
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    provider_server.kill().expect("Failed to kill provider server");
    consumer_server.kill().expect("Failed to kill consumer server");
    Ok(())
}
