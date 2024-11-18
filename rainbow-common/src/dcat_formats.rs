use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use utoipa::ToSchema;

#[derive(Debug, ToSchema)]
pub enum FormatProtocol {
    Http,
    Quic,
    Grpc,
    Kafka,
    Mqtt,
    S3,
}
#[derive(Debug, ToSchema)]
pub enum FormatAction {
    Push,
    Pull,
}

impl PartialEq for FormatAction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FormatAction::Push, FormatAction::Push) => true,
            (FormatAction::Pull, FormatAction::Pull) => true,
            (_, _) => false
        }
    }
}

#[derive(Debug, ToSchema)]
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
