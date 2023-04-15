use super::data_types::{
    APIError, GrandExchangeAverage, GrandExchangeLatest, GrandExchangeTimeseries, ItemId,
    MappingItem,
};
use serde::Deserialize;

/// The client used for interacting with the
/// [OSRS Grand Exchange real-time prices API](https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices).
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
    OneDay,
}

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    ResponseError(APIError),
}

type APIResult<T> = Result<T, Error>;

impl Client {
    /// Creates a client used for interacting with the api.
    ///
    /// Please use a descriptive User-Agent such that the wiki admins can reach you in
    /// case of excessive numbers of calls, or changes to the api, see
    /// [api documentation](https://oldschool.runescape.wiki/w/RuneScape:Real-time_Prices#Please_set_a_descriptive_User-Agent!).
    /// ```
    /// # use ge_api::client::{Client, Endpoint};
    /// let client = Client::new(Endpoint::OldSchoolRuneScape, "nicolb2305");
    /// ```
    #[must_use]
    pub fn new(endpoint: Endpoint, user_agent: &str) -> Self {
        Client {
            client: reqwest::Client::new(),
            endpoint,
            user_agent: String::from(user_agent),
        }
    }

    async fn get_endpoint<T>(&self, route: &str, query: Option<Vec<(&str, String)>>) -> APIResult<T>
    where
        T: for<'a> Deserialize<'a>,
    {
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
            reqwest::StatusCode::OK => Ok(resp.json().await?),
            _ => Err(resp.json::<APIError>().await?.into()),
        }
    }

    /// Queries the [/latest](https://prices.runescape.wiki/api/v1/osrs/latest)
    /// route of the api, either returns item with specified `item_id`, or all items on
    /// the Grand Exchange, if specified.
    /// ```
    /// # use ge_api::{client::{Client, Endpoint}, data_types::ItemId};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new(Endpoint::OldSchoolRuneScape, "nicolb2305");
    /// let all_items = client.get_latest(None).await?;
    /// let cannonball = client.get_latest(Some(ItemId(2))).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// # Errors
    /// An error is returned if the api request cannot be completed, or if the response
    /// could not be parsed succesfully.
    pub async fn get_latest(&self, item_id: Option<ItemId>) -> APIResult<GrandExchangeLatest> {
        let params = item_id.map(|x| vec![("id", x.to_string())]);
        self.get_endpoint("latest", params).await
    }

    /// Queries the [/mapping](https://prices.runescape.wiki/api/v1/osrs/mapping)
    /// route of the api, returns a list of all items tradeable on the Grand Exchange.
    /// ```
    /// # use ge_api::client::{Client, Endpoint};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new(Endpoint::OldSchoolRuneScape, "nicolb2305");
    /// let mapping = client.get_mapping().await?;
    /// # Ok(())
    /// # }
    /// ```
    /// # Errors
    /// An error is returned if the api request cannot be completed, or if the response
    /// could not be parsed succesfully.
    pub async fn get_mapping(&self) -> APIResult<Vec<MappingItem>> {
        self.get_endpoint("mapping", None).await
    }

    /// Queries any of the time-duration average endpoints
    /// (e.g., [/5m](https://prices.runescape.wiki/api/v1/osrs/5m)).
    ///
    /// `timestep` determines which endpoint is called, while `timestamp` can optionally
    /// be used to specify the starting point time window to average over.
    ///
    /// __NOTE__: `timestamp` must be divisible by the number of seconds in `timestep`,
    /// [`round_to_previous_timestamp()`](super::utils::round_to_previous_timestamp) can
    /// be used to round down to nearest valid `timestamp`.
    /// ```
    /// # use ge_api::client::{Client, Endpoint, Timestep};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new(Endpoint::OldSchoolRuneScape, "nicolb2305");
    /// let average_5m = client.get_average(Timestep::FiveMinutes, None).await?;
    /// let average_5m_past = client.get_average(Timestep::FiveMinutes, Some(1678190400)).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// # Errors
    /// An error is returned if the api request cannot be completed, or if the response
    /// could not be parsed succesfully.
    pub async fn get_average(
        &self,
        timestep: Timestep,
        timestamp: Option<i64>,
    ) -> APIResult<GrandExchangeAverage> {
        let route = timestep.to_string();
        let params = timestamp.map(|x| vec![("timestamp", x.to_string())]);
        self.get_endpoint(&route, params).await
    }

    /// Queries the [/timeseries](https://prices.runescape.wiki/api/v1/osrs/timeseries?timestep=5m&id=4151)
    /// endpoint of the api, returns the price of an item with given `item_id` over
    /// time, with averaged over a duration of `timestep`.
    ///
    /// ```
    /// # use ge_api::{client::{Client, Endpoint, Timestep}, data_types::ItemId};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new(Endpoint::OldSchoolRuneScape, "nicolb2305");
    /// let timeseries_cannonball = client.get_timeseries(ItemId(2), Timestep::FiveMinutes).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// # Errors
    /// An error is returned if the api request cannot be completed, or if the response
    /// could not be parsed succesfully.
    pub async fn get_timeseries(
        &self,
        item_id: ItemId,
        timestep: Timestep,
    ) -> APIResult<GrandExchangeTimeseries> {
        let params = Some(vec![
            ("id", item_id.to_string()),
            ("timestep", timestep.to_string()),
        ]);
        self.get_endpoint("timeseries", params).await
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
            Timestep::OneDay => write!(f, "24h"),
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
            Timestep::OneDay => 86400,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReqwestError(err) => write!(f, "{err}"),
            Self::ResponseError(err) => write!(f, "{err}"),
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
