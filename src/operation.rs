use crate::error::Error;
use crate::result::ExecutionSuccess;
use crate::Result;
use aurora_engine::parameters::{SubmitResult, TransactionStatus, WithdrawResult, StorageBalance};
use aurora_engine::fungible_token::FungibleTokenMetadata;
use aurora_engine_sdk::promise::PromiseId;
use aurora_engine_types::types::Wei;
use borsh::BorshDeserialize;
#[cfg(feature = "ethabi")]
use ethabi::ParamType;
#[cfg(feature = "ethabi")]
use ethabi::Token;
use ethereum_types::Address;
use ethereum_types::{H256, U256};
use near_account_id::AccountId;
use workspaces::operations::CallTransaction;
use workspaces::result::ExecutionFinalResult;
use workspaces::result::ViewResultDetails;

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
    (CallDeployErc20Token, ExecutionSuccess<Address>, try_from),
    (CallEvm, ExecutionSuccess<SubmitResult>, try_from_borsh),
    (CallSubmit, ExecutionSuccess<SubmitResult>, try_from_borsh),
    (CallRegisterRelayer, ExecutionSuccess<()>, try_from),
    (CallFtOnTransfer, ExecutionSuccess<()>, try_from),
    (
        CallWithdraw,
        ExecutionSuccess<WithdrawResult>,
        try_from_borsh
    ),
    (CallDeposit, ExecutionSuccess<PromiseId>, try_from),
    (CallFtTransfer, ExecutionSuccess<()>, try_from),
    (CallFtTransferCall, ExecutionSuccess<PromiseId>, try_from),
    (CallStorageDeposit, ExecutionSuccess<()>, try_from),
    (CallStorageUnregister, ExecutionSuccess<()>, try_from),
    (CallStorageWithdraw, ExecutionSuccess<()>, try_from)
];

macro_rules! impl_view_return {
    ($(($name:ident, $return:ty, $field:ident)),*) => {
        $(pub struct $name {
            pub $field: $return,
            pub logs: Vec<String>,
        })*
    }
}

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

impl_view_return![
    (ViewVersion, String, version),
    (ViewOwner, AccountId, account_id),
    (ViewBridgeProver, AccountId, account_id),
    (ViewChainId, U256, chain_id),
    (ViewUpgradeIndex, u64, upgrade_index),
    (ViewPausedPrecompiles, u32, paused_flags),
    (ViewBlockHash, H256, block_hash),
    (ViewBalance, U256, balance),
    (ViewNonce, U256, nonce),
    (ViewStorage, H256, storage),
    (ViewEvm, TransactionStatus, status),
    (ViewIsProofUsed, bool, is_proof_used),
    (ViewFtTotalSupply, u128, total_supply),
    (ViewFtBalanceOf, u128, balance),
    (ViewAddressBalance, U256, balance),
    (ViewEthTotalSupply, U256, total_supply),
    (ViewFtMetadata, FungibleTokenMetadata, metadata),
    (ViewStorageBalanceOf, StorageBalance, balance)
];

#[cfg(not(feature = "ethabi"))]
impl_view_return![(ViewCode, Vec<u8>, code)];
#[cfg(feature = "ethabi")]
// TODO: Feature to support ethabi lib
impl_view_return![(ViewCode, Vec<Token>, code)];

impl TryFrom<ViewResultDetails> for ViewVersion {
    type Error = Error;

    fn try_from(view: ViewResultDetails) -> Result<Self> {
        Ok(ViewVersion {
            version: String::from_utf8(view.result)?,
            logs: view.logs,
        })
    }
}

impl TryFrom<ViewResultDetails> for ViewOwner {
    type Error = Error;

    fn try_from(view: ViewResultDetails) -> Result<Self> {
        Ok(ViewOwner {
            account_id: AccountId::try_from_slice(view.result.as_slice())?,
            logs: view.logs,
        })
    }
}

impl TryFrom<ViewResultDetails> for ViewBridgeProver {
    type Error = Error;

    fn try_from(view: ViewResultDetails) -> Result<Self> {
        Ok(ViewBridgeProver {
            account_id: AccountId::try_from_slice(view.result.as_slice())?,
            logs: view.logs,
        })
    }
}

impl From<ViewResultDetails> for ViewChainId {
    fn from(view: ViewResultDetails) -> Self {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(view.result.as_slice());
        ViewChainId {
            chain_id: U256::from(buf),
            logs: view.logs,
        }
    }
}

impl From<ViewResultDetails> for ViewUpgradeIndex {
    fn from(view: ViewResultDetails) -> Self {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(view.result.as_slice());
        ViewUpgradeIndex {
            upgrade_index: u64::from_le_bytes(buf),
            logs: view.logs,
        }
    }
}

// TODO return this as bitflags.
impl From<ViewResultDetails> for ViewPausedPrecompiles {
    fn from(view: ViewResultDetails) -> Self {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(view.result.as_slice());
        ViewPausedPrecompiles {
            paused_flags: u32::from_le_bytes(buf),
            logs: view.logs,
        }
    }
}

impl From<ViewResultDetails> for ViewBlockHash {
    fn from(view: ViewResultDetails) -> Self {
        ViewBlockHash {
            block_hash: H256::from_slice(view.result.as_slice()),
            logs: view.logs,
        }
    }
}

#[cfg(not(feature = "ethabi"))]
impl From<ViewResultDetails> for ViewCode {
    fn from(view: ViewResultDetails) -> Self {
        Self {
            code: view.result,
            logs: view.logs,
        }
    }
}

#[cfg(feature = "ethabi")]
impl ViewCode {
    pub fn decode(types: &[ParamType], view: ViewResultDetails) -> Result<Self> {
        Ok(Self {
            code: ethabi::decode(types, &view.result)?,
            logs: view.logs,
        })
    }
}

impl From<ViewResultDetails> for ViewBalance {
    fn from(view: ViewResultDetails) -> Self {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(view.result.as_slice());
        Self {
            balance: U256::from(buf),
            logs: view.logs,
        }
    }
}

impl From<ViewResultDetails> for ViewNonce {
    fn from(view: ViewResultDetails) -> Self {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(view.result.as_slice());
        Self {
            nonce: U256::from(buf),
            logs: view.logs,
        }
    }
}

impl From<ViewResultDetails> for ViewStorage {
    fn from(view: ViewResultDetails) -> Self {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(view.result.as_slice());
        Self {
            storage: H256::from(buf),
            logs: view.logs,
        }
    }
}

impl TryFrom<ViewResultDetails> for ViewEvm {
    type Error = Error;

    fn try_from(view: ViewResultDetails) -> Result<Self> {
        let status: TransactionStatus = TransactionStatus::try_from_slice(view.result.as_slice())?;
        Ok(Self {
            status,
            logs: view.logs,
        })
    }
}

impl TryFrom<ViewResultDetails> for ViewIsProofUsed {
    type Error = Error;

    fn try_from(view: ViewResultDetails) -> Result<Self> {
        let is_proof_used: bool = borsh::try_from_slice_with_schema(view.result.as_slice())?;
        Ok(Self {
            is_proof_used,
            logs: view.logs,
        })
    }
}

impl TryFrom<ViewResultDetails> for ViewFtTotalSupply {
    type Error = Error;

    fn try_from(view: ViewResultDetails) -> Result<Self> {
        let total_supply: u128 = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            total_supply,
            logs: view.logs,
        })
    }
}

impl TryFrom<ViewResultDetails> for ViewFtBalanceOf {
    type Error = Error;

    fn try_from(view: ViewResultDetails) -> Result<Self> {
        let balance = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            balance,
            logs: view.logs,
        })
    }
}

impl From<ViewResultDetails> for ViewAddressBalance {
    fn from(view: ViewResultDetails) -> Self {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(view.result.as_slice());
        Self {
            balance: U256::from(buf),
            logs: view.logs,
        }
    }
}

impl TryFrom<ViewResultDetails> for ViewEthTotalSupply {
    type Error = Error;

    fn try_from(view: ViewResultDetails) -> Result<Self> {
        let total_supply: Wei = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            total_supply: total_supply.raw(),
            logs: view.logs,
        })
    }
}

impl TryFrom<ViewResultDetails> for ViewFtMetadata {
    type Error = Error;

    fn try_from(view: ViewResultDetails) -> Result<Self> {
        Ok(Self {
            metadata: serde_json::from_slice(view.result.as_slice())?,
            logs: view.logs,
        })
    }
}

impl TryFrom<ViewResultDetails> for ViewStorageBalanceOf {
    type Error = Error;

    fn try_from(view: ViewResultDetails) -> Result<ViewStorageBalanceOf> {
        Ok(Self {
            balance: serde_json::from_slice(view.result.as_slice())?,
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
    PausedFlags, // TODO
    AccountsCounter, // TODO
    Erc20FromNep141, // TODO
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
