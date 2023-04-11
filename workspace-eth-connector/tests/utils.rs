/*
use aurora_workspace_eth_connector::contract::EthConnectorContract;
use aurora_workspace_types::AccountId;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use std::str::FromStr;
use workspaces::types::{KeyType, SecretKey};

pub const CONTRACT_ACCOUNT_ID: &str = "eth_connector.test.near";
pub const CUSTODIAN_ADDRESS: &str = "096DE9C2B8A5B8c22cEe3289B101f6960d68E51E";
pub const OWNER_ID: &str = "aurora.test.near";
pub const PROVER_ID: &str = "prover.test.near";
const WASM_BIN_FILE_PATH: &str = "../bin/mock_eth_connector.wasm";

pub async fn init_and_deploy_contract() -> anyhow::Result<EthConnectorContract> {
    let worker = workspaces::sandbox()
        .await
        .map_err(|err| anyhow::anyhow!("Failed init sandbox: {:?}", err))?;
    let sk = SecretKey::from_random(KeyType::ED25519);
    let account = worker
        .create_tla(AccountId::from_str(CONTRACT_ACCOUNT_ID)?, sk)
        .await?
        .into_result()?;
    let wasm = std::fs::read(WASM_BIN_FILE_PATH)
        .map_err(|e| anyhow::anyhow!("failed read wasm file: {e}"))?;
    let prover_account = AccountId::from_str(PROVER_ID)?;
    let eth_custodian_address = CUSTODIAN_ADDRESS.to_string();
    let metadata = FungibleTokenMetadata {
        spec: String::from("1.0.0"),
        symbol: String::default(),
        name: String::default(),
        icon: None,
        reference: None,
        reference_hash: None,
        decimals: 0,
    };
    let account_with_access_right = account.clone();
    let owner_id = AccountId::from_str(OWNER_ID)?;
    let contract = EthConnectorContract::deploy_and_init(
        account,
        prover_account,
        eth_custodian_address,
        metadata,
        account_with_access_right.id(),
        owner_id,
        wasm,
    )
    .await?;
    Ok(contract)
}
*/
