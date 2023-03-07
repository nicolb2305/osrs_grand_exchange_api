use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct GrandExchangeLatest {
    data: HashMap<ItemId, GrandExchangeItem>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ItemId(pub i64);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GrandExchangeItem {
    high: Option<i32>,
    high_time: Option<i64>,
    low: Option<i32>,
    low_time: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct MappingItem {
    examine: String,
    id: ItemId,
    members: bool,
    lowalch: Option<i32>,
    highalch: Option<i32>,
    limit: Option<i64>,
    value: i32,
    icon: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct GrandExchangeAverage {
    data: HashMap<ItemId, GrandExchangeAverageItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GrandExchangeAverageItem {
    avg_high_price: Option<i32>,
    avg_high_price_volume: Option<u64>,
    avg_low_price: Option<i32>,
    avg_low_price_volume: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct GrandExchangeTimeseries {
    data: Vec<GrandExchangeTimeseriesItem>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GrandExchangeTimeseriesItem {
    timestamp: i64,
    avg_high_price: Option<i32>,
    avg_low_price: Option<i32>,
    high_price_volume: Option<u64>,
    low_price_volume: Option<u64>

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
        write!(f, "{}", self.0.to_string())
    }
}
