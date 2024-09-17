use tracing::debug;

pub fn is_consumer_pid_valid(consumer_pid: &String) -> anyhow::Result<bool> {
    debug!("Validating consumer_pid: {}", consumer_pid);
    Ok(true)
}

pub fn is_agreement_valid(agreement_id: &String) -> anyhow::Result<bool> {
    debug!("Validating agreement_id: {}", agreement_id);
    Ok(true)
}

pub fn is_provider_valid(provider_pid: &String) -> anyhow::Result<bool> {
    debug!("Validating provider_pid: {}", provider_pid);
    Ok(true)
}
