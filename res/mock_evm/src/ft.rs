use crate::*;
use near_sdk::serde_json;

#[near_bindgen]
impl MockEvmContract {
    pub fn ft_on_transfer(&mut self, sender_id: AccountId, amount: u128, msg: String) -> String {
        serde_json::to_string(&0).expect("Failed to serialize message")
    }
}
