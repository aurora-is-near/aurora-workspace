use aurora_workspace_types::input::{FungibleTokenMetadata, IsUsedProofCallArgs, StorageBalance};
use aurora_workspace_types::{output::TransactionStatus, AccountId, Raw, U256};
use near_sdk::{borsh, near_bindgen};

use crate::{MockEvmContract, MockEvmContractExt};

#[near_bindgen]
impl MockEvmContract {
    pub fn get_version(&self) -> String {
        "v1".to_string()
    }

    #[result_serializer(borsh)]
    pub fn get_owner(&self) -> AccountId {
        "owner.test.near".parse().unwrap()
    }

    pub fn get_chain_id(&self) -> String {
        1313161556.to_string()
    }

    pub fn ft_total_eth_supply_on_aurora(&self) -> String {
        "0".into()
    }

    pub fn ft_total_supply(&self) -> u128 {
        0
    }

    pub fn ft_balance_of(&self, #[serializer(borsh)] _account: String) -> u128 {
        0
    }

    #[result_serializer(borsh)]
    pub fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata::default()
    }

    #[result_serializer(borsh)]
    pub fn get_view(&self, #[serializer(borsh)] _input: Raw) -> TransactionStatus {
        TransactionStatus::Succeed(vec![])
    }

    #[result_serializer(borsh)]
    pub fn get_code(&self, #[serializer(borsh)] _input: Raw) -> Raw {
        // `(string,bool,string) (spiral,true,quasar)`
        Raw(hex::decode(b"00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000000673706972616c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000067175617361720000000000000000000000000000000000000000000000000000").unwrap())
    }

    #[result_serializer(borsh)]
    pub fn get_storage_at(&self, #[serializer(borsh)] _input: Raw) -> [u8; 32] {
        [1; 32]
    }

    #[result_serializer(borsh)]
    pub fn get_erc20_from_nep141(&self, #[serializer(borsh)] _input: Raw) -> AccountId {
        "erc20.test.near".parse().unwrap()
    }

    #[result_serializer(borsh)]
    pub fn get_nep141_from_erc20(&self, #[serializer(borsh)] _input: Raw) -> AccountId {
        "nep141.test.near".parse().unwrap()
    }

    #[result_serializer(borsh)]
    pub fn get_paused_flags(&self, #[serializer(borsh)] _input: Raw) -> u8 {
        0
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

    pub fn storage_balance_of(&self, #[serializer(borsh)] _account: AccountId) -> StorageBalance {
        StorageBalance::default()
    }

    pub fn is_used_proof(&self, #[serializer(borsh)] _proof: IsUsedProofCallArgs) -> bool {
        true
    }
}
