/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::Arc;

use crate::data::factory::OAuthDataFactory;
use crate::data::in_memory::repos::{
    InMemoryAuthCodeRepository, InMemoryClientRepository, InMemoryPatRepository,
    InMemoryRefreshTokenRepository, InMemoryUserRepository,
};
use crate::data::repositories::auth_code::AuthCodeRepository;
use crate::data::repositories::client::ClientRepository;
use crate::data::repositories::pat::PatRepository;
use crate::data::repositories::token::TokenRepository;
use crate::data::repositories::user::UserRepository;

pub(crate) struct InMemoryDataFactory {
    user_repo: Arc<dyn UserRepository>,
    refresh_token_repo: Arc<dyn TokenRepository>,
    client_repo: Arc<dyn ClientRepository>,
    auth_code_repo: Arc<dyn AuthCodeRepository>,
    pat_repo: Arc<dyn PatRepository>,
}

impl InMemoryDataFactory {
    pub fn new() -> Self {
        Self {
            user_repo: Arc::new(InMemoryUserRepository::new()),
            refresh_token_repo: Arc::new(InMemoryRefreshTokenRepository::new()),
            client_repo: Arc::new(InMemoryClientRepository::new()),
            auth_code_repo: Arc::new(InMemoryAuthCodeRepository::new()),
            pat_repo: Arc::new(InMemoryPatRepository::new()),
        }
    }
}

impl OAuthDataFactory for InMemoryDataFactory {
    fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repo.clone()
    }

    fn token_repository(&self) -> Arc<dyn TokenRepository> {
        self.refresh_token_repo.clone()
    }

    fn client_repository(&self) -> Arc<dyn ClientRepository> {
        self.client_repo.clone()
    }

    fn auth_code_repository(&self) -> Arc<dyn AuthCodeRepository> {
        self.auth_code_repo.clone()
    }

    fn pat_repository(&self) -> Arc<dyn PatRepository> {
        self.pat_repo.clone()
    }
}
