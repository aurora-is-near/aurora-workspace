use aurora_workspace_eth_connector::operation::ViewResultDetails;
use aurora_workspace_eth_connector::types::{
    MigrationCheckResult, MigrationInputData, Proof, UNPAUSE_ALL,
};
use aurora_workspace_types::AccountId;
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_sdk::json_types::{U128, U64};
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

#[tokio::test]
async fn test_set_engine_account() {}

#[tokio::test]
async fn test_remove_engine_account() {}

#[tokio::test]
async fn test_get_engine_accounts() {}

#[tokio::test]
async fn test_storage_deposit() {}

#[tokio::test]
async fn test_storage_withdraw() {}

#[tokio::test]
async fn test_storage_unregister() {}

#[tokio::test]
async fn test_engine_storage_deposit() {}

#[tokio::test]
async fn test_engine_storage_withdraw() {}

#[tokio::test]
async fn test_engine_storage_unregister() {}

#[tokio::test]
async fn test_storage_balance_of() {}

#[tokio::test]
async fn test_ft_resolve_transfer() {}

#[tokio::test]
async fn test_storage_balance_bounds() {}

#[tokio::test]
async fn test_set_paused_flags() {}

#[tokio::test]
async fn test_set_access_right() {}

#[tokio::test]
async fn test_withdraw() {}

#[tokio::test]
async fn test_deposit() {}

#[tokio::test]
async fn test_migrate() {}

#[tokio::test]
async fn test_ft_metadata() {
    let contract = utils::init_and_deploy_contract().await.unwrap();
    let res = contract.as_account().ft_metadata().await.unwrap();
    let expected =  FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            icon: Some("data:image/png;base64,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".to_string()),
            reference: None,
            reference_hash: None,
            decimals: 18,
        };
    assert_eq!(res.result.spec, expected.spec);
    assert_eq!(res.result.name, expected.name);
    assert_eq!(res.result.symbol, expected.symbol);
    assert_eq!(res.result.icon, expected.icon);
    assert_eq!(res.result.reference, expected.reference);
    assert_eq!(res.result.reference_hash, expected.reference_hash);
    assert_eq!(res.result.decimals, expected.decimals);
}

#[tokio::test]
async fn test_get_accounts_counter() {
    let contract = utils::init_and_deploy_contract().await.unwrap();
    let res = contract.as_account().get_accounts_counter().await.unwrap();
    let expected = ViewResultDetails {
        result: U64::from(10),
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_get_access_right() {
    let contract = utils::init_and_deploy_contract().await.unwrap();
    let res = contract.as_account().get_access_right().await.unwrap();
    let expected = ViewResultDetails {
        result: AccountId::from_str("contract.root").unwrap(),
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_get_paused_flags() {
    let contract = utils::init_and_deploy_contract().await.unwrap();
    let res = contract.as_account().get_paused_flags().await.unwrap();
    let expected = ViewResultDetails {
        result: UNPAUSE_ALL,
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_is_owner() {
    let contract = utils::init_and_deploy_contract().await.unwrap();
    let res = contract.as_account().is_owner().await.unwrap();
    let expected = ViewResultDetails {
        result: true,
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_check_migration_correctness() {
    let contract = utils::init_and_deploy_contract().await.unwrap();
    let data = MigrationInputData::default();
    let res = contract
        .as_account()
        .check_migration_correctness(data)
        .await
        .unwrap();
    let expected = ViewResultDetails {
        result: MigrationCheckResult::Success,
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_is_used_proof() {
    let contract = utils::init_and_deploy_contract().await.unwrap();
    let proof = Proof::default();
    let res = contract.as_account().is_used_proof(proof).await.unwrap();
    let expected = ViewResultDetails {
        result: true,
        logs: vec![],
    };
    assert_eq!(res, expected);
}

#[tokio::test]
async fn test_get_bridge_prover() {
    let contract = utils::init_and_deploy_contract().await.unwrap();
    let res = contract.as_account().get_bridge_prover().await.unwrap();
    let expected = ViewResultDetails {
        result: AccountId::from_str("bridge_prover.root").unwrap(),
        logs: vec![],
    };
    assert_eq!(res, expected);
}
