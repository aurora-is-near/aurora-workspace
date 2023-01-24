use std::thread::AccessError;
use crate::*;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};

#[near_bindgen]
impl MockEvmContract {
    pub fn get_version(&self) -> String {
        "v1".to_string()
    }

    #[result_serializer(borsh)]
    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    #[result_serializer(borsh)]
    pub fn get_chain_id(&self) -> [u8; 32]{
        self.chain_id
    }

    #[result_serializer(borsh)]
    pub fn get_bridge_prover(&self) -> AccountId {
        self.bridge_prover_id.clone()
    }

    #[result_serializer(borsh)]
    pub fn get_upgrade_index(&self) -> u64 {
        self.upgrade_delay_blocks
    }

    #[result_serializer(borsh)]
    pub fn get_paused_precompiles(&self) -> u32 {
        0
    }

    // pub fn get_block_hash(&self, #[serializer(borsh)] block_height: u64) -> H256 {
    //     H256::from([0u8; 32])
    // }
}