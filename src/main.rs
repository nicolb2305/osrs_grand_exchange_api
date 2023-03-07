#![allow(dead_code, unused_variables)]
mod api;
use crate::api::{client::*, data_types::ItemId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(Endpoint::OldSchoolRuneScape, "nicolb2305#3850");

    let grand_exchange_latest = client.grand_exchange_latest(None).await?;
    let mappings = client.mappings().await?;
    let average = client.average(Timestep::FiveMinutes, None).await?;
    let timeseries = client.timeseries(ItemId(2), Timestep::FiveMinutes).await?;

    // dbg!(grand_exchange_latest);
    // dbg!(mappings);
    // dbg!(average);
    dbg!(timeseries);

    Ok(())
}
