use aurora_engine_types::types::Address;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance, StorageUsage};
use std::collections::HashMap;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MigrationInputData {
    pub accounts: HashMap<AccountId, Balance>,
    pub total_supply: Option<Balance>,
    pub account_storage_usage: Option<StorageUsage>,
    pub statistics_aurora_accounts_counter: Option<u64>,
    pub used_proofs: Vec<String>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, Eq, PartialEq)]
pub enum MigrationCheckResult {
    Success,
    AccountNotExist(Vec<AccountId>),
    AccountAmount(HashMap<AccountId, Balance>),
    TotalSupply(Balance),
    StorageUsage(StorageUsage),
    StatisticsCounter(u64),
    Proof(Vec<String>),
}

#[derive(Debug, Default, BorshDeserialize, BorshSerialize, Clone)]
pub struct Proof {
    pub log_index: u64,
    pub log_entry_data: Vec<u8>,
    pub receipt_index: u64,
    pub receipt_data: Vec<u8>,
    pub header_data: Vec<u8>,
    pub proof: Vec<Vec<u8>>,
}

pub type PausedMask = u8;

pub const UNPAUSE_ALL: PausedMask = 0;
pub const PAUSE_DEPOSIT: PausedMask = 1 << 0;
pub const PAUSE_WITHDRAW: PausedMask = 1 << 1;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct WithdrawResult {
    pub amount: Balance,
    pub recipient_id: Address,
    pub eth_custodian_address: Address,
}
