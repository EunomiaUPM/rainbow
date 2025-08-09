/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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

use crate::provider::core::ds_protocol::ds_protocol_errors::DSProtocolCatalogErrors;
use crate::provider::core::rainbow_entities::rainbow_catalog_err::CatalogError;
use crate::provider::core::rainbow_entities::rainbow_catalog_types::{NewKeywordRequest, NewThemeRequest};
use crate::provider::core::rainbow_entities::RainbowCatalogKeywordsThemesTrait;
use anyhow::bail;
use axum::async_trait;
use rainbow_common::protocol::catalog::catalog_definition::Catalog;
use rainbow_common::protocol::catalog::EntityTypes;
use rainbow_common::protocol::contract::contract_odrl::OdrlOffer;
use rainbow_common::utils::get_urn_from_string;
use rainbow_db::catalog::entities::keyword;
use rainbow_db::catalog::entities::keyword::Model as key_Model;
use rainbow_db::catalog::entities::theme::Model as theme_Model;
use rainbow_db::catalog::repo::{CatalogRepo, DataServiceRepo, DatasetRepo, DistributionRepo, OdrlOfferRepo, CatalogRecordRepo, KeywordThemesRepo};
use rainbow_events::core::notification::notification_types::{RainbowEventsNotificationBroadcastRequest, RainbowEventsNotificationMessageCategory, RainbowEventsNotificationMessageOperation, RainbowEventsNotificationMessageTypes};
use rainbow_events::core::notification::RainbowEventsNotificationTrait;
use serde_json::{json, to_value};
use std::sync::Arc;
use urn::Urn;

pub struct RainbowCatalogKeywordThemeService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + KeywordThemesRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    repo: Arc<T>,
    notification_service: Arc<U>,
}

impl<T, U> RainbowCatalogKeywordThemeService<T, U>
where 
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + KeywordThemesRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait + Send + Sync,
{
    pub fn new(repo: Arc<T>, notification_service: Arc<U>) -> Self {
        Self {repo, notification_service}
    }
}

#[async_trait]
impl<T, U> RainbowCatalogKeywordsThemesTrait for RainbowCatalogKeywordThemeService<T, U>
where
    T: CatalogRepo + DatasetRepo + DistributionRepo + DataServiceRepo + OdrlOfferRepo + CatalogRecordRepo + KeywordThemesRepo + Send + Sync + 'static,
    U: RainbowEventsNotificationTrait  + Send + Sync,
{
    async fn get_all_keywords(&self) -> anyhow::Result<Vec<key_Model>>  {
        let keywords = self.repo
            .get_all_keywords()
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(keywords)
    }
    async fn post_keyword(&self, input: NewKeywordRequest) -> anyhow::Result<key_Model> {
        let keyword = self.repo 
            .create_keyword(input.into())
            .await
            .map_err(CatalogError::DbErr)?;
        Ok(keyword)
    }
    async fn delete_keyword(&self, id: Urn) -> anyhow::Result<()> {
        let _ = self.repo   
            .delete_keyword(id.clone())
            .await
            .map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogRecordNotfound => {
                    CatalogError::NotFound { id: id.clone(), entity: EntityTypes::Keyword.to_string()}
                }
                _ => CatalogError::DbErr(err),
            })?;
        Ok(())
    }
    async fn get_all_themes(&self) -> anyhow::Result<Vec<theme_Model>>  {
        let keywords = self.repo
            .get_all_themes()
            .await
            .map_err(DSProtocolCatalogErrors::DbErr)?;
        Ok(keywords)
    }
    async fn post_theme(&self, input: NewThemeRequest) -> anyhow::Result<theme_Model> {
        let keyword = self.repo 
            .create_theme(input.into())
            .await
            .map_err(CatalogError::DbErr)?;
        Ok(keyword)
    }
    async fn delete_theme(&self, id: Urn) -> anyhow::Result<()> {
        let _ = self.repo   
            .delete_theme(id.clone())
            .await
            .map_err(|err| match err {
                rainbow_db::catalog::repo::CatalogRepoErrors::CatalogRecordNotfound => {
                    CatalogError::NotFound { id: id.clone(), entity: EntityTypes::Theme.to_string()}
                }
                _ => CatalogError::DbErr(err),
            })?;
        Ok(())
    }
}