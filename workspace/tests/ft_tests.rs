mod common;

#[tokio::test]
async fn test_ft_transfer() {
    let contract = common::init_and_deploy_contract().await.unwrap();

    contract
        .as_account()
        .ft_transfer("some_account.test", 10, Some("some message".to_string()))
        .transact()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_ft_on_transfer() {
    let contract = common::init_and_deploy_contract().await.unwrap();

    let res = contract
        .as_account()
        .ft_on_transfer("some_account.test", 100, String::new())
        .transact()
        .await
        .unwrap()
        .into_value();

    assert_eq!(0u8.to_string(), res);
}

#[tokio::test]
async fn test_ft_transfer_call() {
    let contract = common::init_and_deploy_contract().await.unwrap();

    let res = contract
        .as_account()
        .ft_transfer_call(
            "receiver.test",
            10_000_000,
            Some("some memo".to_string()),
            "0x047e3eE8Da241acfF5d04fc77e138b50BAFf02f0".to_string(),
        )
        .transact()
        .await
        .unwrap()
        .into_value();
    assert_eq!(res, aurora_engine_sdk::promise::PromiseId::new(10000000));
}
