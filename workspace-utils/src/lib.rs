#![allow(dead_code)]

use aurora_workspace_types::AccountId;
use serde::de::DeserializeOwned;
use std::borrow::Borrow;
use workspaces::network::NetworkClient;
use workspaces::result::{ExecutionFinalResult, ExecutionOutcome};
use workspaces::rpc::query::{Query, ViewFunction};
use workspaces::types::{Gas, KeyType, SecretKey};
use workspaces::{Account, Worker};

#[derive(Debug)]
pub struct ViewResult<T> {
    pub result: T,
    pub logs: Vec<String>,
}

impl<T: DeserializeOwned> ViewResult<T> {
    pub(crate) fn json(view: workspaces::result::ViewResultDetails) -> anyhow::Result<Self> {
        Ok(Self {
            result: view.json()?,
            logs: view.logs,
        })
    }
}

impl<T: borsh::BorshDeserialize> ViewResult<T> {
    pub(crate) fn borsh(view: workspaces::result::ViewResultDetails) -> anyhow::Result<Self> {
        Ok(Self {
            result: view.borsh()?,
            logs: view.logs,
        })
    }
}

#[derive(Debug)]
pub struct ExecutionResult<T> {
    inner: workspaces::result::ExecutionSuccess,
    value: T,
    success: bool,
}

impl<T: DeserializeOwned> ExecutionResult<T> {
    pub(crate) fn json(result: workspaces::result::ExecutionFinalResult) -> anyhow::Result<Self> {
        let success = result.is_success();
        let inner = result.into_result()?;
        let value = inner.json()?;
        Ok(Self::new(inner, value, success))
    }
}

impl<T: borsh::BorshDeserialize> ExecutionResult<T> {
    pub(crate) fn borsh(result: workspaces::result::ExecutionFinalResult) -> anyhow::Result<Self> {
        let success = result.is_success();
        let inner = result.into_result()?;
        let value = inner.borsh()?;
        Ok(Self::new(inner, value, success))
    }
}

impl<T> ExecutionResult<T> {
    pub fn new(inner: workspaces::result::ExecutionSuccess, value: T, success: bool) -> Self {
        Self {
            inner,
            value,
            success,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn into_value(self) -> T {
        self.value
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
        self.success
    }

    pub fn is_failure(&self) -> bool {
        !self.success
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

pub struct ViewTransaction<'a> {
    inner: Query<'a, ViewFunction>,
}

impl<'a> ViewTransaction<'a> {
    pub(crate) fn new(view_tx: Query<'a, ViewFunction>) -> Self {
        Self { inner: view_tx }
    }

    pub fn args_json<U: serde::Serialize>(mut self, args: U) -> Self {
        self.inner = self.inner.args_json(args);
        self
    }

    pub fn args_borsh<U: borsh::BorshSerialize>(mut self, args: U) -> Self {
        self.inner = self.inner.args_borsh(args);
        self
    }

    pub(crate) async fn transact(self) -> anyhow::Result<workspaces::result::ViewResultDetails> {
        Ok(self.inner.await?)
    }
}

pub struct CallTransaction<'a> {
    inner: workspaces::operations::CallTransaction<'a>,
}

impl<'a> CallTransaction<'a> {
    pub(crate) fn new(call_tx: workspaces::operations::CallTransaction<'a>) -> Self {
        Self { inner: call_tx }
    }

    pub fn args_json<S: serde::Serialize>(mut self, args: S) -> Self {
        self.inner = self.inner.args_json(args);
        self
    }

    pub fn args_borsh<B: borsh::BorshSerialize>(mut self, args: B) -> Self {
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

    fn view<F: AsRef<str>>(&self, function: &F) -> ViewTransaction {
        let transaction = match self {
            AccountKind::Account { contract_id, inner } => {
                inner.view(contract_id, function.as_ref())
            }
            AccountKind::Contract(con) => con.view(function.as_ref()),
        };
        ViewTransaction::new(transaction)
    }

    fn id(&self) -> &AccountId {
        match self {
            AccountKind::Account { inner, .. } => inner.id(),
            AccountKind::Contract(con) => con.id(),
        }
    }
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

    pub fn near_view<F: AsRef<str>>(&self, function_name: &F) -> ViewTransaction {
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

    pub async fn create_account_from_random_seed(
        account_id: AccountId,
    ) -> anyhow::Result<workspaces::Account> {
        let worker = workspaces::sandbox()
            .await
            .map_err(|err| anyhow::anyhow!("Failed init sandbox: {:?}", err))?;
        let sk = SecretKey::from_random(KeyType::ED25519);
        Ok(worker.create_tla(account_id, sk).await?.into_result()?)
    }

    pub async fn create_root_account(root_acc_name: &str) -> anyhow::Result<workspaces::Account> {
        use workspaces::AccessKey;

        let worker = workspaces::sandbox()
            .await
            .map_err(|err| anyhow::anyhow!("Failed init sandbox: {:?}", err))?;
        let testnet = workspaces::testnet()
            .await
            .map_err(|err| anyhow::anyhow!("Failed init testnet: {:?}", err))?;
        let registrar: AccountId = "registrar".parse()?;
        let registrar = worker
            .import_contract(&registrar, &testnet)
            .transact()
            .await?;
        Self::waiting_account_creation(&worker, registrar.id()).await?;
        let sk = SecretKey::from_seed(KeyType::ED25519, "registrar");
        let root: AccountId = root_acc_name.parse()?;
        registrar
            .as_account()
            .batch(&root)
            .create_account()
            .add_key(sk.public_key(), AccessKey::full_access())
            .transfer(near_units::parse_near!("200 N"))
            .transact()
            .await?
            .into_result()?;

        Ok(Account::from_secret_key(root, sk, &worker))
    }

    pub async fn create_sub_account(root_account: Account, name: &str) -> anyhow::Result<Account> {
        Ok(root_account
            .create_subaccount(name)
            .initial_balance(near_units::parse_near!("15 N"))
            .transact()
            .await?
            .into_result()?)
    }

    /// Waiting for the account creation
    async fn waiting_account_creation<T: NetworkClient + ?Sized>(
        worker: &Worker<T>,
        account_id: &AccountId,
    ) -> anyhow::Result<()> {
        let timer = std::time::Instant::now();
        // Try to get account within 30 secs
        for _ in 0..60 {
            if worker.view_account(account_id).await.is_err() {
                //tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            } else {
                return Ok(());
            }
        }

        anyhow::bail!(
            "Account `{}` was not created in {:?} sec",
            account_id,
            timer.elapsed()
        )
    }
}

#[macro_export]
macro_rules! impl_view_return  {
    ($(($name:ident => $return:ty, $fn_name:expr, $deser_fn:ident)),* $(,)?) => {
        $(pub struct $name<'a>(ViewTransaction<'a>);
        impl<'a> $name<'a> {
            pub(crate) fn view(contract: &'a Contract) -> Self {
                Self(contract.near_view(&$fn_name))
            }
            pub(crate) fn args_json<S: serde::Serialize>(mut self, args: S) -> Self {
                self.0 = self.0.args_json(args);
                self
            }
            pub(crate) fn args_borsh<B: borsh::BorshSerialize>(mut self, args: B) -> Self {
                self.0 = self.0.args_borsh(args);
                self
            }
            pub async fn transact(self)  -> anyhow::Result<ViewResult<$return>> {
                ViewResult::$deser_fn(self.0.transact().await?)
            }
        })*
    };
}

#[macro_export]
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
            pub(crate) fn args_json<S: serde::Serialize>(mut self, args: S) -> Self {
                self.0 = self.0.args_json(args);
                self
            }
            pub(crate) fn args_borsh<B: borsh::BorshSerialize>(mut self, args: B) -> Self {
                self.0 = self.0.args_borsh(args);
                self
            }
            pub(crate) async fn transact(self) -> anyhow::Result<ExecutionResult<$return>> {
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
            pub(crate) fn args_json<S: serde::Serialize>(mut self, args: S) -> Self {
                self.0 = self.0.args_json(args);
                self

            }
            pub(crate) fn args_borsh<B: borsh::BorshSerialize>(mut self, args: B) -> Self {
                self.0 = self.0.args_borsh(args);
                self
            }
            pub async fn transact(self) -> anyhow::Result<ExecutionResult<()>> {
                let result = self.0.transact().await?;
                let success = result.is_success();
                let inner = result.into_result()?;
                Ok(ExecutionResult::new(inner, (), success))
            }
        })*
    };
}

//=========================================
impl_call_return!(
    (CallFtTransfer, SelfCall::SetEthConnectorContractData),
    (CallNew, SelfCall::SetEthConnectorContractData),
);
impl_call_return!(
    (CallFtTransfer1 => u8, SelfCall::SetEthConnectorContractData, borsh),
);
impl_view_return!((ViewFtTransfer => u64, SelfCall::SetEthConnectorContractData, borsh),);

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

pub struct EthConnector {
    contract: Contract,
}

impl EthConnector {
    pub fn init(&self, balance: u64) -> CallNew {
        CallNew::call(&self.contract).args_json((balance,))
    }
    pub fn tst_fn(&self, balance: u64) -> CallFtTransfer {
        CallFtTransfer::call(&self.contract).args_borsh((balance, 33))
    }
    pub fn tst_v_fn(&self, balance: u64) -> ViewFtTransfer {
        ViewFtTransfer::view(&self.contract).args_borsh((balance, 33))
    }

    pub async fn deploy_and_init(account: Account) -> anyhow::Result<Self> {
        let contract = Contract::deploy(account, vec![]).await?;
        let eth_contract = Self { contract };
        let res = eth_contract.init(1).transact().await?;
        assert!(res.is_success());
        Ok(eth_contract)
    }
}

pub async fn tstq() -> anyhow::Result<()> {
    use std::str::FromStr;
    let worker = workspaces::sandbox().await.unwrap();
    let sk = SecretKey::from_random(KeyType::ED25519);
    let account = worker
        .create_tla(AccountId::from_str("tst.test.near").unwrap(), sk)
        .await?
        .into_result()?;

    let contract = EthConnector::deploy_and_init(account).await?;
    contract.tst_fn(1).transact().await?;
    Ok(())
}

pub async fn tstw() -> anyhow::Result<()> {
    use std::str::FromStr;
    let worker = workspaces::sandbox().await.unwrap();
    let sk = SecretKey::from_random(KeyType::ED25519);
    let account = worker
        .create_tla(AccountId::from_str("tst.test.near").unwrap(), sk)
        .await?
        .into_result()?;

    let contract = EthConnector::deploy_and_init(account).await?;
    let _res: u64 = contract.tst_v_fn(1).transact().await?.result;
    Ok(())
}
