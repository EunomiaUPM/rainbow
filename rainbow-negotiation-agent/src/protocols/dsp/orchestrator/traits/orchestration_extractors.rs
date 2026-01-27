use crate::entities::negotiation_process::NegotiationProcessDto;
use crate::protocols::dsp::protocol_types::{
    NegotiationProcessMessageTrait, NegotiationProcessMessageType, NegotiationProcessState,
};
use async_trait::async_trait;
use rainbow_common::config::types::roles::RoleConfig;
use rainbow_common::errors::{CommonErrors, ErrorLog};

#[async_trait]
pub trait OrchestrationExtractors: Send + Sync {
    fn get_role_from_dto(&self, dto: &NegotiationProcessDto) -> anyhow::Result<RoleConfig> {
        let role = &dto.inner.role;
        let role = role.parse::<RoleConfig>()?;
        Ok(role)
    }

    fn get_state_from_dto(
        &self,
        dto: &NegotiationProcessDto,
    ) -> anyhow::Result<NegotiationProcessState> {
        let state = &dto.inner.state;
        let state = state.parse::<NegotiationProcessState>().map_err(|_e| {
            let err = CommonErrors::parse_new(
                "Something is wrong. Seems this process' state is not protocol compliant",
            );
            log::error!("{}", err.log());
            err
        })?;
        Ok(state)
    }

    fn get_role_from_message_type(
        &self,
        message: &NegotiationProcessMessageType,
    ) -> anyhow::Result<RoleConfig>;
    fn get_state_from_message_type(
        &self,
        message: &NegotiationProcessMessageType,
    ) -> anyhow::Result<NegotiationProcessState> {
        Ok(message.clone().into())
    }
}
