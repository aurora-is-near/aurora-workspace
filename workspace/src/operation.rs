#![allow(dead_code)]
use crate::error::Error;
use crate::result::ExecutionSuccess;
use crate::types::output::SubmitResult;
use crate::Result;
use aurora_engine::fungible_token::FungibleTokenMetadata;
#[cfg(feature = "deposit-withdraw")]
use aurora_engine::parameters::WithdrawResult;
use aurora_engine::parameters::{StorageBalance, TransactionStatus};
use aurora_engine_sdk::promise::PromiseId;
use aurora_engine_types::types::Wei;
use aurora_workspace_types::AccountId;
use borsh::BorshDeserialize;
#[cfg(feature = "ethabi")]
use ethabi::{ParamType, Token};
use ethereum_types::{Address, H256, U256};
use workspaces::operations::CallTransaction;
use workspaces::result::ExecutionFinalResult;

macro_rules! impl_call_return  {
    ($(($name:ident, $return:ty, $fun:ident)),*) => {
        $(pub struct $name<'a>(pub(crate) EvmCallTransaction<'a>);

        impl<'a> $name<'a> {
            pub fn gas(mut self, gas: u64) -> $name<'a> {
                self.0 = self.0.gas(gas);
                self
            }

            pub fn max_gas(mut self) -> $name<'a> {
                self.0 = self.0.max_gas();
                self
            }

            pub async fn transact(self) -> Result<$return> {
                ExecutionSuccess::$fun(self.0.transact().await?)
            }
        })*
    }
}

impl_call_return![
    (
        CallDeployCode,
        ExecutionSuccess<SubmitResult>,
        try_from_borsh
    ),
    (CallDeployErc20, ExecutionSuccess<Address>, try_from),
    (CallEvm, ExecutionSuccess<SubmitResult>, try_from_borsh),
    (CallSubmit, ExecutionSuccess<SubmitResult>, try_from_borsh),
    (CallRegisterRelayer, ExecutionSuccess<()>, try_from),
    (CallFtOnTransfer, ExecutionSuccess<String>, try_from_json),
    (CallFtTransfer, ExecutionSuccess<()>, try_from),
    (CallFtTransferCall, ExecutionSuccess<PromiseId>, try_from),
    (CallStorageDeposit, ExecutionSuccess<()>, try_from),
    (CallStorageUnregister, ExecutionSuccess<()>, try_from),
    (CallStorageWithdraw, ExecutionSuccess<()>, try_from)
];

#[cfg(feature = "deposit-withdraw")]
impl_call_return![
    (CallDeposit, ExecutionSuccess<PromiseId>, try_from),
    (
        CallWithdraw,
        ExecutionSuccess<WithdrawResult>,
        try_from_borsh
    )
];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Call {
    DeployCode,
    DeployErc20Token,
    Evm,
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
            Evm => "call",
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
pub struct ViewResultDetails<T> {
    pub result: T,
    pub logs: Vec<String>,
}

impl TryFrom<workspaces::result::ViewResultDetails> for ViewResultDetails<String> {
    type Error = Error;

    fn try_from(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        Ok(Self {
            result: String::from_utf8(view.result)?,
            logs: view.logs,
        })
    }
}

impl TryFrom<workspaces::result::ViewResultDetails> for ViewResultDetails<AccountId> {
    type Error = Error;

    fn try_from(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        Ok(Self {
            result: AccountId::try_from_slice(view.result.as_slice())?,
            logs: view.logs,
        })
    }
}

impl From<workspaces::result::ViewResultDetails> for ViewResultDetails<U256> {
    fn from(view: workspaces::result::ViewResultDetails) -> Self {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(view.result.as_slice());
        Self {
            result: U256::from(buf),
            logs: view.logs,
        }
    }
}

impl From<workspaces::result::ViewResultDetails> for ViewResultDetails<u64> {
    fn from(view: workspaces::result::ViewResultDetails) -> Self {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(view.result.as_slice());
        Self {
            result: u64::from_le_bytes(buf),
            logs: view.logs,
        }
    }
}

// TODO return this as bitflags.
impl From<workspaces::result::ViewResultDetails> for ViewResultDetails<u32> {
    fn from(view: workspaces::result::ViewResultDetails) -> Self {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(view.result.as_slice());
        Self {
            result: u32::from_le_bytes(buf),
            logs: view.logs,
        }
    }
}

impl From<workspaces::result::ViewResultDetails> for ViewResultDetails<u8> {
    fn from(view: workspaces::result::ViewResultDetails) -> Self {
        let mut buf = [0u8; 1];
        buf.copy_from_slice(view.result.as_slice());
        Self {
            result: buf[0],
            logs: view.logs,
        }
    }
}

impl From<workspaces::result::ViewResultDetails> for ViewResultDetails<H256> {
    fn from(view: workspaces::result::ViewResultDetails) -> Self {
        Self {
            result: H256::from_slice(view.result.as_slice()),
            logs: view.logs,
        }
    }
}

impl From<workspaces::result::ViewResultDetails> for ViewResultDetails<Vec<u8>> {
    fn from(view: workspaces::result::ViewResultDetails) -> Self {
        Self {
            result: view.result,
            logs: view.logs,
        }
    }
}

#[cfg(feature = "ethabi")]
impl ViewResultDetails<Vec<Token>> {
    pub fn decode(
        types: &[ParamType],
        view: workspaces::result::ViewResultDetails,
    ) -> Result<Self> {
        Ok(Self {
            result: ethabi::decode(types, &view.result)?,
            logs: view.logs,
        })
    }
}

impl ViewResultDetails<u128> {
    pub(crate) fn from_u256(view: workspaces::result::ViewResultDetails) -> Self {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(view.result.as_slice());
        let value = U256::from(buf);
        Self {
            result: value.as_u128(),
            logs: vec![],
        }
    }
}

impl TryFrom<workspaces::result::ViewResultDetails> for ViewResultDetails<TransactionStatus> {
    type Error = Error;

    fn try_from(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        let result: TransactionStatus = TransactionStatus::try_from_slice(view.result.as_slice())?;
        Ok(Self {
            result,
            logs: view.logs,
        })
    }
}

impl TryFrom<workspaces::result::ViewResultDetails> for ViewResultDetails<bool> {
    type Error = Error;

    fn try_from(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        let result: bool = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            result,
            logs: view.logs,
        })
    }
}

impl TryFrom<workspaces::result::ViewResultDetails> for ViewResultDetails<u128> {
    type Error = Error;

    fn try_from(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        let result: u128 = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            result,
            logs: view.logs,
        })
    }
}

impl ViewResultDetails<U256> {
    pub(crate) fn try_from_json(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        let result: Wei = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            result: result.raw(),
            logs: view.logs,
        })
    }
}

impl TryFrom<workspaces::result::ViewResultDetails> for ViewResultDetails<FungibleTokenMetadata> {
    type Error = Error;

    fn try_from(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        let result: FungibleTokenMetadata =
            FungibleTokenMetadata::try_from_slice(view.result.as_slice())?;
        Ok(Self {
            result,
            logs: view.logs,
        })
    }
}

impl TryFrom<workspaces::result::ViewResultDetails> for ViewResultDetails<StorageBalance> {
    type Error = Error;

    fn try_from(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        Ok(Self {
            result: serde_json::from_slice(view.result.as_slice())?,
            logs: view.logs,
        })
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
    Storage,
    Evm,
    IsProofUsed,
    FtTotalSupply,
    FtBalanceOf,
    BalanceOfEth,
    EthTotalSupply,
    FtMetadata,
    StorageBalanceOf,
    PausedFlags,     // TODO
    AccountsCounter, // TODO
    Erc20FromNep141, // TODO
    Nep141FromErc20, // TODO
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
            Storage => "get_storage_at",
            Evm => "get_view",
            IsProofUsed => "is_used_proof",
            FtTotalSupply => "ft_total_supply",
            FtBalanceOf => "ft_balance_of",
            BalanceOfEth => "ft_balance_of_eth",
            EthTotalSupply => "ft_total_eth_supply_on_aurora",
            FtMetadata => "ft_metadata",
            StorageBalanceOf => "storage_balance_of",
            PausedFlags => "get_paused_flags",
            AccountsCounter => "get_accounts_counter",
            Erc20FromNep141 => "get_erc20_from_nep141",
            Nep141FromErc20 => "get_nep141_from_erc20",
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

pub struct EvmCallTransaction<'a> {
    inner: CallTransaction<'a>,
}

impl<'a, 'b> EvmCallTransaction<'a> {
    pub(crate) fn call(transaction: CallTransaction<'a>) -> Self {
        EvmCallTransaction { inner: transaction }
    }

    pub(crate) fn args(mut self, args: Vec<u8>) -> EvmCallTransaction<'a> {
        self.inner = self.inner.args(args);
        self
    }

    pub(crate) fn args_json<S: serde::Serialize>(mut self, args: S) -> EvmCallTransaction<'a> {
        self.inner = self.inner.args_json(args);
        self
    }

    pub(crate) fn args_borsh<B: borsh::BorshSerialize>(
        mut self,
        args: B,
    ) -> EvmCallTransaction<'a> {
        self.inner = self.inner.args_borsh(args);
        self
    }

    pub fn gas(mut self, gas: u64) -> EvmCallTransaction<'a> {
        self.inner = self.inner.gas(gas);
        self
    }

    pub fn max_gas(mut self) -> EvmCallTransaction<'a> {
        self.inner = self.inner.max_gas();
        self
    }

    pub async fn transact(self) -> Result<ExecutionFinalResult> {
        // TODO: return EVM execution result.
        Ok(self.inner.transact().await?)
    }
}
