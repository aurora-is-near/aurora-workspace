use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    near_bindgen, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};
use aurora_workspace_types::AccountId;
use aurora_workspace_types::input::{NewInput, SetEthConnectorInput};
use aurora_workspace_types::output::SubmitReturn;

mod metadata;
mod storage;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MockEvmContract {
    pub chain_id: [u8; 32],
    pub owner_id: AccountId,
    pub bridge_prover_id: AccountId,
    pub upgrade_delay_blocks: u64,
    pub eth_connector: Option<SetEthConnectorInput>,
}

#[near_bindgen]
impl MockEvmContract {
    #[init]
    pub fn new(chain_id: [u8; 32], owner_id: AccountId, bridge_prover_id: AccountId, upgrade_delay_blocks: u64) -> MockEvmContract {
        MockEvmContract {
            chain_id,
            owner_id,
            bridge_prover_id,
            upgrade_delay_blocks,
            eth_connector: None,
        }
    }

    pub fn new_eth_connector(&mut self, #[serializer(borsh)] input: SetEthConnectorInput) {
        self.eth_connector = Some(input);
    }

    #[result_serializer(borsh)]
    pub fn deploy_code(&mut self, #[serializer(borsh)] input: Vec<u8>) -> SubmitReturn {

    }
}
