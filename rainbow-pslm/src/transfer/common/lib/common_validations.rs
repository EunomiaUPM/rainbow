use crate::transfer::common::err::TransferErrorType::{PidSchemeError, PidUuidError};
use anyhow::bail;
use serde_json::Value;
use uuid::Uuid;

async fn _check_urn(pid: Option<&str>) -> anyhow::Result<()> {
    if pid.is_some() {
        let scheme = pid.unwrap().contains("urn:uuid:");
        let uuid_value = Uuid::parse_str(&pid.unwrap().replace("urn:uuid:", ""));
        if uuid_value.is_err() {
            bail!(PidUuidError)
        }
        if scheme == false {
            bail!(PidSchemeError)
        }
    }
    Ok(())
}
pub async fn pids_as_urn_validation(json_value: Value) -> anyhow::Result<()> {
    let provider_pid = json_value.get("dspace:providerPid").and_then(|v| v.as_str());
    let consumer_pid = json_value.get("dspace:consumerPid").and_then(|v| v.as_str());
    let agreement_pid = json_value.get("dspace:agreementId").and_then(|v| v.as_str());

    _check_urn(provider_pid).await?;
    _check_urn(consumer_pid).await?;
    _check_urn(agreement_pid).await?;

    Ok(())
}
