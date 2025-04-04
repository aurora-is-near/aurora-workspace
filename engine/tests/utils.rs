use std::sync::LazyLock;

use aurora_workspace_engine::EngineContract;
use aurora_workspace_utils::{compile::compile_project, Contract};
use ethereum_types::U256;
use near_workspaces::types::{KeyType, SecretKey};

const AURORA_LOCAL_CHAIN_ID: u64 = 1313161556;
const AURORA_ACCOUNT_ID: &str = "aurora";
const OWNER_ACCOUNT_ID: &str = "owner";

pub static CONTRACT_WASM: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let wasm_path = compile_project("../res/mock_engine");
    std::fs::read(wasm_path)
        .map_err(|e| anyhow::anyhow!("failed read wasm file: {e}"))
        .unwrap()
});

pub async fn deploy_and_init_contract() -> anyhow::Result<EngineContract> {
    let worker = near_workspaces::sandbox()
        .await
        .map_err(|err| anyhow::anyhow!("Failed init sandbox: {:?}", err))?;
    let sk = SecretKey::from_random(KeyType::ED25519);
    let evm_account = worker
        .create_tla(AURORA_ACCOUNT_ID.parse()?, sk)
        .await?
        .into_result()?;

    // create contract
    let contract = Contract::deploy(&evm_account, CONTRACT_WASM.to_owned()).await?;
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
