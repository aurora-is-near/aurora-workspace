use crate::transactions::{CallTransaction, ViewTransaction};
use near_workspaces::network::NetworkClient;
use near_workspaces::types::{KeyType, NearToken, SecretKey};
use near_workspaces::{Account, AccountId, Worker};

pub mod compile;
pub mod macros;
pub mod results;
pub mod transactions;

pub trait ContractId {
    fn as_contract(&self) -> &Contract;
    fn id(&self) -> &AccountId;
}

#[derive(Debug, Clone)]
pub enum AccountKind {
    Account {
        contract_id: AccountId,
        inner: Account,
    },
    Contract(near_workspaces::Contract),
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
    pub fn new(contract_id: AccountId, account: Account) -> Self {
        Self {
            account: AccountKind::Account {
                contract_id,
                inner: account,
            },
        }
    }

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

    pub async fn deploy(account: &Account, wasm: Vec<u8>) -> anyhow::Result<Self> {
        let contract = account.deploy(&wasm).await?.into_result()?;
        Ok(Self {
            account: AccountKind::Contract(contract),
        })
    }

    pub async fn create_account_from_random_seed(account_id: AccountId) -> anyhow::Result<Account> {
        let worker = near_workspaces::sandbox()
            .await
            .map_err(|err| anyhow::anyhow!("Failed init sandbox: {:?}", err))?;
        let sk = SecretKey::from_random(KeyType::ED25519);

        Ok(worker.create_tla(account_id, sk).await?.into_result()?)
    }

    pub async fn find_root_account() -> anyhow::Result<Account> {
        let worker = near_workspaces::sandbox()
            .await
            .map_err(|err| anyhow::anyhow!("Failed init sandbox: {:?}", err))?;
        Ok(worker.root_account()?)
    }

    pub async fn create_root_account(
        root_acc_name: &str,
        balance: NearToken,
    ) -> anyhow::Result<Account> {
        use near_workspaces::AccessKey;

        let worker = near_workspaces::sandbox()
            .await
            .map_err(|err| anyhow::anyhow!("Failed init sandbox: {:?}", err))?;
        let testnet = near_workspaces::testnet()
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
            .transfer(balance)
            .transact()
            .await?
            .into_result()?;

        Ok(Account::from_secret_key(root, sk, &worker))
    }

    pub async fn create_sub_account(
        root_account: &Account,
        name: &str,
        balance: NearToken,
    ) -> anyhow::Result<Account> {
        Ok(root_account
            .create_subaccount(name)
            .initial_balance(balance)
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
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
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
