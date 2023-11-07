use aurora_workspace_engine::EngineContract;
use aurora_workspace_utils::Contract;
use ethereum_types::U256;
use near_workspaces::types::{KeyType, SecretKey};

const AURORA_LOCAL_CHAIN_ID: u64 = 1313161556;
const AURORA_ACCOUNT_ID: &str = "aurora.test.near";
const OWNER_ACCOUNT_ID: &str = "owner.test.near";
const WASM_BIN_FILE_PATH: &str = "../bin/mock_engine.wasm";

pub async fn deploy_and_init_contract() -> anyhow::Result<EngineContract> {
    let worker = near_workspaces::sandbox()
        .await
        .map_err(|err| anyhow::anyhow!("Failed init sandbox: {:?}", err))?;
    let sk = SecretKey::from_random(KeyType::ED25519);
    let evm_account = worker
        .create_tla(AURORA_ACCOUNT_ID.parse()?, sk)
        .await?
        .into_result()?;
    let wasm = std::fs::read(WASM_BIN_FILE_PATH)
        .map_err(|e| anyhow::anyhow!("failed read wasm file: {e}"))?;
    // create contract
    let contract = Contract::deploy(&evm_account, wasm).await?;
    let engine_contract = EngineContract::new_from_contract(contract, evm_account);

    engine_contract
        .new(
            into_chain_id(AURORA_LOCAL_CHAIN_ID),
            OWNER_ACCOUNT_ID.parse().unwrap(),
            1,
        )
        .transact()
        .await
        .unwrap();

    Ok(engine_contract)
}

fn into_chain_id(value: u64) -> [u8; 32] {
    let chain_id = U256::from(value);
    let mut result = [0; 32];
    chain_id.to_big_endian(&mut result);

    result
}
