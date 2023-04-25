use crate::types::{MigrationCheckResult, PausedMask, WithdrawResult};
use aurora_workspace_types::AccountId;
use aurora_workspace_utils::results::{ExecutionResult, ViewResult};
use aurora_workspace_utils::transactions::{CallTransaction, ViewTransaction};
use aurora_workspace_utils::{impl_call_return, impl_view_return, Contract};
use near_contract_standards::{
    fungible_token::metadata::FungibleTokenMetadata, storage_management::StorageBalance,
};
use near_sdk::{
    json_types::{U128, U64},
    PromiseOrValue,
};

impl_call_return![
    (CallNew, Call::New),
    (CallFtTransfer, Call::FtTransfer),
    (CallEngineFtTransfer, Call::EngineFtTransfer),
    (CallSetEngineAccount, Call::SetEngineAccount),
    (CallRemoveEngineAccount, Call::RemoveEngineAccount),
    (CallDeposit, Call::Deposit),
    (CallSetPausedFlags, Call::SetPausedFlags),
    (CallSetAccessRight, Call::SetAccessRight),
    (CallMigrate, Call::Migrate),
];

impl_call_return![
    (CallFtTransferCall => PromiseOrValue<U128>, Call::FtTransferCall, try_from),
    (CallEngineFtTransferCall => PromiseOrValue<U128>, Call::EngineFtTransferCall, try_from),
    (CallStorageDeposit => StorageBalance, Call::StorageDeposit, json),
    (CallStorageUnregister => bool, Call::StorageUnregister, json),
    (CallStorageWithdraw => StorageBalance, Call::StorageWithdraw, json),
    (CallEngineStorageDeposit => StorageBalance, Call::EngineStorageDeposit, json),
    (CallEngineStorageUnregister => bool, Call::EngineStorageUnregister, json),
    (CallEngineStorageWithdraw => StorageBalance, Call::EngineStorageWithdraw, json),
    (CallWithdraw => WithdrawResult, Call::Withdraw, borsh),
];

impl_view_return![
    (ViewFtTotalSupply => U128, View::FtTotalSupply, json),
    (ViewFtBalanceOf => U128, View::FtBalanceOf, json),
    (ViewGetEngineAccounts => Vec<AccountId>, View::GetEngineAccounts, json),
    (ViewStorageBalanceOf => StorageBalance, View::StorageBalanceOf, json),
    (ViewStorageBalanceBounds => StorageBalance, View::StorageBalanceBounds, json),
    (ViewCheckMigrationCorrectness => MigrationCheckResult, View::CheckMigrationCorrectness, borsh),
    (ViewFtMetadata => FungibleTokenMetadata, View::FtMetadata, json),
    (ViewGetAccountsCounter => U64, View::GetAccountsCounter, borsh),
    (ViewGetPausedFlags => PausedMask, View::GetPausedFlags, borsh),
    (ViewGetAccessRight => AccountId, View::GetAccessRight, json),
    (ViewIsOwner => bool, View::IsOwner, json),
    (ViewIsUsedProof => bool, View::IsUsedProof, borsh),
    (ViewGetBridgeProver => AccountId, View::GetBridgeProver, json),
];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Call {
    New,
    Withdraw,
    Deposit,
    FtTransfer,
    FtTransferCall,
    EngineFtTransfer,
    EngineFtTransferCall,
    SetEngineAccount,
    RemoveEngineAccount,
    StorageDeposit,
    StorageUnregister,
    StorageWithdraw,
    EngineStorageDeposit,
    EngineStorageUnregister,
    EngineStorageWithdraw,
    SetPausedFlags,
    SetAccessRight,
    Migrate,
}

impl AsRef<str> for Call {
    fn as_ref(&self) -> &str {
        use Call::*;
        match self {
            New => "new",
            Withdraw => "withdraw",
            Deposit => "deposit",
            FtTransfer => "ft_transfer",
            FtTransferCall => "ft_transfer_call",
            SetEngineAccount => "set_engine_account",
            RemoveEngineAccount => "remove_engine_account",
            EngineFtTransfer => "engine_ft_transfer",
            EngineFtTransferCall => "engine_ft_transfer_call",
            StorageDeposit => "storage_deposit",
            StorageUnregister => "storage_unregister",
            StorageWithdraw => "storage_withdraw",
            EngineStorageDeposit => "engine_storage_deposit",
            EngineStorageUnregister => "engine_storage_unregister",
            EngineStorageWithdraw => "engine_storage_withdraw",
            SetPausedFlags => "set_paused_flags",
            SetAccessRight => "set_access_right",
            Migrate => "migrate",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum View {
    FtTotalSupply,
    FtBalanceOf,
    FtMetadata,
    StorageBalanceOf,
    StorageBalanceBounds,
    AccountsCounter,
    GetEngineAccounts,
    GetAccountsCounter,
    GetPausedFlags,
    GetAccessRight,
    IsOwner,
    CheckMigrationCorrectness,
    IsUsedProof,
    GetBridgeProver,
}

impl AsRef<str> for View {
    fn as_ref(&self) -> &str {
        use View::*;
        match self {
            FtTotalSupply => "ft_total_supply",
            FtBalanceOf => "ft_balance_of",
            FtMetadata => "ft_metadata",
            StorageBalanceOf => "storage_balance_of",
            StorageBalanceBounds => "storage_balance_bounds",
            AccountsCounter => "get_accounts_counter",
            GetEngineAccounts => "get_engine_accounts",
            GetAccountsCounter => "get_accounts_counter",
            GetPausedFlags => "get_paused_flags",
            GetAccessRight => "get_access_right",
            IsOwner => "is_owner",
            CheckMigrationCorrectness => "check_migration_correctness",
            IsUsedProof => "is_used_proof",
            GetBridgeProver => "get_bridge_prover",
        }
    }
}
