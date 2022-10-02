use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use aurora_workspace_types::output::{Log, TransactionStatus};

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, Serialize, Deserialize)]
pub struct SubmitResult {
    version: u8,
    status: TransactionStatus,
    gas_used: u64,
    logs: Vec<Log>,
}

impl SubmitResult {
    pub const VERSION: u8 = 7;

    pub fn new(status: TransactionStatus, gas_used: u64, logs: Vec<Log>) -> Self {
        Self {
            version: Self::VERSION,
            status,
            gas_used,
            logs,
        }
    }
}
