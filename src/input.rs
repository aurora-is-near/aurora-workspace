use borsh::{BorshSerialize, BorshDeserialize};
use near_account_id::AccountId;

/// Borsh-encoded parameters for the `new` function.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
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
