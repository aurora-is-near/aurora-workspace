use aurora_workspace::EvmContract;
use aurora_workspace_types::output::{Log, SubmitResult, TransactionStatus};
use aurora_workspace_types::{AccountId, Address, H256};
use std::str::FromStr;
use workspaces::types::{KeyType, SecretKey};

mod common;

#[tokio::test]
async fn test_deploy_code() {
    let contract = common::init_and_deploy_contract().await.unwrap();

    let res = contract
        .as_account()
        .deploy_code(vec![1u8; 32])
        .transact()
        .await
        .unwrap()
        .into_value();

    let log = Log::new(
        Address::from([1u8; 20]),
        vec![H256::from([2u8; 32])],
        vec![3u8; 10],
    );
    let expected = SubmitResult::new(TransactionStatus::Succeed(vec![0]), 100_000, vec![log]);

    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_deploy_erc20() {
    let contract = common::init_and_deploy_contract().await.unwrap();

    let res = contract
        .as_account()
        .deploy_erc20(AccountId::from_str("some_account.test").unwrap())
        .transact()
        .await
        .unwrap()
        .into_value();

    let expected = Address::from([1u8; 20]);

    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_call() {
    let contract = common::init_and_deploy_contract().await.unwrap();
    let res = contract
        .as_account()
        .call(Address::from([1u8; 20]), 10, vec![])
        .transact()
        .await
        .unwrap()
        .into_value();
    let log = Log::new(
        Address::from([1u8; 20]),
        vec![H256::from([2u8; 32])],
        vec![3u8; 10],
    );
    let expected = SubmitResult::new(TransactionStatus::Succeed(vec![0]), 100_000, vec![log]);

    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_submit() {
    let contract = common::init_and_deploy_contract().await.unwrap();

    let res = contract
        .as_account()
        .submit(vec![1u8; 32])
        .transact()
        .await
        .unwrap()
        .into_value();

    let log = Log::new(
        Address::from([1u8; 20]),
        vec![H256::from([2u8; 32])],
        vec![3u8; 10],
    );
    let expected = SubmitResult::new(TransactionStatus::Succeed(vec![0]), 100_000, vec![log]);

    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_from_secret_key() {
    let testnet_worker = workspaces::testnet().await.unwrap();
    let sk = SecretKey::from_random(KeyType::ED25519);
    let _contract = EvmContract::from_secret_key("aurora.test.near", sk, &testnet_worker).unwrap();
}

#[tokio::test]
async fn test_register_relayer() {
    let contract = common::init_and_deploy_contract().await.unwrap();

    contract
        .as_account()
        .register_relayer(Address::from([1u8; 20]))
        .transact()
        .await
        .unwrap()
        .into_value();

    // Nothing to expect here...
}
