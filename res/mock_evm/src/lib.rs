use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, Balance, AccountId, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};
use aurora_workspace::input::NewInput;

mod metadata;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MockEvmContract {
    pub chain_id: [u8; 32],
    pub owner_id: AccountId,
    pub bridge_prover_id: AccountId,
    pub upgrade_delay_blocks: u64,
}

#[near_bindgen]
impl MockEvmContract {
    #[init]
    pub fn new(#[serializer(borsh)] new: NewInput) -> MockEvmContract {
        MockEvmContract {
            chain_id: new.chain_id,
            owner_id: new.owner_id,
            bridge_prover_id: new.bridge_prover_id,
            upgrade_delay_blocks: new.upgrade_delay_blocks,
        }
    }
}
