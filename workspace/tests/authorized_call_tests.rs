mod common;

#[tokio::test]
async fn test_pause_precompiles() {
    let contract = common::init_and_deploy_contract().await.unwrap();
    contract
        .as_account()
        .pause_precompiles(0)
        .transact()
        .await
        .unwrap();
}
