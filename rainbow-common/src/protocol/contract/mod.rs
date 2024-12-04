struct ContractRequestMessage {
    context: String,
    _type: String,
    provider_pid: String,
    consumer_pid: String,
    callback_address: String,
    odrl_offer: String,
}

struct ContractOfferMessage {
    context: String,
    _type: String,
    provider_pid: String,
    callback_address: String,
    odrl_offer: String,
}

struct ContractAgreementMessage {
    context: String,
    _type: String,
    provider_pid: String,
    consumer_pid: String,
    callback_address: String,
    odrl_agreement: String,
}

struct ContractAgreementVerificationMessage {
    context: String,
    _type: String,
    provider_pid: String,
    consumer_pid: String,
}

struct ContractNegotiationEventMessage {
    context: String,
    _type: String,
    provider_pid: String,
    consumer_pid: String,
}