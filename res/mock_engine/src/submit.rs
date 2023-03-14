use aurora_workspace_types::output::{Log, TransactionStatus};
use aurora_workspace_types::{Address, H256};
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

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

    pub fn dummy_submit_result() -> Self {
        let log = Log::new(
            Address::from([1u8; 20]),
            vec![H256::from([2u8; 32])],
            vec![3u8; 10],
        );
        Self::new(TransactionStatus::Succeed(vec![0]), 100_000, vec![log])
    }
}
