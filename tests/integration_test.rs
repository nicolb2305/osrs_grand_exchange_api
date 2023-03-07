extern crate ge_api;

#[cfg(test)]
mod tests {
    use ge_api::{client::{Client, Endpoint, Timestep}, data_types::ItemId};

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
    async fn test_mappings() {
        let client = create_client();
        let mappings = client.mappings().await.unwrap();
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
    async fn test_average_5m_timestamp() {
        test_average(Timestep::FiveMinutes, Some(1678190401)).await;
    }

    #[tokio::test]
    async fn test_average_10m() {
        test_average(Timestep::TenMinutes, None).await;
    }

    #[tokio::test]
    async fn test_average_10m_timestamp() {
        test_average(Timestep::TenMinutes, Some(1678190401)).await;
    }

    #[tokio::test]
    async fn test_average_30m() {
        test_average(Timestep::ThirtyMinutes, None).await;
    }

    #[tokio::test]
    async fn test_average_30m_timestamp() {
        test_average(Timestep::ThirtyMinutes, Some(1678190401)).await;
    }

    #[tokio::test]
    async fn test_average_1h() {
        test_average(Timestep::OneHour, None).await;
    }

    #[tokio::test]
    async fn test_average_1h_timestamp() {
        test_average(Timestep::OneHour, Some(1678190401)).await;
    }

    #[tokio::test]
    async fn test_average_3h() {
        test_average(Timestep::ThreeHours, None).await;
    }

    #[tokio::test]
    async fn test_average_3h_timestamp() {
        test_average(Timestep::ThreeHours, Some(1678190401)).await;
    }

    #[tokio::test]
    async fn test_average_6h() {
        test_average(Timestep::SixHours, None).await;
    }

    #[tokio::test]
    async fn test_average_6h_timestamp() {
        test_average(Timestep::SixHours, Some(1678190401)).await;
    }

    #[tokio::test]
    async fn test_timeseries() {
        let cannonball = ItemId(2);
        let client = create_client();
        let timeseries = client.timeseries(cannonball, Timestep::FiveMinutes).await.unwrap();
        assert_ne!(timeseries.data.len(), 0);
    }
}
