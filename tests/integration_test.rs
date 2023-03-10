extern crate ge_api;

#[cfg(test)]
mod tests {
    use ge_api::{
        client::{Client, Endpoint, Timestep},
        data_types::{GrandExchangeAverage, ItemId},
        utils::round_to_previous_timestamp,
    };

    fn create_client() -> Client {
        return Client::new(Endpoint::OldSchoolRuneScape, "nicolb2305");
    }

    #[tokio::test]
    async fn test_grand_exchange_latest_no_id() {
        let cannonball = ItemId(2);
        let client = create_client();
        let latest = client.get_latest(None).await.unwrap();
        latest.data.get(&cannonball);
    }

    #[tokio::test]
    async fn test_grand_exchange_latest_with_id() {
        let cannonball = ItemId(2);
        let client = create_client();
        let latest = client.get_latest(Some(ItemId(2))).await.unwrap();
        latest.data.get(&cannonball);
    }

    #[tokio::test]
    async fn test_mapping() {
        let client = create_client();
        let mappings = client.get_mapping().await.unwrap();
        assert_ne!(mappings.len(), 0);
    }

    async fn call_average(
        timestep: Timestep,
        timestamp: Option<i64>,
    ) -> Result<GrandExchangeAverage, ge_api::client::Error> {
        let client = create_client();
        client.get_average(timestep, timestamp).await
    }

    #[tokio::test]
    async fn test_average_6h_no_timestamp() {
        let cannonball = ItemId(2);
        let average = call_average(Timestep::SixHours, None).await.unwrap();
        average.data.get(&cannonball);
    }

    #[tokio::test]
    async fn test_average_6h_correct_timestamp() {
        let cannonball = ItemId(2);
        let average = call_average(Timestep::SixHours, Some(1678190400))
            .await
            .unwrap();
        average.data.get(&cannonball);
    }

    #[tokio::test]
    async fn test_average_6h_wrong_timestamp() {
        let average = call_average(Timestep::SixHours, Some(1678190401)).await;
        assert!(average.is_err());
    }

    #[tokio::test]
    async fn test_timeseries() {
        let cannonball = ItemId(2);
        let client = create_client();
        let timeseries = client
            .get_timeseries(cannonball, Timestep::FiveMinutes)
            .await
            .unwrap();
        assert_ne!(timeseries.data.len(), 0);
    }

    #[tokio::test]
    async fn test_round_to_previous_timestamp() {
        let rounded = round_to_previous_timestamp(Timestep::SixHours, 1678190401);
        assert_eq!(rounded, 1678190400);
    }

    #[tokio::test]
    async fn test_map_mappings_to_latest() {
        let client = create_client();
        let latest = client.get_latest(None).await.unwrap();
        let mappings = client.get_mapping().await.unwrap();

        let item_names_with_latest = mappings
            .iter()
            .filter_map(|x| match latest.data.get(&x.id) {
                Some(data) => Some((&x.name, data)),
                None => None,
            });

        assert_ne!(item_names_with_latest.count(), 0);
    }
}
