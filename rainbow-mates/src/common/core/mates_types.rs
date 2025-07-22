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


use rainbow_common::mates::Mates;
use rainbow_common::utils::get_urn;
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapMateRequest {
    pub participant_slug: Option<String>,
    pub participant_type: String,
    pub base_url: String,
}

impl Into<Mates> for BootstrapMateRequest {
    fn into(self) -> Mates {
        Mates {
            participant_id: get_urn(None).to_string(),
            participant_slug: self.participant_slug.unwrap_or(self.participant_type.clone()),
            participant_type: self.participant_type,
            base_url: Some(self.base_url),
            token: None,
            token_actions: None,
            saved_at: chrono::Utc::now().naive_utc(),
            last_interaction: chrono::Utc::now().naive_utc(),
            is_me: true,
        }
    }
}