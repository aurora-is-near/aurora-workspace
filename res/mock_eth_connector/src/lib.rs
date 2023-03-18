#![allow(unused_variables)]
use crate::proof::Proof;
use aurora_eth_connector::connector::EngineFungibleToken;
use near_contract_standards::fungible_token::core::FungibleTokenCore;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{assert_one_yocto, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue};
use std::str::FromStr;

mod proof;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct EthConnectorContract;

#[near_bindgen]
impl EthConnectorContract {
    #[init]
    pub fn new(
        prover_account: AccountId,
        eth_custodian_address: String,
        metadata: FungibleTokenMetadata,
        account_with_access_right: AccountId,
        owner_id: AccountId,
    ) -> Self {
        Self
    }

    #[result_serializer(borsh)]
    pub fn is_used_proof(&self, #[serializer(borsh)] proof: Proof) -> bool {
        true
    }

    pub fn get_bridge_prover(&self) -> AccountId {
        AccountId::from_str("bridge_prover.root").unwrap()
    }
}

#[near_bindgen]
impl FungibleTokenCore for EthConnectorContract {
    #[payable]
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {}

    #[payable]
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_one_yocto();
        PromiseOrValue::Value(U128::from(100))
    }

    fn ft_total_supply(&self) -> U128 {
        U128::from(100)
    }

    fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        U128::from(200)
    }
}

#[near_bindgen]
impl EngineFungibleToken for EthConnectorContract {
    #[payable]
    fn engine_ft_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    ) {
    }

    #[payable]
    fn engine_ft_transfer_call(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_one_yocto();
        PromiseOrValue::Value(U128::from(100))
    }
}
