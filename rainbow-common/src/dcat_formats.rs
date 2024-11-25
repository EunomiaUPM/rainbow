use anyhow::bail;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Clone, PartialEq)]
pub enum FormatProtocol {
    FiwareContextBroker,
    Http,
    Quic,
    Grpc,
    Kafka,
    Mqtt,
    S3,
}

impl ToString for FormatProtocol {
    fn to_string(&self) -> String {
        match self {
            FormatProtocol::FiwareContextBroker => "FiwareContextBroker".to_string(),
            FormatProtocol::Http => "Http".to_string(),
            FormatProtocol::Quic => "Quic".to_string(),
            FormatProtocol::Grpc => "Grpc".to_string(),
            FormatProtocol::Kafka => "Kafka".to_string(),
            FormatProtocol::Mqtt => "Mqtt".to_string(),
            FormatProtocol::S3 => "S3".to_string(),
        }
    }
}

impl FromStr for FormatProtocol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FiwareContextBroker" => Ok(FormatProtocol::FiwareContextBroker),
            "Http" => Ok(FormatProtocol::Http),
            "Quic" => Ok(FormatProtocol::Quic),
            "Grpc" => Ok(FormatProtocol::Grpc),
            "Kafka" => Ok(FormatProtocol::Kafka),
            "Mqtt" => Ok(FormatProtocol::Mqtt),
            "S3" => Ok(FormatProtocol::S3),
            _ => bail!("Value {} not recognized", s),
        }
    }
}

#[derive(Debug, ToSchema, Clone)]
pub enum FormatAction {
    Push,
    Pull,
}

impl ToString for FormatAction {
    fn to_string(&self) -> String {
        match self {
            FormatAction::Push => "Push".to_string(),
            FormatAction::Pull => "Pull".to_string(),
        }
    }
}

impl FromStr for FormatAction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Push" => Ok(FormatAction::Push),
            "Pull" => Ok(FormatAction::Pull),
            _ => bail!("Value {} not recognized", s),
        }
    }
}

impl PartialEq for FormatAction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FormatAction::Push, FormatAction::Push) => true,
            (FormatAction::Pull, FormatAction::Pull) => true,
            (_, _) => false,
        }
    }
}

#[derive(Debug, Clone, ToSchema)]
pub struct DctFormats {
    pub protocol: FormatProtocol,
    pub action: FormatAction,
}

impl Serialize for DctFormats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let protocol = match self.protocol {
            FormatProtocol::FiwareContextBroker => "fiware_context_broker",
            FormatProtocol::Http => "http",
            FormatProtocol::Quic => "quic",
            FormatProtocol::Grpc => "grpc",
            FormatProtocol::Kafka => "kafka",
            FormatProtocol::Mqtt => "mqtt",
            FormatProtocol::S3 => "s3",
        };
        let action = match self.action {
            FormatAction::Push => "push",
            FormatAction::Pull => "pull",
        };
        let combined = format!("{}+{}", protocol, action);
        serializer.serialize_str(&combined)
    }
}

impl<'de> Deserialize<'de> for DctFormats {
    fn deserialize<D>(deserializer: D) -> Result<DctFormats, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = String::deserialize(deserializer)?;
        println!("{}", v);
        let parts: Vec<&str> = v.split("+").collect();
        if parts.len() != 2 {
            println!("estas petando aquí...");
            return Err(Error::custom("Expected string in format PROTOCOL_ACTION"));
        }
        let protocol = match parts[0].to_lowercase().as_str() {
            "fiware_context_broker" => FormatProtocol::FiwareContextBroker,
            "fiware" => FormatProtocol::FiwareContextBroker,
            "cb" => FormatProtocol::FiwareContextBroker,
            "http" => FormatProtocol::Http,
            "quic" => FormatProtocol::Quic,
            "kafka" => FormatProtocol::Kafka,
            _ => return Err(Error::custom("expected a correct protocol")),
        };
        let action = match parts[1].to_lowercase().as_str() {
            "push" => FormatAction::Push,
            "pull" => FormatAction::Pull,
            _ => return Err(Error::custom("expected a correct protocol")),
        };
        Ok(DctFormats { protocol, action })
    }
}
