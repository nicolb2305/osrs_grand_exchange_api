use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct GrandExchangeLatest {
    pub data: HashMap<ItemId, GrandExchangeItem>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ItemId(pub i64);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GrandExchangeItem {
    pub high: Option<i32>,
    pub high_time: Option<i64>,
    pub low: Option<i32>,
    pub low_time: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct MappingItem {
    pub examine: String,
    pub id: ItemId,
    pub members: bool,
    pub lowalch: Option<i32>,
    pub highalch: Option<i32>,
    pub limit: Option<i64>,
    pub value: i32,
    pub icon: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct GrandExchangeAverage {
    pub data: HashMap<ItemId, GrandExchangeAverageItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GrandExchangeAverageItem {
    pub avg_high_price: Option<i32>,
    pub avg_high_price_volume: Option<u64>,
    pub avg_low_price: Option<i32>,
    pub avg_low_price_volume: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct GrandExchangeTimeseries {
    pub data: Vec<GrandExchangeTimeseriesItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GrandExchangeTimeseriesItem {
    pub timestamp: i64,
    pub avg_high_price: Option<i32>,
    pub avg_low_price: Option<i32>,
    pub high_price_volume: Option<u64>,
    pub low_price_volume: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct APIError {
    pub error: String,
}

impl<'de> Deserialize<'de> for ItemId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IdVisitor;

        impl<'de> Visitor<'de> for IdVisitor {
            type Value = ItemId;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("Item ID as a number or string")
            }

            fn visit_i64<E>(self, id: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ItemId(id))
            }

            fn visit_i32<E>(self, id: i32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ItemId(id.into()))
            }

            fn visit_u32<E>(self, id: u32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ItemId(id.into()))
            }

            fn visit_u64<E>(self, id: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match id.try_into() {
                    Ok(id) => Ok(ItemId(id)),
                    Err(_) => Err(E::custom("Failed to cast u64 to i64")),
                }
            }

            fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                id.parse().map(ItemId).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(IdVisitor)
    }
}

impl fmt::Display for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for APIError {}

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
