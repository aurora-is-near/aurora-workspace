use aurora_workspace::operation::ViewResultDetails;
use aurora_workspace::types::AccountId;
use std::str::FromStr;

mod common;


#[tokio::test]
async fn test_version() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract
        .as_account()
        .version()
        .await?;
    let expected = ViewResultDetails{
        result: "\"v1\"".to_string().into(),
        logs: vec![]
    };
    assert_eq!(expected, res);
    Ok(())
}

#[tokio::test]
async fn test_owner() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract
        .as_account()
        .owner()
        .await?;
    let expected = ViewResultDetails{
        result: AccountId::from_str("owner.test.near").expect("Invalid account"),
        logs: vec![]
    };
    assert_eq!(expected, res);
    Ok(())
}

#[tokio::test]
async fn test_chain_id() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract
        .as_account()
        .chain_id()
        .await?;
    let expected = ViewResultDetails{
        result: 1313161556u128,
        logs: vec![]
    };
    assert_eq!(expected, res);
    Ok(())
}