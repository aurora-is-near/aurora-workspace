use aurora_engine::fungible_token::FungibleTokenMetadata;
use aurora_workspace::operation::ViewResultDetails;
use aurora_workspace::types::AccountId;
use aurora_workspace_types::input::{IsUsedProofCallArgs, ProofInput, StorageBalance};
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
    let res = contract.as_account().block_hash(0u64).await?;
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
    let res = contract.as_account().balance([0u8; 20]).await?;
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
    let res = contract.as_account().nonce([0u8; 20]).await?;
    let expected = ViewResultDetails {
        result: 0u128,
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_is_proof_used() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let proof = ProofInput::default();
    let res = contract.as_account().is_proof_used(proof).await?;
    let expected = ViewResultDetails {
        result: true,
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_ft_total_supply() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract.as_account().ft_total_supply().await?;
    let expected = ViewResultDetails {
        result: 0u128,
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_ft_balance_of() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract
        .as_account()
        .ft_balance_of("some_account.test")
        .await?;
    let expected = ViewResultDetails {
        result: 0u128,
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_ft_metadata() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract.as_account().ft_metadata().await?;
    let expected = ViewResultDetails {
        result: FungibleTokenMetadata::default(),
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_eth_total_supply() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract.as_account().eth_total_supply().await?;
    let expected = ViewResultDetails {
        result: U256::from(0),
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_storage_balance_of() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let _res = contract
        .as_account()
        .storage_balance_of("account.test.near")
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_eth_balance_of() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    let res = contract.as_account().eth_balance_of([0u8; 20]).await?;
    let expected = ViewResultDetails {
        result: U256::from(0),
        logs: vec![],
    };
    assert_eq!(res, expected);
    Ok(())
}

#[tokio::test]
async fn test_version() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;

    let res = contract.as_account().version().await?;

    let expected = ViewResultDetails {
        result: "\"v1\"".to_string(),
        logs: vec![],
    };
    assert_eq!(res, expected);

    Ok(())
}

#[tokio::test]
async fn test_view() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;

    let res = contract
        .as_account()
        .view(
            Address::from([0u8; 20]),
            Address::from([0u8; 20]),
            [0; 32],
            Vec::new(),
        )
        .await?;

    let expected = ViewResultDetails {
        result: TransactionStatus::Succeed(vec![]),
        logs: vec![],
    };
    assert_eq!(res, expected);

    Ok(())
}

#[tokio::test]
async fn test_eth_total_supply() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;

    let res = contract.as_account().eth_total_supply().await?;

    let expected = ViewResultDetails {
        result: U256::default(),
        logs: vec![],
    };
    assert_eq!(res, expected);

    Ok(())
}

#[cfg(not(feature = "ethabi"))]
#[tokio::test]
async fn test_code() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;

    let res = contract.as_account().code(Address::from([0u8; 20])).await?;

    let expected = ViewResultDetails {
        result: hex::decode(b"00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000000673706972616c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000067175617361720000000000000000000000000000000000000000000000000000").unwrap(),
        logs: Default::default(),
    };
    assert_eq!(res, expected);

    Ok(())
}

#[cfg(feature = "ethabi")]
#[tokio::test]
async fn test_code() -> anyhow::Result<()> {
    use ethabi::{ParamType, Token};

    let contract = common::init_and_deploy_contract().await?;

    let res = contract
        .as_account()
        .code(
            &[ParamType::Tuple(vec![
                ParamType::String,
                ParamType::Bool,
                ParamType::String,
            ])],
            Address::from([0u8; 20]),
        )
        .await?;

    let expected = ViewResultDetails {
        result: vec![Token::Tuple(vec![
            Token::String("spiral".into()),
            Token::Bool(true),
            Token::String("quasar".into()),
        ])],
        logs: Default::default(),
    };
    assert_eq!(res, expected);

    Ok(())
}

#[tokio::test]
async fn test_storage() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;

    let res = contract
        .as_account()
        .storage(Address::from([0u8; 20]), [0; 32])
        .await?;

    let expected = ViewResultDetails {
        result: H256::from([1; 32]),
        logs: Default::default(),
    };
    assert_eq!(res, expected);

    Ok(())
}

#[tokio::test]
async fn test_erc20_from_nep141() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;

    let res = contract
        .as_account()
        .erc20_from_nep141("nep141.test.near".parse().unwrap())
        .await?;

    let expected = ViewResultDetails {
        result: "erc20.test.near".parse().unwrap(),
        logs: Default::default(),
    };
    assert_eq!(res, expected);

    Ok(())
}

#[tokio::test]
async fn test_nep141_from_erc20() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;

    let res = contract
        .as_account()
        .nep141_from_erc20("erc20.test.near".parse().unwrap())
        .await?;

    let expected = ViewResultDetails {
        result: "nep141.test.near".parse().unwrap(),
        logs: Default::default(),
    };
    assert_eq!(res, expected);

    Ok(())
}

#[tokio::test]
async fn test_paused_flags() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;

    let res = contract.as_account().paused_flags().await?;

    let expected = ViewResultDetails {
        result: 0u8,
        logs: Default::default(),
    };
    assert_eq!(res, expected);

    Ok(())
}
