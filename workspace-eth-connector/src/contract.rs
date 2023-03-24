use crate::operation::{Call, CallFtTransfer, CallFtTransferCall, EthConnectorCallTransaction};
use crate::Result;
use aurora_workspace_types::AccountId;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::json_types::U128;
use serde_json::json;
use workspaces::{Account, Contract};

#[derive(Debug, Clone)]
pub struct EthConnectorAccount {
    account: AccountKind,
}

impl EthConnectorAccount {
    pub async fn new(account: Account, contract_id: AccountId) -> Self {
        Self {
            account: AccountKind::Account {
                contract_id,
                inner: account,
            },
        }
    }

    pub fn near_call<'a, F: AsRef<str> + ?Sized>(
        &'a self,
        function: &'a F,
    ) -> EthConnectorCallTransaction<'_> {
        self.account.call(function)
    }

    pub async fn near_view<F: AsRef<str>>(
        &self,
        function: &F,
        args: Vec<u8>,
    ) -> Result<workspaces::result::ViewResultDetails> {
        self.account.view(function, args).await
    }

    pub fn ft_transfer(
        &self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    ) -> CallFtTransfer<'_> {
        CallFtTransfer(
            self.near_call(&Call::FtTransfer)
                .args_json(json!({ "receiver_id": receiver_id, "amount": amount, "memo": memo })),
        )
    }

    pub fn ft_transfer_call(
        &self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> CallFtTransferCall<'_> {
        CallFtTransferCall(self.near_call(&Call::FtTransferCall).args_json(json!({
            "receiver_id": receiver_id,
            "amount": amount,
            "memo": memo,
            "msg": msg,
        })))
    }
    /*
    pub fn storage_deposit<A: AsRef<str>>(
        &self,
        account_id: Option<A>,
        registration_only: Option<bool>,
    ) -> CallStorageDeposit<'_> {
        let args = json!({ "receiver_id": "" });
        CallStorageDeposit(self.near_call(&Call::StorageDeposit).args_json(args))
    }

    pub fn storage_unregister(&self, force: bool) -> CallStorageUnregister<'_> {
        let val = serde_json::json!({ "force": force });
        CallStorageUnregister(self.near_call(&Call::StorageUnregister).args_json(val))
    }

    pub fn storage_withdraw(&self, amount: Option<u128>) -> CallStorageWithdraw<'_> {
        let args = json!({ "receiver_id": "" });
        CallStorageWithdraw(self.near_call(&Call::StorageWithdraw).args_json(args))
    }

    pub async fn is_proof_used(&self, proof: ProofInput) -> Result<ViewResultDetails<bool>> {
        let args = IsUsedProofCallArgs { proof };
        ViewResultDetails::try_from(
            self.near_view(&View::IsProofUsed, args.try_to_vec()?)
                .await?,
        )
    }

    pub async fn ft_total_supply(&self) -> Result<ViewResultDetails<u128>> {
        ViewResultDetails::try_from(self.near_view(&View::FtTotalSupply, vec![]).await?)
    }

    pub async fn ft_balance_of<A: AsRef<str>>(
        &self,
        account_id: A,
    ) -> Result<ViewResultDetails<u128>> {
        let account = AccountId::from_str(account_id.as_ref()).unwrap();
        let args = borsh::to_vec(&account).unwrap();
        ViewResultDetails::try_from(self.near_view(&View::FtBalanceOf, args).await?)
    }*/
}

#[derive(Debug, Clone)]
pub enum AccountKind {
    Account {
        contract_id: AccountId,
        inner: Account,
    },
    Contract(Contract),
}

impl AccountKind {
    fn call<'a, F: AsRef<str> + ?Sized>(
        &'a self,
        function: &'a F,
    ) -> EthConnectorCallTransaction<'_> {
        let transaction = match self {
            AccountKind::Account { contract_id, inner } => {
                inner.call(contract_id, function.as_ref())
            }
            AccountKind::Contract(con) => con.call(function.as_ref()),
        };
        EthConnectorCallTransaction::call(transaction)
    }

    async fn view<F: AsRef<str>>(
        &self,
        function: &F,
        args: Vec<u8>,
    ) -> Result<workspaces::result::ViewResultDetails> {
        Ok(match self {
            AccountKind::Account { contract_id, inner } => {
                inner
                    .view(contract_id, function.as_ref())
                    .args(args)
                    .await?
            }
            AccountKind::Contract(con) => con.view(function.as_ref()).args(args).await?,
        })
    }

    pub fn id(&self) -> &AccountId {
        match self {
            AccountKind::Account { inner, .. } => inner.id(),
            AccountKind::Contract(con) => con.id(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EthConnectorContract {
    contract: EthConnectorAccount,
}

impl AsRef<EthConnectorAccount> for EthConnectorContract {
    fn as_ref(&self) -> &EthConnectorAccount {
        &self.contract
    }
}

impl AsMut<EthConnectorAccount> for EthConnectorContract {
    fn as_mut(&mut self) -> &mut EthConnectorAccount {
        &mut self.contract
    }
}

impl From<Contract> for EthConnectorAccount {
    fn from(contract: Contract) -> Self {
        EthConnectorAccount {
            account: AccountKind::Contract(contract),
        }
    }
}

impl EthConnectorContract {
    pub fn new(contract: Contract) -> Self {
        Self {
            contract: EthConnectorAccount::from(contract),
        }
    }

    pub async fn deploy_and_init(
        account: Account,
        prover_account: AccountId,
        eth_custodian_address: String,
        metadata: FungibleTokenMetadata,
        account_with_access_right: &AccountId,
        owner_id: AccountId,
        wasm: Vec<u8>,
    ) -> Result<Self> {
        let contract = Self::deploy(account, wasm).await?;
        contract
            .init(
                prover_account,
                eth_custodian_address,
                metadata,
                account_with_access_right,
                owner_id,
            )
            .await?;
        Ok(contract)
    }

    pub async fn deploy(account: Account, wasm: Vec<u8>) -> Result<EthConnectorContract> {
        let contract = account.deploy(&wasm).await?.into_result()?;
        Ok(Self::new(contract))
    }

    /// Init eth-connector
    pub async fn init(
        &self,
        prover_account: AccountId,
        eth_custodian_address: String,
        metadata: FungibleTokenMetadata,
        account_with_access_right: &AccountId,
        owner_id: AccountId,
    ) -> Result<()> {
        self.contract
            .near_call("new")
            .args_json(json!({
                "prover_account": prover_account,
                "account_with_access_right": account_with_access_right,
                "owner_id": owner_id,
                "eth_custodian_address": eth_custodian_address,
                "metadata": metadata,
            }))
            .transact()
            .await?
            .into_result()?;

        Ok(())
    }

    pub fn as_account(&self) -> &EthConnectorAccount {
        &self.contract
    }
}
