use workspaces::operations::CallTransaction;
use workspaces::result::ExecutionFinalResult;
use crate::Result;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Function {
    Call(Call),
    View(View),
    SelfCall(SelfCall),
    AuthorizedCall(AuthorizedCall),
    OwnerCall(OwnerCall),
}

impl AsRef<str> for Function {
    fn as_ref(&self) -> &str {
        use Function::*;
        match self {
            Call(c) => c.as_ref(),
            View(v) => v.as_ref(),
            SelfCall(c) => c.as_ref(),
            AuthorizedCall(c) => c.as_ref(),
            OwnerCall(c) => c.as_ref(),
        }
    }
}

impl From<Call> for Function {
    fn from(call: Call) -> Self {
        Function::Call(call)
    }
}

impl From<View> for Function {
    fn from(view: View) -> Self {
        Function::View(view)
    }
}

impl From<SelfCall> for Function {
    fn from(self_call: SelfCall) -> Self {
        Function::SelfCall(self_call)
    }
}

impl From<AuthorizedCall> for Function {
    fn from(authorized_call: AuthorizedCall) -> Self {
        Function::AuthorizedCall(authorized_call)
    }
}

impl From<OwnerCall> for Function {
    fn from(owner_call: OwnerCall) -> Self {
        Function::OwnerCall(owner_call)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Call {
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
    function: &'b Function,
}

impl<'a, 'b> EvmCallTransaction<'a, 'b> {
    pub(crate) fn call(function: &'b Function, transaction: CallTransaction<'a, 'b>) -> Self {
        EvmCallTransaction{
            inner: transaction,
            function,
        }
    }

    pub(crate) fn args(mut self, args: Vec<u8>) -> EvmCallTransaction<'a, 'b> {
        self.inner = self.inner.args(args);
        self
    }

    pub(crate) fn args_json<S: serde::Serialize>(mut self, args: S) -> EvmCallTransaction<'a, 'b> {
        self.inner = self.inner.args_json(args);
        self
    }

    pub(crate) fn args_borsh<B: borsh::BorshSerialize>(mut self, args: B) -> EvmCallTransaction<'a, 'b> {
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

    async fn transact(self) -> Result<ExecutionFinalResult> {
        // TODO: return EVM execution result.
        Ok(self.inner.transact().await?)
    }
}
