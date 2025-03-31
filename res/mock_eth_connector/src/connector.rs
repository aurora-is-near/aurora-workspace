use aurora_engine_types::types::Address;
use near_contract_standards::storage_management::StorageBalance;
use near_sdk::{
    ext_contract,
    json_types::U128,
    AccountId, PromiseOrValue,
};

type Balance = u128;

#[ext_contract(ext_withdraw)]
pub trait ConnectorWithdraw {
    #[result_serializer(borsh)]
    fn withdraw(
        &mut self,
        #[serializer(borsh)] recipient_address: Address,
        #[serializer(borsh)] amount: Balance,
    );

    #[result_serializer(borsh)]
    fn engine_withdraw(
        &mut self,
        #[serializer(borsh)] sender_id: AccountId,
        #[serializer(borsh)] recipient_address: Address,
        #[serializer(borsh)] amount: Balance,
    );
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
