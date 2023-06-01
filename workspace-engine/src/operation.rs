use aurora_engine::parameters::{SubmitResult, TransactionStatus, WithdrawResult};
use aurora_workspace_types::{AccountId, Address, H256, U256};
use aurora_workspace_utils::results::{ExecutionResult, ViewResult};
use aurora_workspace_utils::transactions::{CallTransaction, ViewTransaction};
use aurora_workspace_utils::{impl_call_return, impl_view_return, Contract};
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
    (CallRegisterRelayer, Call::RegisterRelayer),
    (CallRefundOnError, Call::RefundOnError),
    (CallFactoryUpdate, Call::FactoryUpdate),
    (CallFactorySetWNearAddress, Call::FactorySetWNearAddress),
    (CallDeployUpgrade, Call::DeployUpgrade),
    (CallResumePrecompiles, Call::ResumePrecompiles),
    (CallPausePrecompiles, Call::PausePrecompiles),
    (CallStageUpgrade, Call::StageUpgrade),
    (CallStateMigration, Call::StateMigration),
];

// Eth-connector
impl_call_return![
    (CallFtTransferCall => PromiseOrValue<U128>, Call::FtTransferCall, try_from),
    (CallStorageDeposit => StorageBalance, Call::StorageDeposit, json),
    (CallStorageUnregister => bool, Call::StorageUnregister, json),
    (CallStorageWithdraw => StorageBalance, Call::StorageWithdraw, json),
    (CallWithdraw => WithdrawResult, Call::Withdraw, borsh),
    (CallDeployCode => SubmitResult, Call::DeployCode, borsh),
    (CallDeployErc20Token => Address, Call::DeployErc20Token, borsh),
    (CallCall => SubmitResult, Call::Call, borsh),
    (CallSubmit => SubmitResult, Call::Submit, borsh),
    (CallFtOnTransfer => U128, Call::Call, json),
];

impl_view_return![
    (ViewFtTotalSupply => U128, View::FtTotalSupply, json),
    (ViewFtBalanceOf => U128, View::FtBalanceOf, json),
    (ViewStorageBalanceOf => StorageBalance, View::StorageBalanceOf, json),
    (ViewFtMetadata => FungibleTokenMetadata, View::FtMetadata, json),
    (ViewVersion => String, View::Version, borsh),
    (ViewOwner => AccountId, View::Owner, borsh),
    (ViewBridgeProver => AccountId, View::BridgeProver, borsh),
    (ViewChainId => AccountId, View::ChainId, borsh),
    (ViewUpgradeIndex => u64, View::UpgradeIndex, borsh),
    (ViewPausedPrecompiles => u32, View::PausedPrecompiles, borsh),
    (ViewBlockHash => H256, View::BlockHash, borsh_H256),
    (ViewCode => Vec<u8>, View::Code, borsh),
    (ViewBalance => U256, View::Balance, borsh_U256),
    (ViewNonce => U256, View::Nonce, borsh_U256),
    (ViewStorageAt => H256, View::StorageAt, borsh_H256),
    (ViewView => TransactionStatus, View::View, borsh),

];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub(crate) enum Call {
    New,
    DeployCode,
    DeployErc20Token,
    Call,
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
    FactorySetWNearAddress,
    SetEthConnectorContractData,
    FactoryUpdateAddressVersion,
    RefundOnError,
}

impl AsRef<str> for Call {
    fn as_ref(&self) -> &str {
        match self {
            Call::New => "new",
            Call::DeployCode => "deploy_code",
            Call::DeployErc20Token => "deploy_erc20_token",
            Call::Call => "call",
            Call::Submit => "submit",
            Call::RegisterRelayer => "register_relayer",
            Call::FtOnTransfer => "ft_on_transfer",
            Call::Withdraw => "withdraw",
            Call::Deposit => "deposit",
            Call::FtTransfer => "ft_transfer",
            Call::FtTransferCall => "ft_transfer_call",
            Call::StorageDeposit => "storage_deposit",
            Call::StorageUnregister => "storage_unregister",
            Call::StorageWithdraw => "storage_withdraw",
            Call::PausePrecompiles => "pause_precompiles",
            Call::StageUpgrade => "stage_upgrade",
            Call::DeployUpgrade => "deploy_upgrade",
            Call::StateMigration => "state_migration",
            Call::ResumePrecompiles => "resume_precompiles",
            Call::FactoryUpdate => "factory_update",
            Call::FactorySetWNearAddress => "factory_set_wnear_address",
            Call::SetEthConnectorContractData => "set_eth_connector_contract_data",
            Call::FactoryUpdateAddressVersion => "factory_update_address_version",
            Call::RefundOnError => "refund_on_error",
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
    View,
    IsUsedProof,
    FtTotalSupply,
    FtBalanceOf,
    BalanceOfEth,
    FtTotalSupplyOnAurora,
    FtMetadata,
    StorageBalanceOf,
    PausedFlags,
    Erc20FromNep141,
    Nep141FromErc20,
}

impl AsRef<str> for View {
    fn as_ref(&self) -> &str {
        match self {
            View::Version => "get_version",
            View::Owner => "get_owner",
            View::BridgeProver => "get_bridge_prover",
            View::ChainId => "get_chain_id",
            View::UpgradeIndex => "get_upgrade_index",
            View::PausedPrecompiles => "get_paused_precompiles",
            View::BlockHash => "get_block_hash",
            View::Code => "get_code",
            View::Balance => "get_balance",
            View::Nonce => "get_nonce",
            View::StorageAt => "get_storage_at",
            View::View => "get_view",
            View::IsUsedProof => "is_used_proof",
            View::FtTotalSupply => "ft_total_supply",
            View::FtBalanceOf => "ft_balance_of",
            View::BalanceOfEth => "ft_balance_of_eth",
            View::FtTotalSupplyOnAurora => "ft_total_eth_supply_on_aurora",
            View::FtMetadata => "ft_metadata",
            View::StorageBalanceOf => "storage_balance_of",
            View::PausedFlags => "get_paused_flags",
            View::Erc20FromNep141 => "get_erc20_from_nep141",
            View::Nep141FromErc20 => "get_nep141_from_erc20",
        }
    }
}

/*
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViewResultDetails<T> {
    pub result: T,
    pub logs: Vec<String>,
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
*/
