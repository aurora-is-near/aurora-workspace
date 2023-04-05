#![allow(dead_code)]

use aurora_workspace_types::AccountId;
use std::borrow::Borrow;
use workspaces::result::{
    ExecutionFailure, ExecutionFinalResult, ExecutionOutcome, ExecutionSuccess,
};
use workspaces::rpc::query::{Query, ViewFunction};
use workspaces::types::Gas;

#[derive(Debug)]
pub struct ExecutionResult<T> {
    inner: workspaces::result::ExecutionFinalResult,
    pub(crate) value: T,
}

impl<T> ExecutionResult<T> {
    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn into_value(self) -> T {
        self.value
    }

    #[allow(clippy::result_large_err)]
    pub fn into_result(self) -> Result<ExecutionSuccess, ExecutionFailure> {
        self.inner.into_result()
    }

    pub fn total_gas_burnt(&self) -> Gas {
        self.inner.total_gas_burnt
    }

    pub fn outcome(&self) -> &ExecutionOutcome {
        self.inner.outcome()
    }

    pub fn outcomes(&self) -> Vec<&ExecutionOutcome> {
        self.inner.outcomes()
    }

    pub fn receipt_outcomes(&self) -> &[ExecutionOutcome] {
        self.inner.receipt_outcomes()
    }

    pub fn failures(&self) -> Vec<&ExecutionOutcome> {
        self.inner.failures()
    }

    pub fn receipt_failures(&self) -> Vec<&ExecutionOutcome> {
        self.inner.receipt_failures()
    }

    pub fn logs(&self) -> Vec<&str> {
        self.inner.logs()
    }

    pub fn is_success(&self) -> bool {
        self.inner.is_success()
    }

    pub fn is_failure(&self) -> bool {
        self.inner.is_failure()
    }

    pub fn inner(self) -> ExecutionFinalResult {
        self.inner
    }
}

impl<T> AsRef<T> for ExecutionResult<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> Borrow<T> for ExecutionResult<T> {
    fn borrow(&self) -> &T {
        &self.value
    }
}

pub struct CallTransaction<'a> {
    inner: workspaces::operations::CallTransaction<'a>,
}

impl<'a> CallTransaction<'a> {
    pub(crate) fn new(call_tx: workspaces::operations::CallTransaction<'a>) -> Self {
        Self { inner: call_tx }
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

#[derive(Debug, Clone)]
pub enum AccountKind {
    Account {
        contract_id: AccountId,
        inner: workspaces::Account,
    },
    Contract(workspaces::Contract),
}

impl AccountKind {
    fn call<'a, F: AsRef<str>>(&'a self, function: &'a F) -> CallTransaction {
        let transaction = match self {
            AccountKind::Account { contract_id, inner } => {
                inner.call(contract_id, function.as_ref())
            }
            AccountKind::Contract(con) => con.call(function.as_ref()),
        };
        CallTransaction::new(transaction)
    }

    fn view<F: AsRef<str>>(&self, function: &F) -> Query<'_, ViewFunction> {
        match self {
            AccountKind::Account { contract_id, inner } => {
                inner.view(contract_id, function.as_ref())
            }
            AccountKind::Contract(con) => con.view(function.as_ref()),
        }
    }

    fn id(&self) -> &AccountId {
        match self {
            AccountKind::Account { inner, .. } => inner.id(),
            AccountKind::Contract(con) => con.id(),
        }
    }
}

pub trait WorkspaceContract {
    fn contract(&mut self, contract: &Contract);
}

#[derive(Debug, Clone)]
pub struct Contract {
    account: AccountKind,
}

impl Contract {
    pub fn as_account(&self) -> &AccountKind {
        &self.account
    }

    pub fn near_call<'a, F: AsRef<str>>(&'a self, function: &'a F) -> CallTransaction {
        self.account.call(function)
    }

    pub fn near_view<F: AsRef<str>>(&self, function_name: &F) -> Query<'_, ViewFunction> {
        self.account.view(function_name)
    }

    pub fn id(&self) -> &AccountId {
        self.account.id()
    }

    pub async fn deploy(account: workspaces::Account, wasm: Vec<u8>) -> anyhow::Result<Self> {
        let contract = account.deploy(&wasm).await?.into_result()?;
        Ok(Self {
            account: AccountKind::Contract(contract),
        })
    }
}

pub struct EthConnector {
    contract: Contract,
}

impl WorkspaceContract for EthConnector {
    fn contract(&mut self, contract: &Contract) {
        self.contract = contract.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SelfCall {
    SetEthConnectorContractData,
}

impl AsRef<str> for SelfCall {
    fn as_ref(&self) -> &str {
        match self {
            SelfCall::SetEthConnectorContractData => "set_eth_connector_contract_data",
        }
    }
}

impl EthConnector {
    pub fn tst_fn(&self, _balance: u64) -> CallFtTransfer {
        CallFtTransfer::call(&self.contract)
    }
}

macro_rules! impl_call_return  {
    ($(($name:ident => $return:ty, $fn_name:expr, $deser_fn:ident)),* $(,)?) => {
        $(pub struct $name<'a>(CallTransaction<'a>);

        impl<'a> $name<'a> {
            pub(crate) fn call(contract: &'a Contract) -> Self {
                Self(contract.near_call(&$fn_name))
            }

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

            pub async fn transact(self) -> anyhow::Result<ExecutionResult<$return>> {
                ExecutionResult::$deser_fn(self.0.transact().await?)
            }
        })*
    };
    ($(($name:ident, $fn_name:expr)),* $(,)?) => {
        $(pub struct $name<'a>(CallTransaction<'a>);

        impl<'a> $name<'a> {
            pub(crate) fn call(contract: &'a Contract) -> Self {
                Self(contract.near_call(&$fn_name))
            }

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

            pub async fn transact(self) -> anyhow::Result<ExecutionResult<()>> {
                Ok(ExecutionResult {
                    inner: self.0.transact().await?,
                    value: (),
                })
            }
        })*
    };
}

impl_call_return!((CallFtTransfer, SelfCall::SetEthConnectorContractData),);
