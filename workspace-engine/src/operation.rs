use aurora_engine::parameters::WithdrawResult;
use aurora_workspace_utils::results::{ExecutionResult, ViewResult};
use aurora_workspace_utils::transactions::{CallTransaction, ViewTransaction};
use aurora_workspace_utils::{impl_call_return, impl_view_return, Contract};
#[cfg(feature = "ethabi")]
use ethabi::{ParamType, Token};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_contract_standards::storage_management::StorageBalance;
use near_sdk::json_types::U128;
use near_sdk::PromiseOrValue;

// Eth-connector
impl_call_return![
    (CallFtTransfer, Call::FtTransfer),
    (CallDeposit, Call::Deposit),
    (
        CallSetEthConnectorContractData,
        Call::SetEthConnectorContractData
    ),
    (
        CallFactoryUpdateAddressVersion,
        Call::FactoryUpdateAddressVersion
    ),
];

// Eth-connector
impl_call_return![
    (CallFtTransferCall => PromiseOrValue<U128>, Call::FtTransferCall, try_from),
    (CallStorageDeposit => StorageBalance, Call::StorageDeposit, json),
    (CallStorageUnregister => bool, Call::StorageUnregister, json),
    (CallStorageWithdraw => StorageBalance, Call::StorageWithdraw, json),
    (CallWithdraw => WithdrawResult, Call::Withdraw, borsh),
];

impl_view_return![
    (ViewFtTotalSupply => U128, View::FtTotalSupply, json),
    (ViewFtBalanceOf => U128, View::FtBalanceOf, json),
    (ViewStorageBalanceOf => StorageBalance, View::StorageBalanceOf, json),
    (ViewFtMetadata => FungibleTokenMetadata, View::FtMetadata, json),
];

/*
    (
        CallSetEthConnectorContractData,
        ExecutionSuccess<()>,
        try_from_borsh
    ),
        (
        CallFactoryUpdateAddressVersion,
        ExecutionSuccess<()>,
        try_from
    ),
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
    (CallFtOnTransfer, ExecutionSuccess<U128>, try_from_json),
    (CallRefundOnError, ExecutionSuccess<()>, try_from),
    (CallFactoryUpdate, ExecutionSuccess<()>, try_from),
    (CallFactorySetWNearAddress, ExecutionSuccess<()>, try_from),
    (CallDeployUpgrade, ExecutionSuccess<()>, try_from),
    (CallResumePrecompiles, ExecutionSuccess<()>, try_from),
    (CallPausePrecompiles, ExecutionSuccess<()>, try_from),
    (CallStageUpgrade, ExecutionSuccess<()>, try_from),
    (CallStateMigration, ExecutionSuccess<()>, try_from),
];*/

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub(crate) enum Call {
    New,
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
    PausePrecompiles,
    StageUpgrade,
    DeployUpgrade,
    StateMigration,
    ResumePrecompiles,
    FactoryUpdate,
    FactorySetWNEARAddress,
    SetEthConnectorContractData,
    FactoryUpdateAddressVersion,
    RefundOnError,
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
            PausePrecompiles => "pause_precompiles",
            New => "new",
            StageUpgrade => "stage_upgrade",
            DeployUpgrade => "deploy_upgrade",
            StateMigration => "state_migration",
            ResumePrecompiles => "resume_precompiles",
            FactoryUpdate => "factory_update",
            FactorySetWNEARAddress => "factory_set_wnear_address",
            SetEthConnectorContractData => "set_eth_connector_contract_data",
            FactoryUpdateAddressVersion => "factory_update_address_version",
            RefundOnError => "refund_on_error",
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
    Storage,
    Evm,
    IsProofUsed,
    FtTotalSupply,
    FtBalanceOf,
    BalanceOfEth,
    EthTotalSupply,
    FtMetadata,
    StorageBalanceOf,
    PausedFlags,
    Erc20FromNep141,
    Nep141FromErc20,
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
            Erc20FromNep141 => "get_erc20_from_nep141",
            Nep141FromErc20 => "get_nep141_from_erc20",
        }
    }
}

/*
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViewResultDetails<T> {
    pub result: T,
    pub logs: Vec<String>,
}

impl TryFrom<workspaces::result::ViewResultDetails> for ViewResultDetails<String> {
    type Error = anyhow::Error;

    fn try_from(view: workspaces::result::ViewResultDetails) -> anyhow::Result<Self> {
        Ok(Self {
            result: String::from_utf8(view.result)?,
            logs: view.logs,
        })
    }
}

impl TryFrom<workspaces::result::ViewResultDetails> for ViewResultDetails<AccountId> {
    type Error = anyhow::Error;

    fn try_from(view: workspaces::result::ViewResultDetails) -> anyhow::Result<Self> {
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
    ) -> anyhow::Result<Self> {
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
    type Error = anyhow::Error;

    fn try_from(view: workspaces::result::ViewResultDetails) -> anyhow::Result<Self> {
        let result: TransactionStatus = TransactionStatus::try_from_slice(view.result.as_slice())?;
        Ok(Self {
            result,
            logs: view.logs,
        })
    }
}

impl TryFrom<workspaces::result::ViewResultDetails> for ViewResultDetails<bool> {
    type Error = anyhow::Error;

    fn try_from(view: workspaces::result::ViewResultDetails) -> anyhow::Result<Self> {
        let result: bool = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            result,
            logs: view.logs,
        })
    }
}

impl TryFrom<workspaces::result::ViewResultDetails> for ViewResultDetails<u128> {
    type Error = anyhow::Error;

    fn try_from(view: workspaces::result::ViewResultDetails) -> anyhow::Result<Self> {
        let result: u128 = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            result,
            logs: view.logs,
        })
    }
}

impl ViewResultDetails<U256> {
    pub(crate) fn try_from_json(
        view: workspaces::result::ViewResultDetails,
    ) -> anyhow::Result<Self> {
        let result: Wei = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            result: result.raw(),
            logs: view.logs,
        })
    }
}

#




pub struct EngineCallTransaction<'a> {
    inner: CallTransaction<'a>,
}

impl<'a, 'b> EngineCallTransaction<'a> {
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

    pub async fn transact(self) -> anyhow::Result<ExecutionFinalResult> {
        Ok(self.inner.transact().await?)
    }
}
*/
