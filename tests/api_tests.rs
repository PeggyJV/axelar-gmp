use axelar_gmp::builder::SearchGMPRequestBuilder;

fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[tokio::test]
async fn test_search_gmp() {
    init_logger();

    let result = SearchGMPRequestBuilder::default()
        .tx_hash("A4EA656045A75F13B113165D14E44253A21B9EE25868A9793187C5CC7B415F71".to_string())
        .source_chain("sommelier".to_string())
        .build()
        .send()
        .await;

    println!("result: {:?}", result);
    assert!(result.is_ok());
}
