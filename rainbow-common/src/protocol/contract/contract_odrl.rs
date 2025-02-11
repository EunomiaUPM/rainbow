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

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub enum OdrlTypes {
    #[serde(rename = "dspace:offer")]
    Offer,
    #[serde(rename = "dspace:agreement")]
    Agreement,
}

/// Offer is PolicyClass + MessageOffer
#[derive(Serialize, Deserialize, Debug)]
pub struct OdrlOffer {
    // PolicyClass
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "odrl:profile")]
    pub profile: Option<OdrlProfile>,
    #[serde(rename = "odrl:permission")]
    pub permission: Option<Vec<OdrlPermission>>, // anyof
    #[serde(rename = "odrl:obligation")]
    pub obligation: Option<Vec<OdrlObligation>>,
    // MessageOffer
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "odrl:prohibition")]
    pub prohibition: Option<Vec<OdrlObligation>>, // anyof
}

/// Offer is PolicyClass + Agreement
#[derive(Serialize, Deserialize, Debug)]
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
    pub _type: String,
    #[serde(rename = "odrl:target")]
    pub target: Option<String>,
    #[serde(rename = "odrl:assigner")]
    pub assigner: String,
    #[serde(rename = "odrl:assignee")]
    pub assignee: String,
    #[serde(rename = "odrl:timestamp")]
    pub timestamp: Option<String>,
    #[serde(rename = "odrl:prohibition")]
    pub prohibition: Option<Vec<OdrlObligation>>, // anyof
}

// ODRL Profile type
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OdrlProfile {
    Single(String),
    Multiple(Vec<String>),
}

/// OdrlPermission
#[derive(Serialize, Deserialize, Debug)]
pub struct OdrlPermission {
    pub action: OdrlAction,
    pub constraint: Option<Vec<OdrlConstraint>>,
    pub duty: Option<OdrlDuty>,
}

/// OdrlDuty
#[derive(Serialize, Deserialize, Debug)]
pub struct OdrlDuty {
    pub action: OdrlAction,
    pub constraint: Option<Vec<OdrlConstraint>>,
}

/// OdrlObligation
pub type OdrlObligation = OdrlDuty;

/// OdrlAction
pub type OdrlAction = String;


/// OdrlConstraint
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OdrlConstraint {
    Logical(OdrlLogicalConstraint),
    Atomic(OdrlAtomicConstraint),
}

/// LogicalConstraint permite una de las siguientes propiedades: "and", "andSequence", "or" o "xone".
/// Se usan Option para cada una;
#[derive(Serialize, Deserialize, Debug)]
pub struct OdrlLogicalConstraint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<OdrlConstraint>>,
    #[serde(rename = "andSequence", skip_serializing_if = "Option::is_none")]
    pub and_sequence: Option<Vec<OdrlConstraint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<OdrlConstraint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xone: Option<Vec<OdrlConstraint>>,
}

/// la regla de que exactamente una debe estar presente se valida externamente.
/// let constraint: LogicalConstraint = serde_json::from_str(json_data)?;
/// constraint.validate()?; // si falla, se retorna un error
impl OdrlLogicalConstraint {
    pub fn validate(&self) -> Result<(), String> {
        let count = self.and.is_some() as usize +
            self.and_sequence.is_some() as usize +
            self.or.is_some() as usize +
            self.xone.is_some() as usize;
        if count != 1 {
            Err(format!("Exactamente uno de 'and', 'andSequence', 'or' o 'xone' debe estar presente, encontrado {}", count))
        } else {
            Ok(())
        }
    }
}

// AtomicConstraint define los tres campos requeridos: rightOperand, leftOperand y operator.
#[derive(Serialize, Deserialize, Debug)]
pub struct OdrlAtomicConstraint {
    #[serde(rename = "odrl:rightOperand")]
    pub right_operand: OdrlRightOperand,
    #[serde(rename = "odrl:leftOperand")]
    pub left_operand: OdrlLeftOperand,
    #[serde(rename = "odrl:operator")]
    pub operator: Operator,
}

// Operator se define como un enum con los valores permitidos.
#[derive(Serialize, Deserialize, Debug)]
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

// RightOperand se define para aceptar string, objeto o arreglo.
// Se utiliza serde_json::Value para permitir esta variabilidad.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OdrlRightOperand {
    Str(String),
    Object(serde_json::Map<String, Value>),
    Array(Vec<Value>),
}

// LeftOperand es un string.
pub type OdrlLeftOperand = String;
