use aurora_engine::metadata::FungibleTokenMetadata;
use aurora_workspace::EvmContract;
use aurora_workspace_types::output::{Log, SubmitResult, TransactionStatus};
use aurora_workspace_types::{AccountId, Address, H256};
use std::str::FromStr;
use workspaces::types::{KeyType, SecretKey};

mod utils;

#[tokio::test]
async fn test_deploy_code() {
    let contract = utils::init_and_deploy_contract().await.unwrap();

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
    let contract = utils::init_and_deploy_contract().await.unwrap();

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
    let contract = utils::init_and_deploy_contract().await.unwrap();
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
    let contract = utils::init_and_deploy_contract().await.unwrap();

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
    let contract = utils::init_and_deploy_contract().await.unwrap();

    contract
        .as_account()
        .register_relayer(Address::from([1u8; 20]))
        .transact()
        .await
        .unwrap()
        .into_value();

    // Nothing to expect here...
}

#[tokio::test]
async fn test_set_eth_connector_contract_data() {
    let contract = utils::init_and_deploy_contract().await.unwrap();

    contract
        .as_account()
        .set_eth_connector_contract_data(
            "prover.test.near",
            "custodian.test.near",
            FungibleTokenMetadata::default(),
        )
        .transact()
        .await
        .unwrap();
}
/*
#[tokio::test]
async fn test_factory_update_address_version() {
    let contract = common::init_and_deploy_contract().await.unwrap();

    let res = contract
        .as_account()
        .factory_update_address_version(Address::default(), 0)
        .transact()
        .await
        .unwrap()
        .into_value();

    let expected = 0;
    assert_eq!(expected, res);
}
*/
#[tokio::test]
async fn test_refund_on_error() {
    let contract = utils::init_and_deploy_contract().await.unwrap();

    let res = contract
        .as_account()
        .refund_on_error(Address::default(), Some(Address::default()), 0.into())
        .transact()
        .await
        .unwrap()
        .into_value();

    let expected = 0;
    assert_eq!(expected, res);
}
