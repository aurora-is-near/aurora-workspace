use crate::*;
use near_sdk::json_types::U128;
use near_sdk::serde_json;
use near_sdk::PromiseOrValue;

#[near_bindgen]
impl MockEvmContract {
    #[allow(unused_variables)]
    #[payable]
    pub fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        use std::str::FromStr;
        assert_eq!(
            receiver_id,
            AccountId::from_str("some_account.test").unwrap()
        );
        assert_eq!(amount.0, 10);
        assert_eq!(memo, Some("some message".to_string()));
    }

    #[allow(unused_variables)]
    pub fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> String {
        serde_json::to_string(&0).expect("Failed to serialize message")
    }

    #[allow(unused_variables)]
    #[payable]
    pub fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        PromiseOrValue::Value(amount)
    }
}
