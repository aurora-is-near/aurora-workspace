use crate::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};

/// Json-encoded parameters for the `new` function.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct NewInput {
    /// Chain id, according to the EIP-115 / ethereum-lists spec.
    pub chain_id: [u8; 32],
    /// Account which can upgrade this contract.
    /// Use empty to disable updatability.
    pub owner_id: AccountId,
    /// Account of the bridge prover.
    /// Use empty to not use base token as bridged asset.
    pub bridge_prover_id: AccountId,
    /// How many blocks after staging upgrade can deploy it.
    pub upgrade_delay_blocks: u64,
}

#[derive(Debug, Eq, PartialEq, Clone, BorshSerialize, BorshDeserialize)]
pub struct DeployErc20Input {
    pub nep141: AccountId,
}

#[derive(Debug, Eq, PartialEq, Clone, BorshSerialize, BorshDeserialize)]
pub struct CallInput {
    pub contract: [u8; 20],
    pub value: [u8; 32],
    pub input: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct FtOnTransferInput {
    pub sender_id: AccountId,
    pub amount: U128,
    pub msg: String,
}

// #[cfg(feature = "deposit-withdraw")]
#[derive(Debug, Default, Eq, PartialEq, Clone, BorshSerialize, BorshDeserialize)]
pub struct ProofInput {
    pub log_index: u64,
    pub log_entry_data: Vec<u8>,
    pub receipt_index: u64,
    pub receipt_data: Vec<u8>,
    pub header_data: Vec<u8>,
    pub proof: Vec<Vec<u8>>,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, BorshSerialize, BorshDeserialize)]
pub struct IsUsedProofCallArgs {
    /// Proof data
    pub proof: ProofInput,
}

#[cfg(feature = "deposit-withdraw")]
#[derive(Debug, Eq, PartialEq, Clone, BorshSerialize, BorshDeserialize)]
pub struct WithdrawInput {
    pub recipient_address: [u8; 20],
    pub amount: u128,
}
