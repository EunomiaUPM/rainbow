use crate::ssi_auth::consumer::SSI_AUTH_HTTP_CLIENT;
use anyhow::bail;
use axum::http::StatusCode;
use rainbow_common::config::config::{get_consumer_ssi_holder, GLOBAL_CONFIG};
use tracing::debug;

pub type ConsumerSSIVCRequest = serde_json::Value;

pub async fn consumer_vc_request(input: ConsumerSSIVCRequest) -> anyhow::Result<()> {
    let ssi_consumer_url = get_consumer_ssi_holder()?;
    let res = SSI_AUTH_HTTP_CLIENT
        .post(ssi_consumer_url)
        .json(&input)
        .send()
        .await
        .map_err(|_| bail!("SSI holder not available"))?;

    match res.status() {
        StatusCode::CREATED => {
            let vc = res.text().await?;
            debug!(vc);
            Ok(())
        }
        _ => bail!("Credential not issued"),
    }
}

pub async fn consumer_wf_exchange_from_consumer() -> anyhow::Result<()> {
    Ok(())
}
