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

use crate::protocol::contract::contract_odrl::OdrlOffer;
use sea_orm::sea_query::ValueType;
use sea_orm::sea_query::ValueTypeErr;
use sea_orm::TryGetable;
use sea_orm::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OdrlOfferWrapper(pub OdrlOffer);

impl Into<Value> for OdrlOfferWrapper {
    fn into(self) -> Value {
        let json = serde_json::to_value(self.0).unwrap_or_default();
        Value::Json(Some(Box::new(json)))
    }
}

impl ValueType for OdrlOfferWrapper {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Json(Some(json)) => {
                serde_json::from_value::<OdrlOffer>(*json)
                    .map(OdrlOfferWrapper)
                    .map_err(|_| ValueTypeErr)
            }
            Value::Json(None) => Ok(OdrlOfferWrapper(OdrlOffer::default())),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "json".to_string()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::Json
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::Json
    }
}

impl TryGetable for OdrlOfferWrapper {
    fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, idx: I) -> Result<Self, sea_orm::TryGetError> {
        let json: serde_json::Value = res.try_get_by(idx)?;
        serde_json::from_value(json)
            .map(OdrlOfferWrapper)
            .map_err(|e| sea_orm::TryGetError::DbErr(sea_orm::DbErr::Custom(e.to_string())))
    }
}


impl From<OdrlOffer> for OdrlOfferWrapper {
    fn from(offer: OdrlOffer) -> Self {
        OdrlOfferWrapper(offer)
    }
}

impl From<OdrlOfferWrapper> for OdrlOffer {
    fn from(wrapper: OdrlOfferWrapper) -> Self {
        wrapper.0
    }
}