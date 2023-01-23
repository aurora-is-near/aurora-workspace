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
}