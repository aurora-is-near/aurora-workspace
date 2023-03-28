use aurora_workspace_eth_connector::operation::ViewResultDetails;
use aurora_workspace_types::AccountId;
use near_sdk::json_types::U128;
use near_sdk::PromiseOrValue;
use std::str::FromStr;

mod utils;

#[tokio::test]
async fn test_ft_transfer() {
    let contract = utils::init_and_deploy_contract().await.unwrap();
    let some_acc = AccountId::from_str("some_account.test.near").unwrap();
    let amount: U128 = 10.into();
    let memo = Some(String::from("some message"));

    contract
        .as_account()
        .ft_transfer(some_acc, amount, memo)
        .max_gas()
        .deposit(1)
        .transact()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_ft_transfer_call() {
    let contract = utils::init_and_deploy_contract().await.unwrap();
    let some_acc = AccountId::from_str("some_account.test.near").unwrap();
    let amount: U128 = 10.into();
    let memo = Some(String::from("some message"));
    let msg = String::from("some msg");

    let res: PromiseOrValue<U128> = contract
        .as_account()
        .ft_transfer_call(some_acc, amount, memo, msg)
        .max_gas()
        .deposit(1)
        .transact()
        .await
        .unwrap()
        .into_value();

    let val = match res {
        PromiseOrValue::Value(v) => v,
        _ => panic!("failed parse"),
    };
    assert_eq!(U128::from(10), val);
}

#[tokio::test]
async fn test_ft_total_supply() {
    let contract = utils::init_and_deploy_contract().await.unwrap();
    let res = contract.as_account().ft_total_supply().await.unwrap();
    let expected = ViewResultDetails {
        result: U128::from(100),
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_ft_balance_of() {
    let contract = utils::init_and_deploy_contract().await.unwrap();
    let account = contract.as_account().id().clone();
    let res = contract.as_account().ft_balance_of(account).await.unwrap();
    let expected = ViewResultDetails {
        result: U128::from(200),
        logs: vec![],
    };
    assert_eq!(res, expected);
}
