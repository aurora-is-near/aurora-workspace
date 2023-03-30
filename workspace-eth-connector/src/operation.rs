#![allow(dead_code)]
use crate::result::ExecutionSuccess;
use crate::types::MigrationCheckResult;
use crate::Result;
use aurora_workspace_types::AccountId;
use borsh::BorshDeserialize;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_contract_standards::storage_management::{StorageBalance, StorageBalanceBounds};
use near_sdk::json_types::U128;
use near_sdk::PromiseOrValue;
use workspaces::operations::CallTransaction;
use workspaces::result::ExecutionFinalResult;

macro_rules! impl_call_return  {
    ($(($name:ident, $return:ty, $deser_fn:ident)),* $(,)?) => {
        $(pub struct $name<'a>(pub(crate) EthConnectorCallTransaction<'a>);

        impl<'a> $name<'a> {
            pub fn gas(mut self, gas: u64) -> Self {
                self.0 = self.0.gas(gas);
                self
            }

            pub fn max_gas(mut self) -> Self {
                self.0 = self.0.max_gas();
                self
            }

            pub fn deposit(mut self, deposit: u128) -> Self {
                self.0 = self.0.deposit(deposit);
                self
            }

            pub async fn transact(self) -> Result<$return> {
                ExecutionSuccess::$deser_fn(self.0.transact().await?)
            }
        })*
    }
}

impl_call_return![
    (CallFtTransfer, ExecutionSuccess<()>, try_from),
    (
        CallFtTransferCall,
        ExecutionSuccess<PromiseOrValue<U128>>,
        try_from
    ),
    (CallSetEngineAccount, ExecutionSuccess<()>, try_from),
    (CallRemoveEngineAccount, ExecutionSuccess<()>, try_from),
    (
        CallStorageDeposit,
        ExecutionSuccess<StorageBalance>,
        try_from_json
    ),
    (CallStorageUnregister, ExecutionSuccess<bool>, try_from_json),
    (
        CallStorageWithdraw,
        ExecutionSuccess<StorageBalance>,
        try_from_json
    ),
    (
        CallWithdraw,
        ExecutionSuccess<StorageBalance>,
        try_from_borsh
    ),
    (CallFtResolveTransfer, ExecutionSuccess<U128>, try_from_json),
    (CallSetPausedFlags, ExecutionSuccess<()>, try_from),
    (CallSetAccessRight, ExecutionSuccess<()>, try_from),
];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Call {
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
    FtResolveTransfer,
    SetPausedFlags,
    SetAccessRight,
    FinishDeposit,
    Migrate,
}

impl AsRef<str> for Call {
    fn as_ref(&self) -> &str {
        use Call::*;
        match self {
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
            FtResolveTransfer => "ftz_resolve_transfer",
            SetPausedFlags => "set_paused_flags",
            SetAccessRight => "set_access_right",
            FinishDeposit => "finish_deposit",
            Migrate => "migrate",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViewResultDetails<T> {
    pub result: T,
    pub logs: Vec<String>,
}

impl ViewResultDetails<StorageBalanceBounds> {
    pub(crate) fn try_from_json(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        let result: StorageBalanceBounds = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            result,
            logs: view.logs,
        })
    }
}

impl ViewResultDetails<Option<StorageBalance>> {
    pub(crate) fn try_from_json(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        let result: Option<StorageBalance> = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            result,
            logs: view.logs,
        })
    }
}

impl ViewResultDetails<Vec<AccountId>> {
    pub(crate) fn try_from_json(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        let result: Vec<AccountId> = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            result,
            logs: view.logs,
        })
    }
}

impl ViewResultDetails<FungibleTokenMetadata> {
    pub(crate) fn try_from_json(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        let result: FungibleTokenMetadata = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            result,
            logs: view.logs,
        })
    }
}

impl ViewResultDetails<U128> {
    pub(crate) fn try_from_json(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        let result: U128 = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            result,
            logs: view.logs,
        })
    }
}

impl ViewResultDetails<MigrationCheckResult> {
    pub(crate) fn try_from_borsh(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        Ok(Self {
            result: MigrationCheckResult::try_from_slice(view.result.as_slice())?,
            logs: view.logs,
        })
    }
}

impl ViewResultDetails<AccountId> {
    pub(crate) fn try_from_borsh(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        Ok(Self {
            result: AccountId::try_from_slice(view.result.as_slice())?,
            logs: view.logs,
        })
    }
}

impl ViewResultDetails<bool> {
    pub(crate) fn try_from_borsh(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        Ok(Self {
            result: bool::try_from_slice(view.result.as_slice())?,
            logs: view.logs,
        })
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

pub struct EthConnectorCallTransaction<'a> {
    inner: CallTransaction<'a>,
}

impl<'a, 'b> EthConnectorCallTransaction<'a> {
    pub(crate) fn call(transaction: CallTransaction<'a>) -> Self {
        Self { inner: transaction }
    }

    pub(crate) fn args(mut self, args: Vec<u8>) -> Self {
        self.inner = self.inner.args(args);
        self
    }

    pub(crate) fn args_json<S: serde::Serialize>(mut self, args: S) -> Self {
        self.inner = self.inner.args_json(args);
        self
    }

    pub(crate) fn args_borsh<B: borsh::BorshSerialize>(mut self, args: B) -> Self {
        self.inner = self.inner.args_borsh(args);
        self
    }

    pub fn gas(mut self, gas: u64) -> Self {
        self.inner = self.inner.gas(gas);
        self
    }

    pub fn max_gas(mut self) -> Self {
        self.inner = self.inner.max_gas();
        self
    }

    pub fn deposit(mut self, deposit: u128) -> Self {
        self.inner = self.inner.deposit(deposit);
        self
    }

    pub async fn transact(self) -> Result<ExecutionFinalResult> {
        Ok(self.inner.transact().await?)
    }
}
