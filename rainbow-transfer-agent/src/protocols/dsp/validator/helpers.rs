use crate::entities::transfer_process::TransferProcessDto;
use anyhow::{anyhow, bail};
use log::error;
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use rainbow_common::protocol::transfer::TransferRoles;
use std::str::FromStr;
use urn::Urn;

pub fn validate_role(role: &str) -> anyhow::Result<&str> {
    match &role.parse::<TransferRoles>() {
        Ok(role_) => match role_ {
            TransferRoles::Provider => Ok("providerPid"),
            TransferRoles::Consumer => Ok("consumerPid"),
        },
        Err(_e) => {
            let err = CommonErrors::parse_new(format!("No role expected: {}", role).as_str());
            error!("{}", err.log());
            bail!(err)
        }
    }
}

pub fn get_pid_from_identifiers(dto: &TransferProcessDto, key: &str) -> anyhow::Result<String> {
    dto.identifiers.get(key).cloned().ok_or_else(|| {
        let err = CommonErrors::missing_resource_new(key, format!("Identifiers not found {}", key).as_str());
        error!("{}", err.log());
        anyhow!(err)
    })
}

pub fn validate_pid_match(pid: &str, id: &Urn) -> anyhow::Result<()> {
    let pid_as_urn = Urn::from_str(pid)?;
    if pid_as_urn.to_string() != id.to_string() {
        let err = CommonErrors::parse_new("Body and Uri identifiers do not coincide");
        error!("{}", err.log());
        bail!(err);
    }
    Ok(())
}

pub fn validate_pids_correlation(
    provider_pid: &str,
    consumer_pid: &str,
    dto_provider_pid: &str,
    dto_consumer_pid: &str,
) -> anyhow::Result<()> {
    if provider_pid != dto_provider_pid || consumer_pid != dto_consumer_pid {
        let err = CommonErrors::format_new(
            BadFormat::Received,
            "ConsumerPid or providerPid not coincide with transfer process identifiers",
        );
        error!("{}", err.log());
        return Err(anyhow!(err));
    }
    Ok(())
}
