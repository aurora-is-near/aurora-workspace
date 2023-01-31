use crate::*;
use aurora_workspace_types::input::{FungibleTokenMetadata, StorageBalance, IsUsedProofCallArgs};

#[near_bindgen]
impl MockEvmContract {
    pub fn get_version(&self) -> String {
        "v1".to_string()
    }

    pub fn ft_total_supply(&self) -> u128 {
        0u128
    }

    pub fn ft_balance_of(&self, #[serializer(borsh)] account: String) -> u128 {
        0u128
    }

    #[result_serializer(borsh)]
    pub fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata::default()
    }

    #[result_serializer(borsh)]
    pub fn ft_total_eth_supply_on_aurora(&self) ->  [u64; 4] {
        [0u64; 4]
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
        0u32
    }

    #[result_serializer(borsh)]
    pub fn get_block_hash(&self, #[serializer(borsh)] _index: u64) -> [u8; 32] {
        [0u8; 32]
    }

    #[result_serializer(borsh)]
    pub fn get_balance(&self, #[serializer(borsh)] _address: [u8; 20]) -> [u64; 4] {
        [0u64; 4]
    }

    #[result_serializer(borsh)]
    pub fn get_nonce(&self, #[serializer(borsh)] _address: [u8; 20]) -> [u64; 4] {
        [0u64; 4]
    }

    #[result_serializer(borsh)]
    pub fn ft_balance_of_eth(&self, #[serializer(borsh)] _address: [u8; 20]) -> [u64; 4] {
        [0u64; 4]
    }

    pub fn storage_balance_of(&self, #[serializer(borsh)] account: AccountId) -> StorageBalance {
        StorageBalance::default()
    }

    pub fn is_used_proof(&self, #[serializer(borsh)] proof: IsUsedProofCallArgs) -> bool {
        true
    }
}