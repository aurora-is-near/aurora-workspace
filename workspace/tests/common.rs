use aurora_workspace::{types::AccountId, EvmContract, InitConfig};
use std::str::FromStr;
use workspaces::types::{KeyType, SecretKey};

const AURORA_LOCAL_CHAIN_ID: u64 = 1313161556;
const AURORA_ACCOUNT_ID: &str = "aurora.test.near";
const OWNER_ACCOUNT_ID: &str = "owner.test.near";
const PROVER_ACCOUNT_ID: &str = "prover.test.near";
#[allow(dead_code)]
const EVM_CUSTODIAN_ADDRESS: &str = "096DE9C2B8A5B8c22cEe3289B101f6960d68E51E";
const WASM_BIN_FILE_PATH: &str = "../res/bin/main.wasm";

pub async fn init_and_deploy_contract() -> anyhow::Result<EvmContract> {
    let worker = workspaces::sandbox().await?;
    let sk = SecretKey::from_random(KeyType::ED25519);
    let evm_account = worker
        .create_tla(AccountId::from_str(AURORA_ACCOUNT_ID)?, sk)
        .await?
        .into_result()?;
    let init_config = InitConfig {
        chain_id: AURORA_LOCAL_CHAIN_ID.into(),
        owner_id: AccountId::from_str(OWNER_ACCOUNT_ID)?,
        prover_id: AccountId::from_str(PROVER_ACCOUNT_ID)?,
    };
    let wasm = std::fs::read(WASM_BIN_FILE_PATH)?;
    // create contract
    let contract = EvmContract::deploy_and_init(evm_account, init_config, wasm).await?;

    Ok(contract)
}
