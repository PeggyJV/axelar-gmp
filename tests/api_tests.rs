use axelar_gmp::builder::SearchGMPRequestBuilder;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[tokio::test]
async fn test_search_gmp() {
    init_logger();

    let result = SearchGMPRequestBuilder::default()
        .from_time(1715400000)
        .to_time(1715462400)
        .size(100)
        .build()
        .send()
        .await;

    assert!(result.is_ok());
}
