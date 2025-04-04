#![allow(dead_code)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ext_contract, AccountId, StorageUsage};
use std::collections::HashMap;

type Balance = u128;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MigrationInputData {
    pub accounts: HashMap<AccountId, Balance>,
    pub total_supply: Option<Balance>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Eq, PartialEq)]
pub enum MigrationCheckResult {
    Success,
    AccountNotExist(Vec<AccountId>),
    AccountAmount(HashMap<AccountId, Balance>),
    TotalSupply(Balance),
    StorageUsage(StorageUsage),
    StatisticsCounter(u64),
}

#[ext_contract(ext_deposit)]
pub trait Migration {
    fn migrate(&mut self, #[serializer(borsh)] accounts: Vec<AccountId>);

    #[result_serializer(borsh)]
    fn check_migration_correctness(
        &self,
        #[serializer(borsh)] data: MigrationInputData,
    ) -> MigrationCheckResult;
}
