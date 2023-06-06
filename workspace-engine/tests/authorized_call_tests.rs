mod utils;

#[tokio::test]
async fn test_pause_precompiles() {
    let contract = utils::deploy_and_init_contract().await.unwrap();
    contract
        .pause_precompiles(0)
        .max_gas()
        .transact()
        .await
        .unwrap();
}
