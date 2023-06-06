use aurora_engine::fungible_token::FungibleTokenMetadata;
use aurora_workspace_types::{AccountId, Address};
use std::str::FromStr;

mod utils;

// #[tokio::test]
// async fn test_deploy_code() {
//     let contract = utils::deploy_and_init_contract().await.unwrap();
//
//     let res = contract
//         .deploy_code(vec![1u8; 32])
//         .transact()
//         .await
//         .unwrap()
//         .into_value();
//
//     let log = Log::new(
//         Address::from_array([1u8; 20]),
//         vec![H256::from([2u8; 32])],
//         vec![3u8; 10],
//     );
//     let expected = SubmitResult::new(TransactionStatus::Succeed(vec![0]), 100_000, vec![log]);
//
//     assert_eq!(res, expected);
// }

#[tokio::test]
async fn test_deploy_erc20() {
    let contract = utils::deploy_and_init_contract().await.unwrap();

    let res = contract
        .deploy_erc20_token(AccountId::from_str("some_account.test").unwrap())
        .transact()
        .await
        .unwrap()
        .into_value();

    let expected = Address::from_array([1u8; 20]);

    assert_eq!(res, expected);
}

// #[tokio::test]
// async fn test_call() {
//     let contract = utils::deploy_and_init_contract().await.unwrap();
//     let res = contract
//         .call(Address::from_array([1u8; 20]), 10.into(), vec![])
//         .transact()
//         .await
//         .unwrap()
//         .into_value();
//     let log = Log::new(
//         Address::from_array([1u8; 20]),
//         vec![H256::from([2u8; 32])],
//         vec![3u8; 10],
//     );
//     let expected = SubmitResult::new(TransactionStatus::Succeed(vec![0]), 100_000, vec![log]);
//
//     assert_eq!(res, expected);
// }

// #[tokio::test]
// async fn test_submit() {
//     let contract = utils::deploy_and_init_contract().await.unwrap();
//
//     let res = contract
//         .submit(vec![1u8; 32])
//         .transact()
//         .await
//         .unwrap()
//         .into_value();
//
//     let log = Log::new(
//         Address::from_array([1u8; 20]),
//         vec![H256::from([2u8; 32])],
//         vec![3u8; 10],
//     );
//     let expected = SubmitResult::new(TransactionStatus::Succeed(vec![0]), 100_000, vec![log]);
//
//     assert_eq!(res, expected);
// }

// #[tokio::test]
// async fn test_from_secret_key() {
//     let testnet_worker = workspaces::testnet().await.unwrap();
//     let sk = SecretKey::from_random(KeyType::ED25519);
//     let _contract =
//         EngineContract::new
//
//         // ::from_secret_key("aurora.test.near", sk, &testnet_worker).unwrap();
// }

#[tokio::test]
async fn test_register_relayer() {
    let contract = utils::deploy_and_init_contract().await.unwrap();

    contract
        .register_relayer(Address::from_array([1u8; 20]))
        .transact()
        .await
        .unwrap()
        .into_value();

    // Nothing to expect here...
}

#[tokio::test]
async fn test_set_eth_connector_contract_data() {
    let contract = utils::deploy_and_init_contract().await.unwrap();

    contract
        .set_eth_connector_contract_data(
            "prover.test.near".parse().unwrap(),
            Address::zero(),
            FungibleTokenMetadata::default(),
        )
        .transact()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_factory_update_address_version() {
    let contract = utils::deploy_and_init_contract().await.unwrap();

    contract
        .factory_update_address_version(Address::zero(), 0)
        .transact()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_refund_on_error() {
    let contract = utils::deploy_and_init_contract().await.unwrap();

    contract
        .refund_on_error(Address::zero(), Some(Address::zero()), 0.into())
        .transact()
        .await
        .unwrap();
}
