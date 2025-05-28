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

pub mod entities;
pub mod migrations;
pub mod repo;
pub mod entity_mappers;

// impl TryFrom<dataset::Model> for Dataset {
//     type Error = anyhow::Error;
//
//     fn try_from(dataset_model: dataset::Model) -> Result<Self, Self::Error> {
//         Ok(Dataset {
//             context: ContextField::default(),
//             _type: "dcat:Dataset".to_string(),
//             id: dataset_model.id.to_string(),
//             dcat: DatasetDcatDeclaration { theme: "".to_string(), keyword: "".to_string() },
//             dct: DatasetDctDeclaration {
//                 conforms_to: dataset_model.dct_conforms_to,
//                 creator: dataset_model.dct_creator,
//                 identifier: dataset_model
//                     .dct_identifier
//                     .unwrap_or_else(|| dataset_model.id.to_string()),
//                 issued: dataset_model.dct_issued,
//                 modified: dataset_model.dct_modified,
//                 title: dataset_model.dct_title,
//                 description: vec![],
//             },
//             odrl_offer: Value::default(),
//             extra_fields: Value::default(),
//             distribution: vec![],
//         })
//     }
// }
//
// impl TryFrom<catalog::Model> for Catalog {
//     type Error = ();
//
//     fn try_from(catalog_model: catalog::Model) -> anyhow::Result<Self, Self::Error> {
//         let catalog_out = Catalog {
//             context: ContextField::default(),
//             _type: "dcat:Catalog".to_string(),
//             id: catalog_model.id.to_string(),
//             foaf: CatalogFoafDeclaration { homepage: catalog_model.foaf_home_page },
//             dcat: CatalogDcatDeclaration {
//                 // Array of strings...
//                 theme: "".to_string(),
//                 keyword: "".to_string(),
//             },
//             dct: CatalogDctDeclaration {
//                 conforms_to: catalog_model.dct_conforms_to,
//                 creator: catalog_model.dct_creator,
//                 identifier: catalog_model.id.to_string(),
//                 issued: catalog_model.dct_issued,
//                 modified: catalog_model.dct_modified,
//                 title: catalog_model.dct_title,
//                 description: vec![],
//             },
//             dspace: CatalogDSpaceDeclaration {
//                 participant_id: catalog_model.dspace_participant_id,
//             },
//             odrl_offer: Value::default(),
//             extra_fields: Value::default(),
//             datasets: vec![],
//             data_services: vec![],
//         };
//
//         Ok(catalog_out)
//     }
// }
//
// impl TryFrom<distribution::Model> for Distribution {
//     type Error = anyhow::Error;
//
//     fn try_from(distribution_model: distribution::Model) -> Result<Self, Self::Error> {
//         Ok(Distribution {
//             context: ContextField::default(),
//             _type: "dcat:Distribution".to_string(),
//             id: distribution_model.id.to_string(),
//             dcat: DistributionDcatDeclaration { access_service: None },
//             dct: DistributionDctDeclaration {
//                 identifier: distribution_model.id.to_string(),
//                 issued: distribution_model.dct_issued,
//                 modified: distribution_model.dct_modified,
//                 title: distribution_model.dct_title,
//                 description: vec![],
//             },
//             odrl_offer: Value::default(),
//             extra_fields: Value::default(),
//         })
//     }
// }
//
// impl TryFrom<dataservice::Model> for DataService {
//     type Error = anyhow::Error;
//
//     fn try_from(dataservice_model: dataservice::Model) -> Result<Self, Self::Error> {
//         Ok(DataService {
//             context: ContextField::default(),
//             _type: "dcat:DataService".to_string(),
//             id: dataservice_model.id.to_string(),
//             dcat: DataServiceDcatDeclaration {
//                 theme: "".to_string(),
//                 keyword: "".to_string(),
//                 endpoint_description: dataservice_model
//                     .dcat_endpoint_description
//                     .unwrap_or("".to_string()),
//                 endpoint_url: dataservice_model.dcat_endpoint_url,
//             },
//             dct: DataServiceDctDeclaration {
//                 conforms_to: dataservice_model.dct_conforms_to,
//                 creator: dataservice_model.dct_creator,
//                 identifier: dataservice_model
//                     .dct_identifier
//                     .unwrap_or_else(|| dataservice_model.id.to_string()),
//                 issued: dataservice_model.dct_issued,
//                 modified: dataservice_model.dct_modified,
//                 title: dataservice_model.dct_title,
//                 description: vec![],
//             },
//             odrl_offer: Value::default(),
//             extra_fields: Value::default(),
//         })
//     }
// }
//
// impl TryFrom<odrl_offer::Model> for OdrlOffer {
//     type Error = anyhow::Error;
//
//     fn try_from(value: odrl_offer::Model) -> Result<Self, Self::Error> {
//         let message_value = value.odrl_offer.unwrap_or_default();
//         let message = serde_json::from_value::<OdrlPolicyInfo>(message_value)?;
//         Ok(OdrlOffer {
//             id: get_urn_from_string(&value.id)?,
//             profile: message.profile,
//             permission: message.permission,
//             obligation: message.obligation,
//             _type: OdrlTypes::Offer,
//             prohibition: message.prohibition,
//             target: Option::from(get_urn_from_string(&value.entity)?),
//         })
//     }
// }