use crate::entities::resource::ProtocolSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum InteractionConfig {
    #[serde(rename = "PULL")]
    Pull(PullLifecycle),

    #[serde(rename = "PUSH")]
    Push(PushLifecycle),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullLifecycle {
    pub data_access: ProtocolSpec,
    pub scheduler: SchedulerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushLifecycle {
    pub subscribe: ProtocolSpec,
    pub unsubscribe: Option<ProtocolSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SchedulerConfig {
    Interval { seconds: u64 },
    Cron { expression: String },
}
