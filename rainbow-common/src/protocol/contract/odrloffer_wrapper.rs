use sea_orm::sea_query::ValueType;
use sea_orm::Value;
use sea_orm::sea_query::ValueTypeErr;
use crate::protocol::contract::contract_odrl::OdrlOffer;
use serde::{Serialize, Deserialize};
use sea_orm::TryGetable; 

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