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

use crate::protocol::contract::contract_odrl::OdrlPolicyInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub enum TemplateFormTypes {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "radio")]
    Radio,
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "select")]
    Select,
    #[serde(rename = "multiselect")]
    MultiSelect,
    #[serde(rename = "datetime")]
    DateTime,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TemplateOperandDefaultValues {
    Multiple(Vec<String>),
    I32(i32),
    String(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalizedString {
    #[serde(rename = "@language")]
    language: String,
    #[serde(rename = "@value")]
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateOperandOption {
    value: String,
    label: Vec<LocalizedString>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateOperand {
    #[serde(rename = "formType")]
    form_type: TemplateFormTypes,
    label: Vec<LocalizedString>,
    options: Option<Vec<TemplateOperandOption>>,
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<TemplateOperandDefaultValues>,
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePolicyTemplateRequest {
    pub title: String, // TODO localized string
    pub description: String, // TODO localized string
    pub content: OdrlPolicyInfo,
    pub template_operands: HashMap<String, TemplateOperand>,
}
