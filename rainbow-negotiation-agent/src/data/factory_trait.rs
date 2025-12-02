use crate::data::repo_traits::agreement_repo::AgreementRepoTrait;
use crate::data::repo_traits::negotiation_message_repo::NegotiationMessageRepoTrait;
use crate::data::repo_traits::negotiation_process_identifiers_repo::NegotiationIdentifierRepoTrait;
use crate::data::repo_traits::negotiation_process_repo::NegotiationProcessRepoTrait;
use crate::data::repo_traits::offer_repo::OfferRepoTrait;
use std::sync::Arc;

#[mockall::automock]
pub trait NegotiationAgentRepoTrait: Send + Sync + 'static {
    fn get_negotiation_process_repo(&self) -> Arc<dyn NegotiationProcessRepoTrait>;
    fn get_negotiation_message_repo(&self) -> Arc<dyn NegotiationMessageRepoTrait>;
    fn get_negotiation_process_identifiers_repo(&self) -> Arc<dyn NegotiationIdentifierRepoTrait>;
    fn get_offer_repo(&self) -> Arc<dyn OfferRepoTrait>;
    fn get_agreement_repo(&self) -> Arc<dyn AgreementRepoTrait>;
}
