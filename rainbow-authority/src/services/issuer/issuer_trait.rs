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
use crate::data::entities::{issuing, request};
use crate::types::issuing::{AuthServerMetadata, IssuerMetadata, IssuingToken, VCCredOffer};
use serde_json::Value;

pub trait IssuerServiceTrait: Send + Sync + 'static {
    fn start_vci(&self, req_model: &request::Model) -> issuing::NewModel;
    fn generate_issuing_uri(&self, id: &str) -> String;
    fn get_cred_offer_data(&self, model: &issuing::Model) -> anyhow::Result<VCCredOffer>;
    fn get_issuer_data(&self) -> IssuerMetadata;
    fn get_oauth_server_data(&self) -> AuthServerMetadata;
    fn get_token(&self) -> IssuingToken;
    fn issue_cred(&self) -> anyhow::Result<Value>;
}
