use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub enum FormatProtocol {
    Http,
    Quic,
    Grpc,
    Kafka,
    Mqtt,
}
#[derive(Debug)]
pub enum FormatAction {
    Push,
    Pull,
}
#[derive(Debug)]
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
            FormatProtocol::Http => "HTTP",
            FormatProtocol::Quic => "QUIC",
            FormatProtocol::Grpc => "GRPC",
            FormatProtocol::Kafka => "KAFKA",
            FormatProtocol::Mqtt => "MQTT",
        };
        let action = match self.action {
            FormatAction::Push => "PUSH",
            FormatAction::Pull => "PULL",
        };
        let combined = format!("{}_{}", protocol, action);
        serializer.serialize_str(&combined)
    }
}

impl<'de> Deserialize<'de> for DctFormats {
    fn deserialize<D>(deserializer: D) -> Result<DctFormats, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = String::deserialize(deserializer)?;
        let parts: Vec<&str> = v.split("_").collect();
        if parts.len() != 2 {
            return Err(Error::custom("Expected string in format PROTOCOL_ACTION"));
        }
        let protocol = match parts[0] {
            "HTTP" => FormatProtocol::Http,
            "QUIC" => FormatProtocol::Quic,
            "KAFKA" => FormatProtocol::Kafka,
            _ => return Err(Error::custom("expected a correct protocol")),
        };
        let action = match parts[1] {
            "PUSH" => FormatAction::Push,
            "PULL" => FormatAction::Pull,
            _ => return Err(Error::custom("expected a correct protocol")),
        };
        Ok(DctFormats { protocol, action })
    }
}
