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

use super::m20241111_000001_catalog::CatalogCatalogs;
use super::m20241111_000002_dataset::CatalogDatasets;
use super::m20241111_000003_distribution::CatalogDistributions;
use super::m20241111_000004_dataservice::CatalogDataServices;
use super::m20241111_000005_odrl_offers::CatalogODRLOffers;

use super::m20250718_000001_catalogrecord::CatalogRecord;
use super::m20250718_000002_datasetseries::DatasetSeries;

use super::m20250721_000003_keywords::Keywords;
use super::m20250721_000002_themes::Themes;
use super::m20250721_000001_resources::Resources;
use super::m20250721_000004_relations::Relations;
use super::m20250721_000005_qualifiedrelations::QualifiedRelations;
use super::m20250721_000006_references::References;

use rainbow_common::protocol::catalog::distribution_definition::Distribution;
use sea_orm_migration::{manager, prelude::*};

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250722_000001_foreign_keys"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_catalog_ordl_policy")
                    .from(CatalogCatalogs::Table, CatalogCatalogs::OrdlHasPolicy)
                    .to(CatalogODRLOffers::Table, CatalogODRLOffers::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_catalog_current_version")
                    .from(CatalogCatalogs::Table, CatalogCatalogs::DcatHasCurrentVersion)
                    .to(CatalogCatalogs::Table, CatalogCatalogs::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_catalog_previous_version")
                    .from(CatalogCatalogs::Table, CatalogCatalogs::DcatPreviousVersion)
                    .to(CatalogCatalogs::Table, CatalogCatalogs::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_catalog_replaces")
                    .from(CatalogCatalogs::Table, CatalogCatalogs::DctReplaces)
                    .to(CatalogCatalogs::Table, CatalogCatalogs::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_catalog_first")
                    .from(CatalogCatalogs::Table, CatalogCatalogs::DcatFirst)
                    .to(CatalogCatalogs::Table, CatalogCatalogs::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_catalog_last")
                    .from(CatalogCatalogs::Table, CatalogCatalogs::DcatLast)
                    .to(CatalogCatalogs::Table, CatalogCatalogs::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_catalog_prev")
                    .from(CatalogCatalogs::Table, CatalogCatalogs::DcatPrev)
                    .to(CatalogCatalogs::Table, CatalogCatalogs::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataset_catalog")
                    .from(CatalogDatasets::Table, CatalogDatasets::CatalogId)
                    .to(CatalogCatalogs::Table, CatalogCatalogs::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataset_ordl_policy")
                    .from(CatalogDatasets::Table, CatalogDatasets::OrdlHasPolicy)
                    .to(CatalogODRLOffers::Table, CatalogODRLOffers::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataset_current_version")
                    .from(CatalogDatasets::Table, CatalogDatasets::DcatHasCurrentVersion)
                    .to(CatalogDatasets::Table, CatalogDatasets::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataset_previous_version")
                    .from(CatalogDatasets::Table, CatalogDatasets::DcatPreviousVersion)
                    .to(CatalogDatasets::Table, CatalogDatasets::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataset_replaces")
                    .from(CatalogDatasets::Table, CatalogDatasets::DctReplaces)
                    .to(CatalogDatasets::Table, CatalogDatasets::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataset_first")
                    .from(CatalogDatasets::Table, CatalogDatasets::DcatFirst)
                    .to(CatalogDatasets::Table, CatalogDatasets::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataset_last")
                    .from(CatalogDatasets::Table, CatalogDatasets::DcatLast)
                    .to(CatalogDatasets::Table, CatalogDatasets::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataset_prev")
                    .from(CatalogDatasets::Table, CatalogDatasets::DcatPrev)
                    .to(CatalogDatasets::Table, CatalogDatasets::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_distribution_dataset")
                    .from(CatalogDistributions::Table, CatalogDistributions::DatasetId)
                    .to(CatalogDatasets::Table, CatalogDatasets::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_distribution_access_service")
                    .from(CatalogDistributions::Table, CatalogDistributions::AccessServiceId)
                    .to(CatalogDataServices::Table, CatalogDataServices::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_distribution_dataset_series")
                    .from(CatalogDistributions::Table, CatalogDistributions::DatasetSeriesId)
                    .to(DatasetSeries::Table, DatasetSeries::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
                )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_distribution_ordl_policy")
                    .from(CatalogDistributions::Table, CatalogDistributions::OrdlHasPolicy)
                    .to(CatalogODRLOffers::Table, CatalogODRLOffers::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataservice_catalog")
                    .from(CatalogDataServices::Table, CatalogDataServices::CatalogId)
                    .to(CatalogCatalogs::Table, CatalogCatalogs::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataservice_ordl_policy")
                    .from(CatalogDataServices::Table, CatalogDataServices::OrdlHasPolicy)
                    .to(CatalogODRLOffers::Table, CatalogODRLOffers::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataservice_dcat_serves_dataset")
                    .from(CatalogDataServices::Table, CatalogDataServices::DcatServesDataset)
                    .to(CatalogDatasets::Table, CatalogDatasets::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataservice_dcat_replaces")
                    .from(CatalogDataServices::Table, CatalogDataServices::DcatReplaces)
                    .to(CatalogDataServices::Table, CatalogDataServices::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataservice_dcat_previous_version")
                    .from(CatalogDataServices::Table, CatalogDataServices::DcatPreviousVersion)
                    .to(CatalogDataServices::Table, CatalogDataServices::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_first_dataservice")
                    .from(CatalogDataServices::Table, CatalogDataServices::DcatFirst)
                    .to(CatalogDataServices::Table, CatalogDataServices::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_last_dataservice")
                    .from(CatalogDataServices::Table, CatalogDataServices::DcatLast)
                    .to(CatalogDataServices::Table, CatalogDataServices::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_prev_dataservice")
                    .from(CatalogDataServices::Table, CatalogDataServices::DcatPrev)
                    .to(CatalogDataServices::Table, CatalogDataServices::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_catalog_record_catalog")
                    .from(CatalogRecord::Table, CatalogRecord::DcatCatalog)
                    .to(CatalogCatalogs::Table, CatalogCatalogs::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_dataset_series_ordl_policy")
                    .from(DatasetSeries::Table, DatasetSeries::OrdlHasPolicy)
                    .to(CatalogODRLOffers::Table, CatalogODRLOffers::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_first_dataset")
                    .from(DatasetSeries::Table, DatasetSeries::DcatFirst)
                    .to(DatasetSeries::Table, DatasetSeries::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_last_dataset")
                    .from(DatasetSeries::Table, DatasetSeries::DcatLast)
                    .to(DatasetSeries::Table, DatasetSeries::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_prev_dataset")
                    .from(DatasetSeries::Table, DatasetSeries::DcatPrev)
                    .to(DatasetSeries::Table, DatasetSeries::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_themes_resource_id")
                    .from(Themes::Table, Themes::DcatResource)
                    .to(Resources::Table, Resources::ResourceId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_keywords_resource_id")
                    .from(Keywords::Table, Keywords::DcatResource)
                    .to(Resources::Table, Resources::ResourceId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_relations_resource1")
                    .from(Relations::Table, Relations::DcatResource1)
                    .to(Resources::Table, Resources::ResourceId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_relations_resource2")
                    .from(Relations::Table, Relations::DcatResource2)
                    .to(Resources::Table, Resources::ResourceId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_relations_resource1")
                    .from(QualifiedRelations::Table, QualifiedRelations::DcatResource1)
                    .to(Resources::Table, Resources::ResourceId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_relations_resource2")
                    .from(QualifiedRelations::Table, QualifiedRelations::DcatResource2)
                    .to(Resources::Table, Resources::ResourceId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_references_resource")
                    .from(References::Table, References::ReferencedResourceId)
                    .to(Resources::Table, Resources::ResourceId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned()
            )
            .await?;
        Ok(())
    }




    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_catalog_ordl_policy")
                    .table(CatalogCatalogs::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_catalog_current_version")
                    .table(CatalogCatalogs::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_catalog_previous_version")
                    .table(CatalogCatalogs::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_catalog_replaces")
                    .table(CatalogCatalogs::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_catalog_first")
                    .table(CatalogCatalogs::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_catalog_last")
                    .table(CatalogCatalogs::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_catalog_prev")
                    .table(CatalogCatalogs::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataset_catalog")
                    .table(CatalogDatasets::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataset_ordl_policy")
                    .table(CatalogDatasets::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataset_current_version")
                    .table(CatalogDatasets::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataset_previous_version")
                    .table(CatalogDatasets::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataset_replaces")
                    .table(CatalogDatasets::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataset_first")
                    .table(CatalogDatasets::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataset_last")
                    .table(CatalogDatasets::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataset_prev")
                    .table(CatalogDatasets::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_distribution_dataset")
                    .table(CatalogDistributions::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_distribution_access_service")
                    .table(CatalogDistributions::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_distribution_dataset_series")
                    .table(CatalogDistributions::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_distribution_ordl_policy")
                    .table(CatalogDistributions::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataservice_catalog")
                    .table(CatalogDataServices::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataservice_ordl_policy")
                    .table(CatalogDataServices::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataservice_dcat_serves_dataset")
                    .table(CatalogDataServices::Table) // Specify the table from which the FK is dropped
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataservice_dcat_replaces")
                    .table(CatalogDataServices::Table)
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataservice_dcat_previous_version")
                    .table(CatalogDataServices::Table)
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_first_dataservice")
                    .table(CatalogDataServices::Table)
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_last_dataservice")
                    .table(CatalogDataServices::Table)
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_prev_dataservice")
                    .table(CatalogDataServices::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_catalog_record_catalog")
                    .table(CatalogCatalogs::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_dataset_series_ordl_policy")
                    .table(CatalogODRLOffers::Table)
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_first_dataset")
                    .table(DatasetSeries::Table)
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_last_dataset")
                    .table(DatasetSeries::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_prev_dataset")
                    .table(DatasetSeries::Table)
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_themes_resource_id")
                    .table(Resources::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_keywords_resource_id")
                    .table(Resources::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_relations_resource1")
                    .table(Resources::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_relations_resource2")
                    .table(Resources::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_relations_resource1")
                    .table(Resources::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_relations_resource2")
                    .table(Resources::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_references_resource")
                    .table(Resources::Table)
                    .to_owned()
            )
            .await?;
        Ok(())
    }
}
