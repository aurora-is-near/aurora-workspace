#![allow(unused_imports)]
use crate::operation::CallFtTransfer;
use crate::types::{MigrationCheckResult, MigrationInputData, PausedMask, Proof};
use aurora_engine_types::types::Address;
use aurora_workspace_types::AccountId;
use aurora_workspace_utils::Contract;
use borsh::BorshSerialize;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_contract_standards::storage_management::{StorageBalance, StorageBalanceBounds};
use near_sdk::json_types::{U128, U64};
use near_sdk::Balance;
use serde_json::json;
use workspaces::Account;

#[derive(Debug, Clone)]
pub struct EthConnectorContract {
    contract: Contract,
}

impl EthConnectorContract {
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
    ) -> CallFtTransfer {
        CallFtTransfer::call(&self.contract).args_json(json!({
           "receiver_id": receiver_id,
           "amount": amount,
           "memo": memo,
           "msg": msg,
        }))
    }
    /*
       pub async fn ft_total_supply(&self) -> anyhow::Result<ViewResultDetails<U128>> {
           ViewResultDetails::<U128>::try_from_json(
               self.near_view(&View::FtTotalSupply, vec![]).await?,
           )
       }

       pub async fn ft_balance_of(
           &self,
           account_id: AccountId,
       ) -> anyhow::Result<ViewResultDetails<U128>> {
           let args = json!((account_id,)).to_string().as_bytes().to_vec();
           ViewResultDetails::try_from_json(self.near_view(&View::FtBalanceOf, args).await?)
       }

       pub fn engine_ft_transfer(
           &self,
           sender_id: AccountId,
           receiver_id: AccountId,
           amount: U128,
           memo: Option<String>,
       ) -> CallFtTransfer<'_> {
           CallFtTransfer(self.near_call(&Call::EngineFtTransfer).args_json(json!({
               "sender_id": sender_id,
               "receiver_id": receiver_id,
               "amount": amount,
               "memo": memo
           })))
       }

       pub fn engine_ft_transfer_call(
           &self,
           sender_id: AccountId,
           receiver_id: AccountId,
           amount: U128,
           memo: Option<String>,
           msg: String,
       ) -> CallFtTransferCall<'_> {
           CallFtTransferCall(
               self.near_call(&Call::EngineFtTransferCall)
                   .args_json(json!({
                       "sender_id": sender_id,
                       "receiver_id": receiver_id,
                       "amount": amount,
                       "memo": memo,
                       "msg": msg,
                   })),
           )
       }

       pub fn set_engine_account(&self, engine_account: AccountId) -> CallSetEngineAccount<'_> {
           CallSetEngineAccount(self.near_call(&Call::SetEngineAccount).args_json(json!({
               "engine_account": engine_account,
           })))
       }

       pub fn remove_engine_account(&self, engine_account: AccountId) -> CallRemoveEngineAccount<'_> {
           CallRemoveEngineAccount(self.near_call(&Call::RemoveEngineAccount).args_json(json!({
               "engine_account": engine_account,
           })))
       }

       pub async fn get_engine_accounts(&self) -> anyhow::Result<ViewResultDetails<Vec<AccountId>>> {
           ViewResultDetails::<Vec<AccountId>>::try_from_json(
               self.near_view(&View::GetEngineAccounts, vec![]).await?,
           )
       }

       pub fn storage_deposit(
           &self,
           account_id: Option<AccountId>,
           registration_only: Option<bool>,
       ) -> CallStorageDeposit<'_> {
           let args = json!({ "account_id": account_id, "registration_only": registration_only});
           CallStorageDeposit(self.near_call(&Call::StorageDeposit).args_json(args))
       }

       pub fn storage_withdraw(&self, amount: Option<U128>) -> CallStorageWithdraw<'_> {
           let args = json!({ "amount": amount });
           CallStorageWithdraw(self.near_call(&Call::StorageWithdraw).args_json(args))
       }

       pub fn storage_unregister(&self, force: Option<bool>) -> CallStorageUnregister<'_> {
           let val = serde_json::json!({ "force": force });
           CallStorageUnregister(self.near_call(&Call::StorageUnregister).args_json(val))
       }

       pub fn engine_storage_deposit(
           &self,
           sender_id: AccountId,
           account_id: Option<AccountId>,
           registration_only: Option<bool>,
       ) -> CallStorageDeposit<'_> {
           let args = json!({ "sender_id":  sender_id, "account_id": account_id, "registration_only": registration_only});
           CallStorageDeposit(self.near_call(&Call::EngineStorageDeposit).args_json(args))
       }

       pub fn engine_storage_withdraw(
           &self,
           sender_id: AccountId,
           amount: Option<U128>,
       ) -> CallStorageWithdraw<'_> {
           let args = json!({ "sender_id":  sender_id, "amount": amount });
           CallStorageWithdraw(self.near_call(&Call::EngineStorageWithdraw).args_json(args))
       }

       pub fn engine_storage_unregister(
           &self,
           sender_id: AccountId,
           force: Option<bool>,
       ) -> CallStorageUnregister<'_> {
           let val = serde_json::json!({ "sender_id":  sender_id, "force": force });
           CallStorageUnregister(
               self.near_call(&Call::EngineStorageUnregister)
                   .args_json(val),
           )
       }

       pub async fn storage_balance_of(
           &self,
           account_id: AccountId,
       ) -> anyhow::Result<ViewResultDetails<Option<StorageBalance>>> {
           let args = json!({ "account_id": account_id })
               .to_string()
               .as_bytes()
               .to_vec();
           ViewResultDetails::<Option<StorageBalance>>::try_from_json(
               self.near_view(&View::StorageBalanceOf, args).await?,
           )
       }

       pub async fn storage_balance_bounds(
           &self,
       ) -> anyhow::Result<ViewResultDetails<StorageBalanceBounds>> {
           ViewResultDetails::<StorageBalanceBounds>::try_from_json(
               self.near_view(&View::StorageBalanceBounds, vec![]).await?,
           )
       }

       pub fn ft_resolve_transfer(
           &self,
           sender_id: AccountId,
           receiver_id: AccountId,
           amount: U128,
       ) -> CallFtResolveTransfer<'_> {
           CallFtResolveTransfer(self.near_call(&Call::FtResolveTransfer).args_json(json!({
               "sender_id": sender_id,
               "receiver_id": receiver_id,
               "amount": amount,
           })))
       }

       pub fn set_paused_flags(&self, paused: PausedMask) -> CallSetPausedFlags<'_> {
           CallSetPausedFlags(self.near_call(&Call::SetPausedFlags).args_borsh(paused))
       }

       pub fn set_access_right(&self, account: AccountId) -> CallSetAccessRight<'_> {
           CallSetAccessRight(self.near_call(&Call::SetAccessRight).args_json((account,)))
       }

       pub fn withdraw(
           &self,
           sender_id: AccountId,
           recipient_address: Address,
           amount: Balance,
       ) -> CallWithdraw<'_> {
           CallWithdraw(self.near_call(&Call::Withdraw).args_borsh((
               sender_id,
               recipient_address,
               amount,
           )))
       }

       pub fn deposit(&self, raw_proof: Proof) -> CallDeposit<'_> {
           CallDeposit(self.near_call(&Call::Deposit).args_borsh(raw_proof))
       }

       pub fn migrate(&self, data: MigrationInputData) -> CallMigrate<'_> {
           CallMigrate(self.near_call(&Call::Migrate).args_borsh(data))
       }

       pub async fn ft_metadata(&self) -> anyhow::Result<ViewResultDetails<FungibleTokenMetadata>> {
           ViewResultDetails::try_from_json(self.near_view(&View::FtMetadata, vec![]).await?)
       }

       pub async fn get_accounts_counter(&self) -> anyhow::Result<ViewResultDetails<U64>> {
           ViewResultDetails::try_from_borsh(self.near_view(&View::GetAccountsCounter, vec![]).await?)
       }

       pub async fn get_paused_flags(&self) -> anyhow::Result<ViewResultDetails<PausedMask>> {
           ViewResultDetails::try_from_borsh(self.near_view(&View::GetPausedFlags, vec![]).await?)
       }

       pub async fn get_access_right(&self) -> anyhow::Result<ViewResultDetails<AccountId>> {
           ViewResultDetails::try_from_json(self.near_view(&View::GetAccessRight, vec![]).await?)
       }

       pub async fn is_owner(&self) -> anyhow::Result<ViewResultDetails<bool>> {
           ViewResultDetails::try_from_json(self.near_view(&View::IsOwner, vec![]).await?)
       }

       pub async fn check_migration_correctness(
           &self,
           data: MigrationInputData,
       ) -> anyhow::Result<ViewResultDetails<MigrationCheckResult>> {
           let args = data.try_to_vec().unwrap();
           ViewResultDetails::<MigrationCheckResult>::try_from_borsh(
               self.near_view(&View::CheckMigrationCorrectness, args)
                   .await?,
           )
       }

       pub async fn is_used_proof(&self, proof: Proof) -> anyhow::Result<ViewResultDetails<bool>> {
           ViewResultDetails::<bool>::try_from_borsh(
               self.near_view(&View::IsUsedProof, proof.try_to_vec()?)
                   .await?,
           )
       }

       pub async fn get_bridge_prover(&self) -> anyhow::Result<ViewResultDetails<AccountId>> {
           ViewResultDetails::try_from_json(self.near_view(&View::GetBridgeProver, vec![]).await?)
       }
    */
}
