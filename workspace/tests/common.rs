use aurora_workspace::contract::EthProverConfig;
use aurora_workspace::{types::AccountId, EvmContract, InitConfig};
use std::str::FromStr;
use workspaces::types::{KeyType, SecretKey};

const EVM_ACCOUNT_ID: &str = "aurora.test.near";
const AURORA_LOCAL_CHAIN_ID: u64 = 1313161556;
const AURORA_ACCOUNT_ID: &str = "aurora.test.near";
const OWNER_ACCOUNT_ID: &str = "owner.test.near";
const PROVER_ACCOUNT_ID: &str = "prover.test.near";
const EVM_CUSTODIAN_ADDRESS: &str = "096DE9C2B8A5B8c22cEe3289B101f6960d68E51E";

pub async fn init_and_deploy_contract() -> anyhow::Result<EvmContract> {
    let worker = workspaces::sandbox().await?;
    let sk = SecretKey::from_random(KeyType::ED25519);
    let evm_account = worker
        .create_tla(AccountId::from_str("aurora.test.near")?, sk)
        .await?
        .into_result()?;
    let eth_prover_config = EthProverConfig::default();
    let init_config = InitConfig {
        owner_id: AccountId::from_str("owner.test.near")?,
        prover_id: AccountId::from_str("prover.test.near")?,
        eth_prover_config: Some(eth_prover_config),
        // eth_prover_config: None,
        chain_id: AURORA_LOCAL_CHAIN_ID.into(),
    };
    let wasm = std::fs::read("../res/bin/main.wasm")?;
    // create contract
    let contract = EvmContract::deploy_and_init(evm_account, init_config, wasm).await?;

    Ok(contract)
}
