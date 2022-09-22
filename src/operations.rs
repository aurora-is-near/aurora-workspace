use crate::impls::AuroraReturns;
use crate::Result;
use aurora_engine::parameters::{SubmitResult, WithdrawResult};
use aurora_engine_sdk::promise::PromiseId;
use ethereum_types::Address;
use workspaces::operations::CallTransaction;
use workspaces::result::ExecutionFinalResult;

macro_rules! impl_call_return  {
    ($(($name:ident, $return:ty, $fun:ident)),*) => {
        $(pub struct $name<'a, 'b>(pub(crate) EvmCallTransaction<'a, 'b>);

        impl<'a, 'b> $name<'a, 'b> {
            pub fn gas(mut self, gas: u64) -> $name<'a, 'b> {
                self.0 = self.0.gas(gas);
                self
            }

            pub fn max_gas(mut self) -> $name<'a, 'b> {
                self.0 = self.0.max_gas();
                self
            }

            pub async fn transact(self) -> Result<$return> {
                self.0.transact().await?.$fun()
            }
        })*
    }
}

impl_call_return![
    (CallDeployCode, SubmitResult, try_to_evm_result),
    (CallDeployErc20Token, Address, try_to_address),
    (CallEvm, SubmitResult, try_to_evm_result),
    (CallSubmit, SubmitResult, try_to_evm_result),
    (CallRegisterRelayer, (), try_to_empty),
    (CallFtOnTransfer, (), try_to_empty),
    (CallWithdraw, WithdrawResult, try_to_withdraw_result),
    (CallDeposit, PromiseId, try_to_promise_id),
    (CallFtTransfer, (), try_to_empty),
    (CallFtTransferCall, PromiseId, try_to_promise_id),
    (CallStorageDeposit, (), try_to_empty),
    (CallStorageUnregister, (), try_to_empty),
    (CallStorageWithdraw, (), try_to_empty)
];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Call {
    DeployCode,
    DeployErc20Token,
    EvmCall,
    Submit,
    RegisterRelayer,
    FtOnTransfer,
    Withdraw,
    Deposit,
    FtTransfer,
    FtTransferCall,
    StorageDeposit,
    StorageUnregister,
    StorageWithdraw,
}

impl AsRef<str> for Call {
    fn as_ref(&self) -> &str {
        use Call::*;
        match self {
            DeployCode => "deploy_code",
            DeployErc20Token => "deploy_erc20_token",
            EvmCall => "call",
            Submit => "submit",
            RegisterRelayer => "register_relayer",
            FtOnTransfer => "ft_on_transfer",
            Withdraw => "withdraw",
            Deposit => "deposit",
            FtTransfer => "ft_transfer",
            FtTransferCall => "ft_transfer_call",
            StorageDeposit => "storage_deposit",
            StorageUnregister => "storage_unregister",
            StorageWithdraw => "storage_withdraw",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum View {
    Version,
    Owner,
    BridgeProver,
    ChainId,
    UpgradeIndex,
    PausedPrecompiles,
    BlockHash,
    Code,
    Balance,
    Nonce,
    StorageAt,
    EvmView,
    IsUsedProof,
    FtTotalSupply,
    FtBalanceOf,
    FtBalanceOfEth,
    FtTotalEthSupplyOnNear,
    FtTotalEthSupplyOnAurora,
    FtMetadata,
    StorageBalanceOf,
    PausedFlags,
    AccountsCounter,
    Erc20FromNep141,
}

impl AsRef<str> for View {
    fn as_ref(&self) -> &str {
        use View::*;
        match self {
            Version => "get_version",
            Owner => "get_owner",
            BridgeProver => "get_bridge_prover",
            ChainId => "get_chain_id",
            UpgradeIndex => "get_upgrade_index",
            PausedPrecompiles => "get_paused_precompiles",
            BlockHash => "get_block_hash",
            Code => "get_code",
            Balance => "get_balance",
            Nonce => "get_nonce",
            StorageAt => "get_storage_at",
            EvmView => "get_view",
            IsUsedProof => "is_used_proof",
            FtTotalSupply => "ft_total_supply",
            FtBalanceOf => "ft_balance_of",
            FtBalanceOfEth => "ft_balance_of_eth",
            FtTotalEthSupplyOnNear => "ft_total_eth_supply_on_near",
            FtTotalEthSupplyOnAurora => "ft_total_eth_supply_on_aurora",
            FtMetadata => "ft_metadata",
            StorageBalanceOf => "storage_balance_of",
            PausedFlags => "get_paused_flags",
            AccountsCounter => "get_accounts_counter",
            Erc20FromNep141 => "get_erc20_from_nep141",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AuthorizedCall {
    PausePrecompiles,
}

impl AsRef<str> for AuthorizedCall {
    fn as_ref(&self) -> &str {
        use AuthorizedCall::*;
        match self {
            PausePrecompiles => "pause_precompiles",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OwnerCall {
    New,
    StageUpgrade,
    DeployUpgrade,
    StateMigration,
    ResumePrecompiles,
    FactoryUpdate,
    FactorySetWNEARAddress,
}

impl AsRef<str> for OwnerCall {
    fn as_ref(&self) -> &str {
        use OwnerCall::*;
        match self {
            New => "new",
            StageUpgrade => "stage_upgrade",
            DeployUpgrade => "deploy_upgrade",
            StateMigration => "state_migration",
            ResumePrecompiles => "resume_precompiles",
            FactoryUpdate => "factory_update",
            FactorySetWNEARAddress => "factory_set_wnear_address",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SelfCall {
    NewEthConnector,
    SetEthConnectorContractData,
    FactoryUpdateAddressVersion,
    RefundOnError,
    FinishDeposit,
    FtResolveTransfer,
    SetPausedFlags,
}

impl AsRef<str> for SelfCall {
    fn as_ref(&self) -> &str {
        use SelfCall::*;
        match self {
            NewEthConnector => "new_eth_connector",
            SetEthConnectorContractData => "set_eth_connector_contract_data",
            FactoryUpdateAddressVersion => "factory_update_address_version",
            RefundOnError => "refund_on_error",
            FinishDeposit => "finish_deposit",
            FtResolveTransfer => "resolve_transfer",
            SetPausedFlags => "set_paused_flags",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TestCallFunction {
    MintAccount,
    VerifyLogEntry,
}

impl AsRef<str> for TestCallFunction {
    fn as_ref(&self) -> &str {
        use TestCallFunction::*;
        match self {
            MintAccount => "mint_account",
            VerifyLogEntry => "verify_log_entry",
        }
    }
}

pub struct EvmCallTransaction<'a, 'b> {
    inner: CallTransaction<'a, 'b>,
}

impl<'a, 'b> EvmCallTransaction<'a, 'b> {
    pub(crate) fn call(transaction: CallTransaction<'a, 'b>) -> Self {
        EvmCallTransaction { inner: transaction }
    }

    pub(crate) fn args(mut self, args: Vec<u8>) -> EvmCallTransaction<'a, 'b> {
        self.inner = self.inner.args(args);
        self
    }

    pub(crate) fn args_json<S: serde::Serialize>(mut self, args: S) -> EvmCallTransaction<'a, 'b> {
        self.inner = self.inner.args_json(args);
        self
    }

    pub(crate) fn args_borsh<B: borsh::BorshSerialize>(
        mut self,
        args: B,
    ) -> EvmCallTransaction<'a, 'b> {
        self.inner = self.inner.args_borsh(args);
        self
    }

    pub fn gas(mut self, gas: u64) -> EvmCallTransaction<'a, 'b> {
        self.inner = self.inner.gas(gas);
        self
    }

    pub fn max_gas(mut self) -> EvmCallTransaction<'a, 'b> {
        self.inner = self.inner.max_gas();
        self
    }

    pub async fn transact(self) -> Result<ExecutionFinalResult> {
        // TODO: return EVM execution result.
        Ok(self.inner.transact().await?)
    }
}
