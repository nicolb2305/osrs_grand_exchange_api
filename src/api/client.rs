use super::data_types::{
    GrandExchangeAverage, GrandExchangeLatest, GrandExchangeTimeseries, ItemId, MappingItem,
};
use serde::Deserialize;
use std::fmt::{Display, Formatter, Result};

pub struct Client {
    client: reqwest::Client,
    endpoint: Endpoint,
    user_agent: String,
}

pub enum Endpoint {
    OldSchoolRuneScape,
    DeadManReborn,
    FreshStartWorlds,
}

pub enum Timestep {
    FiveMinutes,
    TenMinutes,
    ThirtyMinutes,
    OneHour,
    ThreeHours,
    SixHours,
}

impl Client {
    pub fn new(endpoint: Endpoint, user_agent: &str) -> Self {
        Client {
            client: reqwest::Client::new(),
            endpoint,
            user_agent: String::from(user_agent),
        }
    }

    async fn get<T: for<'a> Deserialize<'a>>(
        &self,
        route: &str,
        query: Option<Vec<(&str, String)>>,
    ) -> reqwest::Result<T> {
        self.client
            .get(format!(
                "https://prices.runescape.wiki/api/v1/{}/{}",
                self.endpoint, route
            ))
            .query(&query)
            .header(reqwest::header::USER_AGENT, &self.user_agent)
            .send()
            .await?
            .json::<T>()
            .await
    }

    pub async fn grand_exchange_latest(
        &self,
        id: Option<ItemId>,
    ) -> reqwest::Result<GrandExchangeLatest> {
        let params = id.map(|x| vec![("id", x.to_string())]);
        self.get("latest", params).await
    }

    pub async fn mappings(&self) -> reqwest::Result<Vec<MappingItem>> {
        self.get("mapping", None).await
    }

    pub async fn average(
        &self,
        timestep: Timestep,
        timestamp: Option<i64>,
    ) -> reqwest::Result<GrandExchangeAverage> {
        let route = timestep.to_string();
        let params = timestamp.map(|x| vec![("timestamp", x.to_string())]);
        self.get(&route, params).await
    }

    pub async fn timeseries(
        &self,
        id: ItemId,
        timestep: Timestep,
    ) -> reqwest::Result<GrandExchangeTimeseries> {
        let params = Some(vec![
            ("id", id.to_string()),
            ("timestep", timestep.to_string()),
        ]);
        self.get("timeseries", params).await
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Endpoint::OldSchoolRuneScape => write!(f, "osrs"),
            Endpoint::DeadManReborn => write!(f, "dmm"),
            Endpoint::FreshStartWorlds => write!(f, "fsw"),
        }
    }
}

impl Display for Timestep {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Timestep::FiveMinutes => write!(f, "5m"),
            Timestep::TenMinutes => write!(f, "10m"),
            Timestep::ThirtyMinutes => write!(f, "30m"),
            Timestep::OneHour => write!(f, "1h"),
            Timestep::ThreeHours => write!(f, "3h"),
            Timestep::SixHours => write!(f, "6h"),
        }
    }
}
