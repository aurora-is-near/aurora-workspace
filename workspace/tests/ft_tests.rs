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

// #[tokio::test]
// async fn test_ft_transfer_call() -> anyhow::Result<()> {
//     let contract = common::init_and_deploy_contract().await?;
//
//     let res = contract
//         .as_account()
//         .ft_transfer_call(
//             "receiver.test",
//             10,
//             Some("some memo".to_string()),
//             "some message".to_string(),
//         )
//         .transact()
//         .await?
//         .into_value();
//
//     Ok(())
// }
