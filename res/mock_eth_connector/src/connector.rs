use crate::Proof;
use aurora_engine_types::types::Address;
use near_contract_standards::storage_management::StorageBalance;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    ext_contract,
    json_types::{U128, U64},
    AccountId, Balance, Promise, PromiseOrValue,
};

pub const CUSTODIAN_ADDRESS: &str = "096DE9C2B8A5B8c22cEe3289B101f6960d68E51E";

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub struct FinishDepositCallArgs {
    pub new_owner_id: AccountId,
    pub amount: Balance,
    pub proof_key: String,
    pub msg: Option<Vec<u8>>,
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct WithdrawResult {
    pub amount: Balance,
    pub recipient_id: Address,
    pub eth_custodian_address: Address,
}

#[ext_contract(ext_deposit)]
pub trait ConnectorDeposit {
    fn deposit(&mut self, #[serializer(borsh)] raw_proof: Proof) -> Promise;
}

#[ext_contract(ext_withdraw)]
pub trait ConnectorWithdraw {
    #[result_serializer(borsh)]
    fn withdraw(
        &mut self,
        #[serializer(borsh)] recipient_address: Address,
        #[serializer(borsh)] amount: Balance,
    ) -> WithdrawResult;

    #[result_serializer(borsh)]
    fn engine_withdraw(
        &mut self,
        #[serializer(borsh)] sender_id: AccountId,
        #[serializer(borsh)] recipient_address: Address,
        #[serializer(borsh)] amount: Balance,
    ) -> WithdrawResult;
}

#[ext_contract(ext_funds_finish)]
pub trait ConnectorFundsFinish {
    fn finish_deposit(
        &mut self,
        #[serializer(borsh)] deposit_call: FinishDepositCallArgs,
        #[callback_unwrap]
        #[serializer(borsh)]
        verify_log_result: bool,
    ) -> PromiseOrValue<Option<U128>>;
}

#[ext_contract(ext_ft_statistic)]
pub trait FungibleTokeStatistic {
    fn get_accounts_counter(&self) -> U64;
}

/// Engine compatible methods for NEP-141
#[ext_contract(ext_enine_ft)]
pub trait EngineFungibleToken {
    fn engine_ft_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    );

    fn engine_ft_transfer_call(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

/// Engine compatible methods for NEP-141
#[ext_contract(ext_enine_storage)]
pub trait EngineStorageManagement {
    fn engine_storage_deposit(
        &mut self,
        sender_id: AccountId,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance;

    fn engine_storage_withdraw(
        &mut self,
        sender_id: AccountId,
        amount: Option<U128>,
    ) -> StorageBalance;

    fn engine_storage_unregister(&mut self, sender_id: AccountId, force: Option<bool>) -> bool;
}

#[ext_contract(ext_known_engine_accounts)]
pub trait KnownEngineAccountsManagement {
    fn set_engine_account(&mut self, engine_account: &AccountId);

    fn remove_engine_account(&mut self, engine_account: &AccountId);

    fn is_engine_account_exist(&self, engine_account: &AccountId) -> bool;
}
