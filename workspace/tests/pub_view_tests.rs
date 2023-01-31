use aurora_workspace::operation::ViewResultDetails;
use aurora_workspace::types::AccountId;
// use aurora_engine::fungible_token::FungibleTokenMetadata;
use aurora_engine::parameters::StorageBalanceOfCallArgs;
use aurora_workspace_types::input::StorageBalance;
use ethereum_types::{H256, U256};
use std::str::FromStr;

mod common;

#[tokio::test]
async fn test_version() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract.as_account().version().await?;
    let expected = ViewResultDetails {
        result: "\"v1\"".to_string().into(),
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_owner() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract.as_account().owner().await?;
    let expected = ViewResultDetails {
        result: AccountId::from_str("owner.test.near").expect("Invalid account"),
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_chain_id() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract.as_account().chain_id().await?;
    let expected = ViewResultDetails {
        result: 1313161556u128,
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_bridge_prover() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract.as_account().bridge_prover().await?;
    let expected = ViewResultDetails {
        result: AccountId::from_str("prover.test.near").expect("Invalid account"),
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_upgrade_index() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract.as_account().upgrade_index().await?;
    let expected = ViewResultDetails {
        result: 1,
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_paused_precompiles() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract.as_account().paused_precompiles().await?;
    let expected = ViewResultDetails {
        result: 0,
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_block_hash() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract
        .as_account()
        .block_hash(0u64)
        .await?;
    let expected = ViewResultDetails {
        result: H256::from([0u8; 32]),
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_balance() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract
        .as_account()
        .balance([0u8; 20])
        .await?;
    let expected = ViewResultDetails {
        result: 0u128,
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_nonce() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract
        .as_account()
        .nonce([0u8; 20])
        .await?;
    let expected = ViewResultDetails {
        result: 0u128,
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

// #[tokio::test]
// async fn test_is_proof_used() -> anyhow::Result<()> {
//     let contract = common::init_and_deploy_contract().await?;
//     let res = contract
//         .as_account()
//         .is_proof_used(ProofArgs)
//         .await?;
//     let expected = ViewResultDetails {
//         result: false,
//         logs: vec![],
//     };
//     assert_eq!(res, expected);
//     Ok(())
// }

#[tokio::test]
async fn test_ft_total_supply() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract
        .as_account()
        .ft_total_supply()
        .await?;
    let expected = ViewResultDetails {
        result: 0u128,
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

// #[tokio::test]
// async fn test_ft_balance_of() -> anyhow::Result<()> {
//     let contract = common::init_and_deploy_contract().await?;
//     let res = contract
//         .as_account()
//         .ft_balance_of("some_account.test")
//         .await?;
//     let expected = ViewResultDetails {
//         result: 0u128,
//         logs: vec![],
//     };
//     assert_eq!(res, expected);
//     Ok(())
// }
//
// #[tokio::test]
// async fn test_ft_metadata() -> anyhow::Result<()> {
//     let contract = common::init_and_deploy_contract().await?;
//     let res = contract
//         .as_account()
//         .ft_metadata()
//         .await?;
//     let expected = ViewResultDetails {
//         result: FungibleTokenMetadata::default(),
//         logs: vec![],
//     };
//     assert_eq!(res, expected);
//     Ok(())
// }

#[tokio::test]
async fn test_storage_balance_of() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let _res = contract
        .as_account()
        .storage_balance_of("account.test.near")
        .await?;
    // let expected = ViewResultDetails {
    //     result: StorageBalance::default(),
    //     logs: vec![],
    // };
    // assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_eth_balance_of() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract
        .as_account()
        .eth_balance_of([0u8; 20])
        .await?;
    let expected = ViewResultDetails {
        result: U256::from(0),
        logs: vec![],
    };
    assert_eq!(res,expected);
    Ok(())
}


