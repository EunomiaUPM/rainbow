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

use rainbow_common::config::services::traits::MonoConfigTrait;
use rainbow_common::config::ApplicationConfig;
use rainbow_db::catalog::repo::sql::CatalogRepoForSql;
use rainbow_db::catalog::repo::{CatalogRepo, NewCatalogModel};
use sea_orm::Database;

pub struct CoreProviderSeeding;

impl CoreProviderSeeding {
    pub async fn run(config: &ApplicationConfig) -> anyhow::Result<()> {
        let db_url = config.get_mono_db();
        let db_connection = Database::connect(db_url).await.expect("Database can't connect");

        let catalog_repo = CatalogRepoForSql::new(db_connection);
        let _ = catalog_repo
            .create_main_catalog(NewCatalogModel {
                id: None,
                foaf_home_page: None,
                dct_conforms_to: None,
                dct_creator: None,
                dct_title: Some("Main Catalog".to_string()),
            })
            .await
            .expect("CatalogRepoForSql creation failed");
        Ok(())
    }
}
