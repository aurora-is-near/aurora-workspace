use crate::operation::{
    CallDeposit, CallEngineFtTransfer, CallEngineFtTransferCall, CallEngineStorageDeposit,
    CallEngineStorageUnregister, CallEngineStorageWithdraw, CallEngineWithdraw, CallFtTransfer,
    CallFtTransferCall, CallMigrate, CallNew, CallRemoveEngineAccount, CallSetAccessRight,
    CallSetEngineAccount, CallSetPausedFlags, CallStorageDeposit, CallStorageUnregister,
    CallStorageWithdraw, CallWithdraw, ViewCheckMigrationCorrectness, ViewFtBalanceOf,
    ViewFtMetadata, ViewFtTotalSupply, ViewGetAccountWithAccessRight, ViewGetBridgeProver,
    ViewGetPausedFlags, ViewIsEngineAccountExist, ViewIsOwner, ViewIsUsedProof,
    ViewStorageBalanceBounds, ViewStorageBalanceOf,
};
use crate::types::{MigrationInputData, PausedMask, Proof};
use aurora_engine_types::account_id::AccountId;
use aurora_engine_types::types::Address;
use aurora_workspace_utils::{Contract, ContractId};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::json_types::U128;
use near_sdk::Balance;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct EthConnectorContract {
    contract: Contract,
}

impl EthConnectorContract {
    pub fn new(contract: Contract) -> Self {
        Self { contract }
    }
}

impl ContractId for EthConnectorContract {
    fn as_contract(&self) -> &Contract {
        &self.contract
    }

    fn id(&self) -> &workspaces::AccountId {
        self.contract.id()
    }
}

/// Call functions
impl EthConnectorContract {
    pub fn init<A: AsRef<str>>(
        &self,
        prover_account: A,
        eth_custodian_address: String,
        metadata: FungibleTokenMetadata,
        account_with_access_right: &A,
        owner_id: A,
    ) -> CallNew {
        CallNew::call(&self.contract).args_json(json!({
            "prover_account": prover_account.as_ref(),
            "account_with_access_right": account_with_access_right.as_ref(),
            "owner_id": owner_id.as_ref(),
            "eth_custodian_address": eth_custodian_address,
            "metadata": metadata,
        }))
    }

    pub fn ft_transfer(
        &self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    ) -> CallFtTransfer {
        CallFtTransfer::call(&self.contract)
            .args_json(json!({ "receiver_id": receiver_id, "amount": amount, "memo": memo }))
    }

    pub fn ft_transfer_call(
        &self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> CallFtTransferCall {
        CallFtTransferCall::call(&self.contract).args_json(json!({
           "receiver_id": receiver_id,
           "amount": amount,
           "memo": memo,
           "msg": msg,
        }))
    }

    pub fn engine_ft_transfer(
        &self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    ) -> CallEngineFtTransfer {
        CallEngineFtTransfer::call(&self.contract).args_json(json!({
            "sender_id": sender_id,
            "receiver_id": receiver_id,
            "amount": amount,
            "memo": memo
        }))
    }

    pub fn engine_ft_transfer_call(
        &self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> CallEngineFtTransferCall {
        CallEngineFtTransferCall::call(&self.contract).args_json(json!({
            "sender_id": sender_id,
            "receiver_id": receiver_id,
            "amount": amount,
            "memo": memo,
            "msg": msg,
        }))
    }

    pub fn set_engine_account(&self, engine_account: &AccountId) -> CallSetEngineAccount {
        CallSetEngineAccount::call(&self.contract).args_json(json!({
            "engine_account": engine_account,
        }))
    }

    pub fn remove_engine_account(&self, engine_account: &AccountId) -> CallRemoveEngineAccount {
        CallRemoveEngineAccount::call(&self.contract).args_json(json!({
            "engine_account": engine_account,
        }))
    }

    pub fn storage_deposit(
        &self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> CallStorageDeposit {
        CallStorageDeposit::call(&self.contract)
            .args_json(json!({ "account_id": account_id, "registration_only": registration_only}))
    }

    pub fn storage_withdraw(&self, amount: Option<U128>) -> CallStorageWithdraw {
        CallStorageWithdraw::call(&self.contract).args_json(json!({ "amount": amount }))
    }

    pub fn storage_unregister(&self, force: Option<bool>) -> CallStorageUnregister {
        CallStorageUnregister::call(&self.contract).args_json(serde_json::json!({ "force": force }))
    }

    pub fn engine_storage_deposit(
        &self,
        sender_id: AccountId,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> CallEngineStorageDeposit {
        CallEngineStorageDeposit::call(&self.contract)
            .args_json(json!({ "sender_id":  sender_id, "account_id": account_id, "registration_only": registration_only}))
    }

    pub fn engine_storage_withdraw(
        &self,
        sender_id: AccountId,
        amount: Option<U128>,
    ) -> CallEngineStorageWithdraw {
        CallEngineStorageWithdraw::call(&self.contract)
            .args_json(json!({ "sender_id":  sender_id, "amount": amount }))
    }

    pub fn engine_storage_unregister(
        &self,
        sender_id: AccountId,
        force: Option<bool>,
    ) -> CallEngineStorageUnregister {
        CallEngineStorageUnregister::call(&self.contract)
            .args_json(json!({ "sender_id":  sender_id, "force": force }))
    }

    pub fn set_paused_flags(&self, paused: PausedMask) -> CallSetPausedFlags {
        CallSetPausedFlags::call(&self.contract).args_borsh(paused)
    }

    pub fn set_access_right(&self, account: AccountId) -> CallSetAccessRight {
        CallSetAccessRight::call(&self.contract).args_json((account,))
    }

    pub fn withdraw(&self, recipient_address: Address, amount: Balance) -> CallWithdraw {
        CallWithdraw::call(&self.contract).args_borsh((recipient_address, amount))
    }

    pub fn engine_withdraw(
        &self,
        sender_id: AccountId,
        recipient_address: Address,
        amount: Balance,
    ) -> CallEngineWithdraw {
        CallEngineWithdraw::call(&self.contract).args_borsh((sender_id, recipient_address, amount))
    }

    pub fn deposit(&self, raw_proof: Proof) -> CallDeposit {
        CallDeposit::call(&self.contract).args_borsh(raw_proof)
    }

    pub fn migrate(&self, data: MigrationInputData) -> CallMigrate {
        CallMigrate::call(&self.contract).args_borsh(data)
    }
}

/// View functions
impl EthConnectorContract {
    pub fn get_bridge_prover(&self) -> ViewGetBridgeProver {
        ViewGetBridgeProver::view(&self.contract)
    }

    pub fn check_migration_correctness(
        &self,
        data: MigrationInputData,
    ) -> ViewCheckMigrationCorrectness {
        ViewCheckMigrationCorrectness::view(&self.contract).args_borsh(data)
    }

    pub fn ft_metadata(&self) -> ViewFtMetadata {
        ViewFtMetadata::view(&self.contract)
    }

    pub fn get_paused_flags(&self) -> ViewGetPausedFlags {
        ViewGetPausedFlags::view(&self.contract)
    }

    pub fn get_account_with_access_right(&self) -> ViewGetAccountWithAccessRight {
        ViewGetAccountWithAccessRight::view(&self.contract)
    }

    pub fn is_owner(&self) -> ViewIsOwner {
        ViewIsOwner::view(&self.contract)
    }

    pub fn is_used_proof(&self, proof: Proof) -> ViewIsUsedProof {
        ViewIsUsedProof::view(&self.contract).args_borsh(proof)
    }

    pub fn storage_balance_of(&self, account_id: AccountId) -> ViewStorageBalanceOf {
        ViewStorageBalanceOf::view(&self.contract).args_json(json!({ "account_id": account_id }))
    }

    pub fn storage_balance_bounds(&self) -> ViewStorageBalanceBounds {
        ViewStorageBalanceBounds::view(&self.contract)
    }

    pub fn is_engine_account_exist(&self, engine_account: &AccountId) -> ViewIsEngineAccountExist {
        ViewIsEngineAccountExist::view(&self.contract).args_json(json!({
            "engine_account": engine_account,
        }))
    }

    pub fn ft_total_supply(&self) -> ViewFtTotalSupply {
        ViewFtTotalSupply::view(&self.contract)
    }

    pub fn ft_balance_of(&self, account_id: AccountId) -> ViewFtBalanceOf {
        ViewFtBalanceOf::view(&self.contract).args_json(json!((account_id,)))
    }
}
