use crate::types::{MigrationCheckResult, PausedMask, WithdrawResult};
use aurora_engine_types::account_id::AccountId;
use aurora_workspace_utils::results::{ExecutionResult, ViewResult};
use aurora_workspace_utils::transactions::{CallTransaction, ViewTransaction};
use aurora_workspace_utils::{impl_call_return, impl_view_return, Contract};
use near_contract_standards::storage_management::StorageBalanceBounds;
use near_contract_standards::{
    fungible_token::metadata::FungibleTokenMetadata, storage_management::StorageBalance,
};
use near_sdk::{json_types::U128, PromiseOrValue};
use near_workspaces::types::{Gas, NearToken};

impl_call_return![
    (CallNew, Call::New),
    (CallFtTransfer, Call::FtTransfer),
    (CallEngineFtTransfer, Call::EngineFtTransfer),
    (CallSetEngineAccount, Call::SetEngineAccount),
    (CallRemoveEngineAccount, Call::RemoveEngineAccount),
    (CallDeposit, Call::Deposit),
    (CallPaPauseFeature, Call::PaPauseFeature),
    (CallPaUnpauseFeature, Call::PaUnpauseFeature),
    (CallAclRevokeRole, Call::AclRevokeRole),
    (CallAclGrantRole, Call::AclGrantRole),
    (CallMigrate, Call::Migrate),
    (CallSetAuroraEngineAccountId, Call::SetAuroraEngineAccountId)
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
    (CallEngineWithdraw => WithdrawResult, Call::EngineWithdraw, borsh),
];

impl_view_return![
    (ViewFtTotalSupply => U128, View::FtTotalSupply, json),
    (ViewFtBalanceOf => U128, View::FtBalanceOf, json),
    (ViewIsEngineAccountExist => bool, View::IsEngineAccountExist, json),
    (ViewStorageBalanceOf => StorageBalance, View::StorageBalanceOf, json),
    (ViewStorageBalanceBounds => StorageBalanceBounds, View::StorageBalanceBounds, json),
    (ViewCheckMigrationCorrectness => MigrationCheckResult, View::CheckMigrationCorrectness, borsh),
    (ViewFtMetadata => FungibleTokenMetadata, View::FtMetadata, json),
    (ViewGetPausedFlags => PausedMask, View::GetPausedFlags, borsh),
    (ViewAclGetGrantees => Vec<AccountId>, View::AclGetGrantees, json),
    (ViewIsOwner => bool, View::IsOwner, json),
    (ViewIsUsedProof => bool, View::IsUsedProof, borsh),
    (ViewGetBridgeProver => AccountId, View::GetBridgeProver, json),
    (ViewGetAuroraEngineAccountId => AccountId, View::GetAuroraEngineAccountId, json)
];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Call {
    New,
    Withdraw,
    EngineWithdraw,
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
    PaPauseFeature,
    PaUnpauseFeature,
    AclRevokeRole,
    AclGrantRole,
    Migrate,
    SetAuroraEngineAccountId,
}

impl AsRef<str> for Call {
    fn as_ref(&self) -> &str {
        use Call::*;
        match self {
            New => "new",
            Withdraw => "withdraw",
            EngineWithdraw => "engine_withdraw",
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
            PaPauseFeature => "pa_pause_feature",
            PaUnpauseFeature => "pa_unpause_feature",
            AclGrantRole => "acl_grant_role",
            AclRevokeRole => "acl_revoke_role",
            Migrate => "migrate",
            SetAuroraEngineAccountId => "set_aurora_engine_account_id",
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
    IsEngineAccountExist,
    GetPausedFlags,
    AclGetGrantees,
    IsOwner,
    CheckMigrationCorrectness,
    IsUsedProof,
    GetBridgeProver,
    GetAuroraEngineAccountId,
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
            IsEngineAccountExist => "is_engine_account_exist",
            GetPausedFlags => "get_paused_flags",
            AclGetGrantees => "acl_get_grantees",
            IsOwner => "is_owner",
            CheckMigrationCorrectness => "check_migration_correctness",
            IsUsedProof => "is_used_proof",
            GetBridgeProver => "get_bridge_prover",
            GetAuroraEngineAccountId => "get_aurora_engine_account_id",
        }
    }
}
