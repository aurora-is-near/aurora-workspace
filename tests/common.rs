use std::str::FromStr;
use near_account_id::AccountId;
use workspaces::types::{KeyType, SecretKey};
use aurora_workspace::{InitConfig, EvmContract};

const EVM_ACCOUNT_ID: &str = "aurora.test.near";
const AURORA_LOCAL_CHAIN_ID: u64 = 1313161556;
const AURORA_ACCOUNT_ID: &str = "aurora.test.near";
const OWNER_ACCOUNT_ID: &str = "owner.test.near";
const PROVER_ACCOUNT_ID: &str = "prover.test.near";
const EVM_CUSTODIAN_ADDRESS: &str = "096DE9C2B8A5B8c22cEe3289B101f6960d68E51E";

pub(crate) async fn init_and_deploy_contract() -> anyhow::Result<EvmContract> {
    let worker = workspaces::sandbox().await?;
    let sk = SecretKey::from_random(KeyType::ED25519);
    let evm_account = worker.create_tla(AccountId::from_str("aurora.test.near")?, sk).await?.into_result()?;
    let init_config = InitConfig {
        owner_id: AccountId::from_str("owner.test.near")?,
        prover_id: AccountId::from_str("prover.test.near")?,
        eth_prover_config: None,
        chain_id: AURORA_LOCAL_CHAIN_ID.into(),
    };
    let wasm = std::fs::read("bin/main.wasm")?;
    // create contract
    let contract = EvmContract::deploy_and_init(evm_account, init_config, wasm).await?;

    Ok(contract)
}