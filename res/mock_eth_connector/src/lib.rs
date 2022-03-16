use aurora_workspace_types::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MockEthConnectorContract {
    pub chain_id: [u8; 32],
    pub owner_id: AccountId,
    pub bridge_prover_id: AccountId,
    pub upgrade_delay_blocks: u64,
}

#[near_bindgen]
impl MockEthConnectorContract {}
