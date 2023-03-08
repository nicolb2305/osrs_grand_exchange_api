extern crate ge_api;

#[cfg(test)]
mod tests {
    use ge_api::{
        client::{Client, Endpoint, Timestep},
        data_types::ItemId,
        utils::round_to_previous_timestamp,
    };

    fn create_client() -> Client {
        return Client::new(Endpoint::OldSchoolRuneScape, "nicolb2305");
    }

    #[tokio::test]
    async fn test_grand_exchange_latest_no_id() {
        let cannonball = ItemId(2);
        let client = create_client();
        let latest = client.grand_exchange_latest(None).await.unwrap();
        latest.data.get(&cannonball);
    }

    #[tokio::test]
    async fn test_grand_exchange_latest_with_id() {
        let cannonball = ItemId(2);
        let client = create_client();
        let latest = client.grand_exchange_latest(Some(ItemId(2))).await.unwrap();
        latest.data.get(&cannonball);
    }

    #[tokio::test]
    async fn test_mapping() {
        let client = create_client();
        let mappings = client.mapping().await.unwrap();
        assert_ne!(mappings.len(), 0);
    }

    async fn test_average(timestep: Timestep, timestamp: Option<i64>) {
        let cannonball = ItemId(2);
        let client = create_client();
        let average = client.average(timestep, timestamp).await.unwrap();
        average.data.get(&cannonball);
    }

    #[tokio::test]
    async fn test_average_5m() {
        test_average(Timestep::FiveMinutes, None).await;
    }

    #[tokio::test]
    async fn test_average_10m() {
        test_average(Timestep::TenMinutes, None).await;
    }

    #[tokio::test]
    async fn test_average_1h() {
        test_average(Timestep::OneHour, None).await;
    }

    #[tokio::test]
    async fn test_average_3h() {
        test_average(Timestep::ThreeHours, None).await;
    }

    #[tokio::test]
    async fn test_average_6h() {
        test_average(Timestep::SixHours, None).await;
    }

    #[tokio::test]
    async fn test_average_6h_correct_timestamp() {
        test_average(Timestep::SixHours, Some(1678190400)).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn test_average_6h_wrong_timestamp() {
        test_average(Timestep::SixHours, Some(1678190401)).await;
    }

    #[tokio::test]
    async fn test_timeseries() {
        let cannonball = ItemId(2);
        let client = create_client();
        let timeseries = client
            .timeseries(cannonball, Timestep::FiveMinutes)
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
        let latest = client.grand_exchange_latest(None).await.unwrap();
        let mappings = client.mapping().await.unwrap();

        let item_names_with_latest = mappings
            .iter()
            .filter_map(|x| match latest.data.get(&x.id) {
                Some(data) => Some((&x.name, data)),
                None => None,
            });

        assert_ne!(item_names_with_latest.count(), 0);
    }
}
