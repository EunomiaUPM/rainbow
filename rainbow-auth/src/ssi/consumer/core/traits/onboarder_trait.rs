/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */
use std::sync::Arc;
use axum::async_trait;
use rainbow_db::auth::common::entities::mates;
use rainbow_db::auth::consumer::factory::AuthConsumerRepoTrait;
use crate::ssi::common::services::callback::CallbackTrait;
use crate::ssi::common::types::gnap::CallbackBody;
use crate::ssi::consumer::services::onboarder::OnboarderTrait;
use crate::ssi::consumer::types::ReachProvider;

#[async_trait]
pub trait CoreOnboarderTrait: Send + Sync + 'static {
    fn onboarder(&self) -> Arc<dyn OnboarderTrait>;
    fn repo(&self) -> Arc<dyn AuthConsumerRepoTrait>;
    fn callback(&self) -> Arc<dyn CallbackTrait>;

    async fn onboard_req(&self, payload: ReachProvider) -> anyhow::Result<String> {
        let (req_model, int_model, token_model) = self.onboarder().start(&payload);
        let mut req_model = self.repo().request_req().create(req_model).await?;
        let mut int_model = self.repo().interaction_req().create(int_model).await?;
        let _token_model = self.repo().token_requirements().create(token_model).await?;
        self.onboarder().send_req(&mut req_model, &mut int_model).await?;
        let _req_model = self.repo().request_req().update(req_model).await?;
        let int_model = self.repo().interaction_req().update(int_model).await?;
        let ver_model = self.onboarder().save_verification(&int_model)?;
        let ver_model = self.repo().verification_req().create(ver_model).await?;
        Ok(ver_model.uri)
    }

    async fn continue_req(&self, id: &str, payload: CallbackBody) -> anyhow::Result<mates::Model> {
        let mut int_model = self.repo().interaction_req().get_by_id(id).await?;
        let result = self.callback().check_callback(&mut int_model, &payload);
        let int_model = self.repo().interaction_req().update(int_model).await?;
        result?;
        let response = self.callback().continue_req(&int_model).await?;
        let mut req_model = self.repo().request_req().get_by_id(id).await?;
        let mate = self.onboarder().manage_res(&mut req_model, response).await?;
        self.repo().request_req().update(req_model).await?;
        let mate = self.repo().mates().force_create(mate).await?;
        Ok(mate)
    }
}