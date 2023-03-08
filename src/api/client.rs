use super::data_types::{
    APIError, GrandExchangeAverage, GrandExchangeLatest, GrandExchangeTimeseries, ItemId,
    MappingItem,
};
use serde::Deserialize;

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

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    ResponseError(APIError),
}

type APIResult<T> = Result<T, Error>;

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
    ) -> APIResult<T> {
        let resp = self
            .client
            .get(format!(
                "https://prices.runescape.wiki/api/v1/{}/{}",
                self.endpoint, route
            ))
            .query(&query)
            .header(reqwest::header::USER_AGENT, &self.user_agent)
            .send()
            .await?;
        match resp.status() {
            reqwest::StatusCode::OK => Ok(resp.json::<T>().await?),
            _ => Err(resp.json::<APIError>().await?.into()),
        }
    }

    pub async fn grand_exchange_latest(
        &self,
        id: Option<ItemId>,
    ) -> APIResult<GrandExchangeLatest> {
        let params = id.map(|x| vec![("id", x.to_string())]);
        self.get("latest", params).await
    }

    pub async fn mappings(&self) -> APIResult<Vec<MappingItem>> {
        self.get("mapping", None).await
    }

    pub async fn average(
        &self,
        timestep: Timestep,
        timestamp: Option<i64>,
    ) -> APIResult<GrandExchangeAverage> {
        let route = timestep.to_string();
        let params = timestamp.map(|x| vec![("timestamp", x.to_string())]);
        self.get(&route, params).await
    }

    pub async fn timeseries(
        &self,
        id: ItemId,
        timestep: Timestep,
    ) -> APIResult<GrandExchangeTimeseries> {
        let params = Some(vec![
            ("id", id.to_string()),
            ("timestep", timestep.to_string()),
        ]);
        self.get("timeseries", params).await
    }
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Endpoint::OldSchoolRuneScape => write!(f, "osrs"),
            Endpoint::DeadManReborn => write!(f, "dmm"),
            Endpoint::FreshStartWorlds => write!(f, "fsw"),
        }
    }
}

impl std::fmt::Display for Timestep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

impl From<Timestep> for i64 {
    fn from(timestep: Timestep) -> Self {
        match timestep {
            Timestep::FiveMinutes => 300,
            Timestep::TenMinutes => 600,
            Timestep::ThirtyMinutes => 1800,
            Timestep::OneHour => 3600,
            Timestep::ThreeHours => 10800,
            Timestep::SixHours => 21600,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReqwestError(err) => write!(f, "{}", err),
            Self::ResponseError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err)
    }
}

impl From<APIError> for Error {
    fn from(err: APIError) -> Self {
        Error::ResponseError(err)
    }
}
