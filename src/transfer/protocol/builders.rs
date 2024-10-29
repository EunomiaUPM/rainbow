use crate::transfer::protocol::formats::{DctFormats, FormatAction, FormatProtocol};
use crate::transfer::protocol::messages::{
    DataAddress, TransferCompletionMessage, TransferError, TransferMessageTypes,
    TransferProcessMessage, TransferRequestMessage, TransferStartMessage, TransferState,
    TransferSuspensionMessage, TransferTerminationMessage, TRANSFER_CONTEXT,
};

impl TransferRequestMessage {
    pub fn builder() -> TransferRequestMessageBuilder {
        TransferRequestMessageBuilder::new()
    }
}

pub struct TransferRequestMessageBuilder {
    inner: TransferRequestMessage,
}

impl TransferRequestMessageBuilder {
    pub fn new() -> Self {
        TransferRequestMessageBuilder {
            inner: TransferRequestMessage {
                context: TRANSFER_CONTEXT.to_string(),
                _type: TransferMessageTypes::TransferRequestMessage.to_string(),
                consumer_pid: "".to_string(),
                agreement_id: "".to_string(),
                format: DctFormats {
                    protocol: FormatProtocol::Http,
                    action: FormatAction::Push,
                },
                callback_address: "".to_string(),
                data_address: None,
            },
        }
    }
    pub fn consumer_pid(mut self, consumer_pid: &str) -> Self {
        self.inner.consumer_pid = consumer_pid.to_string();
        self
    }
    pub fn agreement_id(mut self, agreement_id: &str) -> Self {
        self.inner.agreement_id = agreement_id.to_string();
        self
    }
    pub fn callback_address(mut self, callback_address: &str) -> Self {
        self.inner.callback_address = callback_address.to_string();
        self
    }
    pub fn data_address(mut self, data_address: Option<DataAddress>) -> Self {
        self.inner.data_address = data_address;
        self
    }
    pub fn format(mut self, format_action: FormatAction, format_protocol: FormatProtocol) -> Self {
        self.inner.format = DctFormats {
            protocol: format_protocol,
            action: format_action,
        };
        self
    }
    pub fn build(self) -> TransferRequestMessage {
        self.inner
    }
}

impl TransferStartMessage {
    pub fn builder() -> TransferStartMessageBuilder {
        TransferStartMessageBuilder::new()
    }
}

pub struct TransferStartMessageBuilder {
    inner: TransferStartMessage,
}

impl TransferStartMessageBuilder {
    pub fn new() -> Self {
        TransferStartMessageBuilder {
            inner: TransferStartMessage {
                context: TRANSFER_CONTEXT.to_string(),
                _type: TransferMessageTypes::TransferRequestMessage.to_string(),
                provider_pid: "".to_string(),
                consumer_pid: "".to_string(),
                data_address: None,
            },
        }
    }
    pub fn provider_pid(mut self, provider_pid: &str) -> Self {
        self.inner.provider_pid = provider_pid.to_string();
        self
    }
    pub fn consumer_pid(mut self, consumer_pid: &str) -> Self {
        self.inner.consumer_pid = consumer_pid.to_string();
        self
    }
    pub fn data_address(mut self, data_address: Option<DataAddress>) -> Self {
        self.inner.data_address = data_address;
        self
    }
    pub fn build(self) -> TransferStartMessage {
        self.inner
    }
}

impl TransferSuspensionMessage {
    pub fn builder() -> TransferSuspensionMessageBuilder {
        TransferSuspensionMessageBuilder::new()
    }
}

pub struct TransferSuspensionMessageBuilder {
    inner: TransferSuspensionMessage,
}

impl TransferSuspensionMessageBuilder {
    pub fn new() -> Self {
        TransferSuspensionMessageBuilder {
            inner: TransferSuspensionMessage {
                context: TRANSFER_CONTEXT.to_string(),
                _type: TransferMessageTypes::TransferSuspensionMessage.to_string(),
                provider_pid: "".to_string(),
                consumer_pid: "".to_string(),
                code: "400".to_string(),
                reason: vec![],
            },
        }
    }
    pub fn provider_pid(mut self, provider_pid: &str) -> Self {
        self.inner.provider_pid = provider_pid.to_string();
        self
    }
    pub fn consumer_pid(mut self, consumer_pid: &str) -> Self {
        self.inner.consumer_pid = consumer_pid.to_string();
        self
    }
    pub fn code(mut self, code: &str) -> Self {
        self.inner.code = code.to_string();
        self
    }
    pub fn add_reason(mut self, add_reason: &str) -> Self {
        self.inner.reason.push(add_reason.to_string());
        self
    }
    pub fn reason(mut self, reason: Vec<&str>) -> Self {
        self.inner.reason = reason.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn build(self) -> TransferSuspensionMessage {
        self.inner
    }
}

impl TransferCompletionMessage {
    pub fn builder() -> TransferCompletionMessageBuilder {
        TransferCompletionMessageBuilder::new()
    }
}

pub struct TransferCompletionMessageBuilder {
    inner: TransferCompletionMessage,
}

impl TransferCompletionMessageBuilder {
    pub fn new() -> Self {
        TransferCompletionMessageBuilder {
            inner: TransferCompletionMessage {
                context: TRANSFER_CONTEXT.to_string(),
                _type: TransferMessageTypes::TransferCompletionMessage.to_string(),
                provider_pid: "".to_string(),
                consumer_pid: "".to_string(),
            },
        }
    }
    pub fn provider_pid(mut self, provider_pid: &str) -> Self {
        self.inner.provider_pid = provider_pid.to_string();
        self
    }
    pub fn consumer_pid(mut self, consumer_pid: &str) -> Self {
        self.inner.consumer_pid = consumer_pid.to_string();
        self
    }
    pub fn build(self) -> TransferCompletionMessage {
        self.inner
    }
}

impl TransferTerminationMessage {
    pub fn builder() -> TransferSuspensionMessageBuilder {
        TransferSuspensionMessageBuilder::new()
    }
}

pub struct TransferTerminationMessageBuilder {
    inner: TransferTerminationMessage,
}

impl TransferTerminationMessageBuilder {
    pub fn new() -> Self {
        TransferTerminationMessageBuilder {
            inner: TransferTerminationMessage {
                context: TRANSFER_CONTEXT.to_string(),
                _type: TransferMessageTypes::TransferSuspensionMessage.to_string(),
                provider_pid: "".to_string(),
                consumer_pid: "".to_string(),
                code: "400".to_string(),
                reason: vec![],
            },
        }
    }
    pub fn provider_pid(mut self, provider_pid: &str) -> Self {
        self.inner.provider_pid = provider_pid.to_string();
        self
    }
    pub fn consumer_pid(mut self, consumer_pid: &str) -> Self {
        self.inner.consumer_pid = consumer_pid.to_string();
        self
    }
    pub fn code(mut self, code: &str) -> Self {
        self.inner.code = code.to_string();
        self
    }
    pub fn add_reason(mut self, add_reason: &str) -> Self {
        self.inner.reason.push(add_reason.to_string());
        self
    }
    pub fn reason(mut self, reason: Vec<&str>) -> Self {
        self.inner.reason = reason.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn build(self) -> TransferTerminationMessage {
        self.inner
    }
}

impl TransferProcessMessage {
    pub fn builder() -> TransferProcessMessageBuilder {
        TransferProcessMessageBuilder::new()
    }
}

pub struct TransferProcessMessageBuilder {
    inner: TransferProcessMessage,
}

impl TransferProcessMessageBuilder {
    pub fn new() -> Self {
        TransferProcessMessageBuilder {
            inner: TransferProcessMessage {
                context: TRANSFER_CONTEXT.to_string(),
                _type: TransferMessageTypes::TransferProcessMessage.to_string(),
                provider_pid: "".to_string(),
                consumer_pid: "".to_string(),
                state: TransferState::REQUESTED,
            },
        }
    }
    pub fn provider_pid(mut self, provider_pid: &str) -> Self {
        self.inner.provider_pid = provider_pid.to_string();
        self
    }
    pub fn consumer_pid(mut self, consumer_pid: &str) -> Self {
        self.inner.consumer_pid = consumer_pid.to_string();
        self
    }
    pub fn state(mut self, state: TransferState) -> Self {
        self.inner.state = state;
        self
    }
    pub fn build(self) -> TransferProcessMessage {
        self.inner
    }
}

impl TransferError {
    pub fn builder() -> TransferErrorBuilder {
        TransferErrorBuilder::new()
    }
}

pub struct TransferErrorBuilder {
    inner: TransferError,
}

impl TransferErrorBuilder {
    pub fn new() -> Self {
        TransferErrorBuilder {
            inner: TransferError {
                context: TRANSFER_CONTEXT.to_string(),
                _type: TransferMessageTypes::TransferError.to_string(),
                provider_pid: Option::from("".to_string()),
                consumer_pid: "".to_string(),
                code: "".to_string(),
                reason: vec![],
            },
        }
    }
    pub fn provider_pid(mut self, provider_pid: &str) -> Self {
        self.inner.provider_pid = Option::from(provider_pid.to_string());
        self
    }
    pub fn consumer_pid(mut self, consumer_pid: &str) -> Self {
        self.inner.consumer_pid = consumer_pid.to_string();
        self
    }
    pub fn code(mut self, code: &str) -> Self {
        self.inner.code = code.to_string();
        self
    }
    pub fn add_reason(mut self, add_reason: &str) -> Self {
        self.inner.reason.push(add_reason.to_string());
        self
    }
    pub fn reason(mut self, reason: Vec<&str>) -> Self {
        self.inner.reason = reason.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn build(self) -> TransferError {
        self.inner
    }
}
