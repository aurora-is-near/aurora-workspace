use crate::*;
use near_sdk::serde_json;

#[near_bindgen]
impl MockEvmContract {
    pub fn ft_transfer(&mut self, receiver_id: String, amount: u128, memo: Option<String>) {
        assert_eq!(receiver_id, "some_account.test");
        assert_eq!(amount, 10);
        assert_eq!(memo, Some("some message".to_string()));
    }

    pub fn ft_on_transfer(&mut self, sender_id: AccountId, amount: u128, msg: String) -> String {
        serde_json::to_string(&0).expect("Failed to serialize message")
    }

    #[result_serializer(borsh)]
    pub fn ft_transfer_call(&mut self, receiver_id: AccountId, amount: u128, memo: Option<String>, msg: String) -> u64 {
        amount.try_into().unwrap()
    }
}
