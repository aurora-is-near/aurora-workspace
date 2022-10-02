use aurora_workspace_types::{AccountId};
use std::str::FromStr;

mod common;

#[tokio::test]
async fn test_ft_on_transfer() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;

    let res = contract
        .as_account()
        .ft_on_transfer(
            AccountId::from_str("some_account.test").expect("Failed to make Account from str"),
            100u8,
            String::new(),
        )?
        .transact()
        .await?
        .into_value();

    assert_eq!(0u8.to_string(), res);

    Ok(())
}
