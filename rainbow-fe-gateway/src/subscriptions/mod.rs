pub mod provider_subscriptions;

pub enum MicroserviceSubscriptionKey {
    Catalog,
    ContractNegotiation,
    TransferControlPlane,
    TransferDataPlane,
    HeartBeat,
}