//! Decode raw members before typed conversion: Serde's flatten buffer would lose
//! lexical JSON bytes. Only value bytes are retained, not outer document layout.

use std::{collections::BTreeMap, fmt};

use serde::{
    Deserialize, Deserializer,
    de::{self, DeserializeOwned, MapAccess, Visitor},
};
use serde_json::value::RawValue;

pub type RawObject = BTreeMap<String, Box<RawValue>>;

pub(crate) struct RawFields(pub(crate) RawObject);

impl<'de> Deserialize<'de> for RawFields {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct FieldsVisitor;
        impl<'de> Visitor<'de> for FieldsVisitor {
            type Value = RawFields;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a JSON object with unique member names")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut fields = BTreeMap::new();
                while let Some(name) = map.next_key::<String>()? {
                    if fields.contains_key(&name) {
                        return Err(de::Error::custom(format!("duplicate field {name}")));
                    }
                    fields.insert(name, map.next_value::<Box<RawValue>>()?);
                }
                Ok(RawFields(fields))
            }
        }
        deserializer.deserialize_map(FieldsVisitor)
    }
}

pub(crate) fn required<T: DeserializeOwned>(
    fields: &mut RawObject,
    key: &'static str,
) -> serde_json::Result<T> {
    let raw = fields
        .remove(key)
        .ok_or_else(|| <serde_json::Error as de::Error>::missing_field(key))?;
    serde_json::from_str(raw.get())
}

pub(crate) fn optional<T: DeserializeOwned>(
    fields: &mut RawObject,
    key: &str,
) -> serde_json::Result<Option<T>> {
    match fields.remove(key) {
        Some(raw) => serde_json::from_str(raw.get()),
        None => Ok(None),
    }
}

// Shared only by DTOs that retain unknown raw members.
macro_rules! raw_object_model {
    ($model:ty, [$($required:ident),*], [$($optional:ident => $key:literal),*]) => {
        impl TryFrom<$crate::documents::raw_objects::RawFields> for $model {
            type Error = serde_json::Error;
            fn try_from(raw: $crate::documents::raw_objects::RawFields) -> Result<Self, Self::Error> {
                let mut fields = raw.0;
                Ok(Self {
                    $($required: $crate::documents::raw_objects::required(&mut fields, stringify!($required))?,)*
                    $($optional: $crate::documents::raw_objects::optional(&mut fields, $key)?,)*
                    extensions: fields,
                })
            }
        }
    };
}
pub(crate) use raw_object_model;
