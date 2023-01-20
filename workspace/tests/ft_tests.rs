use aurora_workspace_types::AccountId;
use std::str::FromStr;

mod common;

#[tokio::test]
async fn test_ft_transfer() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;

    contract
        .as_account()
        .ft_transfer("some_account.test", 10, Some("some message".to_string()))
        .transact()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_ft_on_transfer() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;

    let res = contract
        .as_account()
        .ft_on_transfer(
            AccountId::from_str("some_account.test").expect("Failed to make Account from str"),
            100u8,
            String::new(),
        )
        .transact()
        .await?
        .into_value();

    assert_eq!(0u8.to_string(), res);

    Ok(())
}

#[tokio::test]
async fn test_ft_transfer_call() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;

    let res = contract
        .as_account()
        .ft_transfer_call(
            AccountId::from_str("receiver.test").expect("Failed to make Account from str"),
            10000000u128,
            Some("some memo".to_string()),
            "0x047e3eE8Da241acfF5d04fc77e138b50BAFf02f0".to_string(),
        )
        .transact()
        .await?
        .into_value();
    // why the amount is 10000000u128 lead to this value 3472328296227680305 ?
    assert_eq!(res, aurora_engine_sdk::promise::PromiseId::new(3472328296227680305));
    Ok(())
}
