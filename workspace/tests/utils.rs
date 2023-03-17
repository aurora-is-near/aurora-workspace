use aurora_workspace::{types::AccountId, EvmContract, InitConfig};
use std::str::FromStr;
use workspaces::types::{KeyType, SecretKey};

const AURORA_LOCAL_CHAIN_ID: u64 = 1313161556;
const AURORA_ACCOUNT_ID: &str = "aurora.test.near";
const OWNER_ACCOUNT_ID: &str = "owner.test.near";
const PROVER_ACCOUNT_ID: &str = "prover.test.near";
const WASM_BIN_FILE_PATH: &str = "../bin/mock_engine.wasm";

pub async fn init_and_deploy_contract() -> anyhow::Result<EvmContract> {
    let worker = workspaces::sandbox()
        .await
        .map_err(|err| anyhow::anyhow!("Failed init sandbox: {:?}", err))?;
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
    let wasm = std::fs::read(WASM_BIN_FILE_PATH)
        .map_err(|e| anyhow::anyhow!("failed read wasm file: {e}"))?;
    // create contract
    let contract = EvmContract::deploy_and_init(evm_account, init_config, wasm).await?;

    Ok(contract)
}
