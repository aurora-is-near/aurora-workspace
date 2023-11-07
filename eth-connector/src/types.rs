use aurora_engine_types::borsh::{self, BorshDeserialize, BorshSerialize};
use aurora_engine_types::types::Address;
use near_sdk::{Balance, StorageUsage};
use near_workspaces::AccountId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct MigrationInputData {
    pub accounts: HashMap<AccountId, Balance>,
    pub total_supply: Option<Balance>,
    pub account_storage_usage: Option<StorageUsage>,
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

#[derive(Debug, Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
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

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct WithdrawResult {
    pub amount: Balance,
    pub recipient_id: Address,
    pub eth_custodian_address: Address,
}
