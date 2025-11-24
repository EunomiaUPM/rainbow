use rainbow_transfer_a::setup::cmd::TransferCommands;
use std::process::Command;

#[tokio::test]
async fn flow_transfer() -> anyhow::Result<()> {
    let cwd = "./../rainbow-transfer";
    let mut provider_server = Command::new("cargo")
        .current_dir(cwd)
        .args(&["run", "--", "start", "-e ./../static/envs/.env.bifrost.core.p"])
        .spawn()
        .expect("Failed to start provider server");
    let cwd = "./../rainbow-transfer";
    let mut consumer_server = Command::new("cargo")
        .current_dir(cwd)
        .args(&["run", "--", "start", "-e ./../static/envs/.env.bifrost.core.c"])
        .spawn()
        .expect("Failed to start consumer server");

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    provider_server.kill().expect("Failed to kill provider server");
    consumer_server.kill().expect("Failed to kill consumer server");
    Ok(())
}
