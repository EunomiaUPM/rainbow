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
use crate::protocol::contract::CNValidate;
use crate::utils::get_urn;
use anyhow::bail;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use urn::Urn;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OdrlTypes {
    #[serde(rename = "dspace:Offer")]
    Offer,
    #[serde(rename = "dspace:Agreement")]
    Agreement,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OfferTypes {
    MessageOffer(OdrlMessageOffer),
    Offer(OdrlOffer),
    Other(Value),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OdrlMessageOffer {
    // PolicyClass
    #[serde(rename = "@id")]
    pub id: Urn,
    #[serde(rename = "odrl:profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<OdrlProfile>,
    #[serde(rename = "odrl:permission")]
    pub permission: Option<Vec<OdrlPermission>>, // anyof
    #[serde(rename = "odrl:obligation")]
    pub obligation: Option<Vec<OdrlObligation>>,
    // MessageOffer
    #[serde(rename = "@type")]
    pub _type: OdrlTypes,
    #[serde(rename = "odrl:prohibition")]
    pub prohibition: Option<Vec<OdrlObligation>>, // anyof
}

impl CNValidate for OdrlMessageOffer {
    fn validate(&self) -> anyhow::Result<()> {
        //

        // Validate any of permission or prohibition
        match (&self.permission, &self.prohibition) {
            (Some(pr), Some(ph)) => {
                bail!("Either one of dspace:offer.permission or dspace:offer.prohibition must be present".to_string(),)
            }
            (None, None) => {
                bail!("At least one of dspace:offer.permission or dspace:offer.prohibition must be present".to_string(),)
            }
            _ => {}
        };
        Ok(())
    }
}

/// Offer is PolicyClass + MessageOffer - Offer
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OdrlOffer {
    // PolicyClass
    #[serde(rename = "@id")]
    pub id: Urn,
    #[serde(rename = "odrl:profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<OdrlProfile>,
    #[serde(rename = "odrl:permission")]
    pub permission: Option<Vec<OdrlPermission>>, // anyof
    #[serde(rename = "odrl:obligation")]
    pub obligation: Option<Vec<OdrlObligation>>,
    // MessageOffer
    #[serde(rename = "@type")]
    pub _type: OdrlTypes,
    #[serde(rename = "odrl:prohibition")]
    pub prohibition: Option<Vec<OdrlObligation>>, // anyof
    // Offer
    #[serde(rename = "odrl:target")]
    pub target: Option<Urn>,
}

impl Default for OdrlOffer {
    fn default() -> Self {
        OdrlOffer {
            id: get_urn(None),
            profile: None,
            permission: None,
            obligation: None,
            _type: OdrlTypes::Offer,
            prohibition: None,
            target: None,
        }
    }
}

impl CNValidate for OdrlOffer {
    fn validate(&self) -> anyhow::Result<()> {
        // Validate any of permission or prohibition
        match (&self.permission, &self.prohibition) {
            (Some(pr), Some(ph)) => {
                bail!("Either one of dspace:offer.permission or dspace:offer.prohibition must be present".to_string(),)
            }
            (None, None) => {
                bail!("At least one of dspace:offer.permission or dspace:offer.prohibition must be present".to_string(),)
            }
            _ => {}
        };
        Ok(())
    }
}

/// Offer is PolicyClass + Agreement
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OdrlAgreement {
    // PolicyClass
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "odrl:profile")]
    pub profile: Option<OdrlProfile>,
    #[serde(rename = "odrl:permission")]
    pub permission: Option<Vec<OdrlPermission>>, // anyof
    #[serde(rename = "odrl:obligation")]
    pub obligation: Option<Vec<OdrlObligation>>,
    // Agreement
    #[serde(rename = "@type")]
    pub _type: OdrlTypes,
    #[serde(rename = "odrl:target")]
    pub target: Urn,
    #[serde(rename = "odrl:assigner")]
    pub assigner: Urn,
    #[serde(rename = "odrl:assignee")]
    pub assignee: Urn,
    #[serde(rename = "odrl:timestamp")]
    pub timestamp: Option<String>,
    #[serde(rename = "odrl:prohibition")]
    pub prohibition: Option<Vec<OdrlObligation>>, // anyof
}

impl Default for OdrlAgreement {
    fn default() -> OdrlAgreement {
        Self {
            id: get_urn(None).to_string(),
            profile: None,
            permission: None,
            obligation: None,
            _type: OdrlTypes::Agreement,
            target: get_urn(None),
            assigner: get_urn(None),
            assignee: get_urn(None),
            timestamp: None,
            prohibition: None,
        }
    }
}

impl CNValidate for OdrlAgreement {
    fn validate(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

// ODRL Profile type
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OdrlProfile {
    Single(String),
    Multiple(Vec<String>),
}

/// OdrlPermission
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OdrlPermission {
    #[serde(rename = "odrl:action")]
    pub action: OdrlAction,
    #[serde(rename = "odrl:constraint")]
    pub constraint: Option<Vec<OdrlConstraint>>,
    #[serde(rename = "odrl:duty")]
    pub duty: Option<OdrlDuty>,
}

/// OdrlDuty
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OdrlDuty {
    #[serde(rename = "odrl:action")]
    pub action: OdrlAction,
    #[serde(rename = "odrl:constraint")]
    pub constraint: Option<Vec<OdrlConstraint>>,
}

/// OdrlObligation
pub type OdrlObligation = OdrlDuty;

/// OdrlAction
pub type OdrlAction = String;

/// OdrlConstraint
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OdrlConstraint {
    Atomic(OdrlAtomicConstraint),
    Logical(OdrlLogicalConstraint),
}

/// LogicalConstraint permite una de las siguientes propiedades: "and", "andSequence", "or" o "xone".
/// Se usan Option para cada una;
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OdrlLogicalConstraint {
    #[serde(rename = "odrl:and")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<OdrlConstraint>>,
    #[serde(rename = "andSequence", skip_serializing_if = "Option::is_none")]
    pub and_sequence: Option<Vec<OdrlConstraint>>,
    #[serde(rename = "odrl:or")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<OdrlConstraint>>,
    #[serde(rename = "odrl:xone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xone: Option<Vec<OdrlConstraint>>,
}

/// the rule that exactly one must be present is validated externally.
/// let constraint: LogicalConstraint = serde_json::from_str(json_data)?;
/// constraint.validate()?; // if it fails, an error is returned.
impl OdrlLogicalConstraint {
    pub fn validate(&self) -> anyhow::Result<()> {
        let count = self.and.is_some() as usize
            + self.and_sequence.is_some() as usize
            + self.or.is_some() as usize
            + self.xone.is_some() as usize;
        if count != 1 {
            bail!("Exactly one of 'and', 'andSequence', 'or' or 'xone' must be present, found {}", count)
        } else {
            Ok(())
        }
    }
}

// AtomicConstraint defines the three required fields: rightOperand, leftOperand and operator.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OdrlAtomicConstraint {
    #[serde(rename = "odrl:rightOperand")]
    pub right_operand: OdrlRightOperand,
    #[serde(rename = "odrl:leftOperand")]
    pub left_operand: OdrlLeftOperand,
    #[serde(rename = "odrl:operator")]
    pub operator: Operator,
}

// Operator is defined as an enum with allowed values.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub enum Operator {
    #[serde(rename = "odrl:eq")]
    Eq,
    #[serde(rename = "odrl:gt")]
    Gt,
    #[serde(rename = "odrl:gteq")]
    Gteq,
    #[serde(rename = "odrl:lteq")]
    Lteq,
    #[serde(rename = "odrl:hasPart")]
    HasPart,
    #[serde(rename = "odrl:isA")]
    IsA,
    #[serde(rename = "odrl:isAllOf")]
    IsAllOf,
    #[serde(rename = "odrl:isAnyOf")]
    IsAnyOf,
    #[serde(rename = "odrl:isNoneOf")]
    IsNoneOf,
    #[serde(rename = "odrl:isPartOf")]
    IsPartOf,
    #[serde(rename = "odrl:lt")]
    Lt,
    #[serde(rename = "odrl:termLteq")]
    TermLteq,
    #[serde(rename = "odrl:neq")]
    Neq,
}

// RightOperand is defined to accept string, object or array.
// serde_json::Value is used to allow this variability.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OdrlRightOperand {
    Str(String),
    Object(serde_json::Map<String, Value>),
    Array(Vec<Value>),
}

// LeftOperand es un string.
pub type OdrlLeftOperand = String;
