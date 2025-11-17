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
use crate::ssi::common::services::vc_requester::VcRequesterTrait;
use rainbow_db::auth::common::entities::req_vc::Model;
use crate::ssi::common::types::entities::{ReachAuthority, ReachMethod};
use axum::async_trait;
use rainbow_db::auth::common::traits::{MatesTrait, ReqInteractionTrait, ReqVcTrait, ReqVerificationTrait};
use std::sync::Arc;
use rainbow_db::auth::common::entities::mates;
use crate::ssi::common::services::callback::CallbackTrait;
use crate::ssi::common::types::gnap::CallbackBody;

#[async_trait]
pub trait CoreVcRequesterTrait: Send + Sync + 'static {
    fn vc_req(&self) -> Arc<dyn VcRequesterTrait>;
    fn vc_req_repo(&self) -> Arc<dyn ReqVcTrait>;
    fn mates_repo(&self) -> Arc<dyn MatesTrait>;
    fn verification_req_repo(&self) -> Arc<dyn ReqVerificationTrait>;
    fn interaction_req_repo(&self) -> Arc<dyn ReqInteractionTrait>;
    fn callback(&self) -> Arc<dyn CallbackTrait>;
    async fn beg_vc(&self, payload: ReachAuthority, method: ReachMethod) -> anyhow::Result<Option<String>> {
        let (vc_model, int_model) = self.vc_req().start(payload);
        let mut vc_model = self.vc_req_repo().create(vc_model).await?;
        let mut int_model = self.interaction_req_repo().create(int_model).await?;
        let uri = self.vc_req().send_req(&mut vc_model, &mut int_model, method).await?;
        let _vc_model = self.vc_req_repo().update(vc_model).await?;
        let int_model = self.interaction_req_repo().update(int_model).await?;
        match uri {
            Some(uri) => {
                let ver_model = self.vc_req().save_ver_data(&uri, &int_model.id)?;
                let _ver_model = self.verification_req_repo().create(ver_model).await?;
                Ok(Some(uri))
            }
            None => Ok(None),
        }
    }
    
    async fn get_all(&self) -> anyhow::Result<Vec<Model>> {
        self.vc_req_repo().get_all(None, None).await
    }
    
    async fn get_by_id(&self, id: String) -> anyhow::Result<Model> {
        self.vc_req_repo().get_by_id(&id).await
    }
    async fn continue_req(&self, id: String, payload: CallbackBody) -> anyhow::Result<mates::Model> {
        let mut int_model = self.interaction_req_repo().get_by_id(&id).await?;
        let result = self.callback().check_callback(&mut int_model, &payload);
        let int_model = self.interaction_req_repo().update(int_model).await?;
        result?;
        let response = self.callback().continue_req(&int_model).await?;
        let mut vc_req_model = self.vc_req_repo().get_by_id(&id).await?;
        let mate = self.vc_req().manage_res(&mut vc_req_model, response).await?;
        self.vc_req_repo().update(vc_req_model).await?;
        let mate = self.mates_repo().force_create(mate).await?;
        Ok(mate)
    }
}
