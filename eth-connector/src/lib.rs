use crate::contract::EthConnectorContract;
use aurora_workspace_utils::Contract;
use std::path::Path;
use workspaces::types::Balance;
use workspaces::Account;

pub mod contract;
pub mod operation;
pub mod types;

const ROOT_ACCOUNT: &str = "root";
const ETH_CONNECTOR_ACCOUNT: &str = "eth_connector";
const ROOT_BALANCE: Balance = near_units::parse_near!("200 N");
const CONTRACT_BALANCE: Balance = near_units::parse_near!("85 N");

/// Deploy eth-connector contract using provided WASM file.
pub async fn deploy<P: AsRef<Path> + Copy>(
    path: P,
) -> anyhow::Result<(EthConnectorContract, Account)> {
    let root_account = Contract::create_root_account(ROOT_ACCOUNT, ROOT_BALANCE).await?;
    let eth_connector = root_account
        .create_subaccount(ETH_CONNECTOR_ACCOUNT)
        .initial_balance(CONTRACT_BALANCE)
        .transact()
        .await?
        .into_result()?;
    // Explicitly read contract file
    let contract_data = std::fs::read(path).map_err(|_| {
        anyhow::anyhow!(
            "Failed read contract in path: {} file: aurora-eth-connector-test.wasm",
            path.as_ref().display()
        )
    })?;
    assert_eq!(
        eth_connector.view_account().await?.balance,
        CONTRACT_BALANCE
    );
    let contract = Contract::deploy(&eth_connector, contract_data).await?;

    Ok((EthConnectorContract::new(contract), root_account))
}
