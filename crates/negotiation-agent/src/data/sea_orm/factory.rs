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

//! Repository factory over SeaORM.

use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::data::factory::DataFactory;
use crate::data::repo::agreement::AgreementRepoTrait;
use crate::data::repo::negotiation_message::NegotiationMessageRepoTrait;
use crate::data::repo::negotiation_process::NegotiationProcessRepoTrait;
use crate::data::repo::negotiation_process_identifier::NegotiationIdentifierRepoTrait;
use crate::data::repo::offer::OfferRepoTrait;
use crate::data::sea_orm::repos::agreement::AgreementRepoForSql;
use crate::data::sea_orm::repos::negotiation_message::NegotiationMessageRepoForSql;
use crate::data::sea_orm::repos::negotiation_process::NegotiationProcessRepoForSql;
use crate::data::sea_orm::repos::negotiation_process_identifier::NegotiationProcessIdentifierRepoForSql;
use crate::data::sea_orm::repos::offer::OfferRepoForSql;

/// Repositories over one database connection.
pub struct SeaOrmDataFactory {
    db: DatabaseConnection,
}

impl SeaOrmDataFactory {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl DataFactory for SeaOrmDataFactory {
    fn negotiation_process_repo(&self) -> Arc<dyn NegotiationProcessRepoTrait> {
        Arc::new(NegotiationProcessRepoForSql::new(self.db.clone()))
    }

    fn negotiation_message_repo(&self) -> Arc<dyn NegotiationMessageRepoTrait> {
        Arc::new(NegotiationMessageRepoForSql::new(self.db.clone()))
    }

    fn negotiation_identifier_repo(&self) -> Arc<dyn NegotiationIdentifierRepoTrait> {
        Arc::new(NegotiationProcessIdentifierRepoForSql::new(self.db.clone()))
    }

    fn offer_repo(&self) -> Arc<dyn OfferRepoTrait> {
        Arc::new(OfferRepoForSql::new(self.db.clone()))
    }

    fn agreement_repo(&self) -> Arc<dyn AgreementRepoTrait> {
        Arc::new(AgreementRepoForSql::new(self.db.clone()))
    }
}
