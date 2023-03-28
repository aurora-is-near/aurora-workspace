#![allow(dead_code)]
use crate::error::Error;
use crate::result::ExecutionSuccess;
use crate::Result;
use aurora_workspace_types::AccountId;
use borsh::BorshDeserialize;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
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
];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Call {
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

impl ViewResultDetails<U128> {
    pub(crate) fn try_from_json(view: workspaces::result::ViewResultDetails) -> Result<Self> {
        let result: U128 = serde_json::from_slice(view.result.as_slice())?;
        Ok(Self {
            result,
            logs: view.logs,
        })
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

impl From<workspaces::result::ViewResultDetails> for ViewResultDetails<Vec<u8>> {
    fn from(view: workspaces::result::ViewResultDetails) -> Self {
        Self {
            result: view.result,
            logs: view.logs,
        }
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum View {
    IsProofUsed,
    FtTotalSupply,
    FtBalanceOf,
    FtMetadata,
    StorageBalanceOf,
    AccountsCounter,
}

impl AsRef<str> for View {
    fn as_ref(&self) -> &str {
        use View::*;
        match self {
            IsProofUsed => "is_used_proof",
            FtTotalSupply => "ft_total_supply",
            FtBalanceOf => "ft_balance_of",
            FtMetadata => "ft_metadata",
            StorageBalanceOf => "storage_balance_of",
            AccountsCounter => "get_accounts_counter",
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
