mod common;

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
