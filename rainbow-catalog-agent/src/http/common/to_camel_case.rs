use convert_case::{Case, Casing};
use serde::{Serialize, Serializer};
use serde_json::Value;

pub struct ToCamelCase<T>(pub T);

impl<T: Serialize> Serialize for ToCamelCase<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = serde_json::to_value(&self.0).map_err(serde::ser::Error::custom)?;
        let converted_value = keys_to_camel(value);
        converted_value.serialize(serializer)
    }
}

fn keys_to_camel(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (k, v) in map {
                let new_key = k.to_case(Case::Camel);
                new_map.insert(new_key, keys_to_camel(v));
            }
            Value::Object(new_map)
        }
        Value::Array(vec) => {
            let new_vec = vec.into_iter().map(keys_to_camel).collect();
            Value::Array(new_vec)
        }
        v => v,
    }
}
