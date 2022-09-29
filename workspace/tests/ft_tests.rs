mod common;

use std::path::PathBuf;
use std::str::FromStr;
use lazy_static::lazy_static;
use near_account_id::AccountId;
use workspaces::Account;
use workspaces::network::Sandbox;
use workspaces::types::{KeyType, SecretKey};
use aurora_workspace::{InitConfig, EvmContract};

lazy_static! {
    static ref DEFAULT_AURORA_ACCOUNT_ID: AccountId =
        AccountId::from_str("aurora.test.near").unwrap();
    static ref DEFAULT_OWNER_ACCOUNT_ID: AccountId =
        AccountId::from_str("owner.test.near").unwrap();
    static ref DEFAULT_PROVER_ACCOUNT_ID: AccountId =
        AccountId::from_str("prover.test.near").unwrap();
}

#[tokio::test]
async fn testy() -> anyhow::Result<()> {
    let contract = common::init_and_deploy_contract().await?;
    println!("{:?}", contract);

    Ok(())
}
