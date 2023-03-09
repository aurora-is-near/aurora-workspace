use crate::operation::{
    Call, CallDeployCode, CallDeployErc20, CallDeployUpgrade, CallEvm, CallFactorySetWNearAddress,
    CallFactoryUpdate, CallFactoryUpdateAddressVersion, CallFtOnTransfer, CallFtTransfer,
    CallFtTransferCall, CallRefundOnError, CallRegisterRelayer, CallResumePrecompiles,
    CallSetEthConnectorContractData, CallSetPausedFlags, CallStageUpgrade, CallStateMigration,
    CallStorageDeposit, CallStorageUnregister, CallStorageWithdraw, CallSubmit, OwnerCall,
    SelfCall, View, ViewResultDetails,
};
#[cfg(feature = "deposit-withdraw")]
use crate::operation::{CallDeposit, CallWithdraw};
use crate::{EngineCallTransaction, Result};
use aurora_engine::{fungible_token::FungibleTokenMetadata, parameters::PausePrecompilesCallArgs};
use aurora_engine::{
    parameters::{
        GetStorageAtArgs, PauseEthConnectorCallArgs, SetContractDataCallArgs, StorageBalance,
        StorageDepositCallArgs, StorageWithdrawCallArgs, TransactionStatus, TransferCallArgs,
        ViewCallArgs,
    },
    xcc::AddressVersionUpdateArgs,
};
use aurora_workspace_types::input::IsUsedProofCallArgs;
use aurora_workspace_types::input::ProofInput;
#[cfg(feature = "deposit-withdraw")]
use aurora_workspace_types::input::WithdrawInput;
use aurora_workspace_types::input::{CallInput, DeployErc20Input, FtOnTransferInput};
use aurora_workspace_types::{AccountId, Address, Raw, H256, U256};
use borsh::BorshSerialize;
#[cfg(feature = "ethabi")]
use ethabi::{ParamType, Token};
use near_sdk::json_types::U128;
use serde_json::json;
use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;
use std::path::Path;
use std::str::FromStr;
use workspaces::types::SecretKey;
use workspaces::{Account, Contract, Network, Worker};

#[derive(Debug, Clone)]
enum AccountKind {
    Account {
        contract_id: AccountId,
        inner: Account,
    },
    Contract(Contract),
}

impl AccountKind {
    fn call<'a, F: AsRef<str> + ?Sized>(&'a self, function: &'a F) -> EngineCallTransaction<'_> {
        let transaction = match self {
            AccountKind::Account { contract_id, inner } => {
                inner.call(contract_id, function.as_ref())
            }
            AccountKind::Contract(con) => con.call(function.as_ref()),
        };
        EngineCallTransaction::call(transaction)
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

    fn id(&self) -> &AccountId {
        match self {
            AccountKind::Account { inner, .. } => inner.id(),
            AccountKind::Contract(con) => con.id(),
        }
    }
}

// TODO(engine): Self should be able to call owner functions.
pub trait PrivateFunctions: UserFunctions {}

pub trait OwnerFunctions: UserFunctions {}

pub trait ProverFunctions: UserFunctions {}

pub trait UserFunctions: private::Sealed {}

pub trait Tester {}

#[derive(Debug, Clone)]
pub struct Private;

impl PrivateFunctions for Private {}

impl UserFunctions for Private {}

impl private::Sealed for Private {}

#[derive(Debug, Clone)]
pub struct Owner;

impl OwnerFunctions for Owner {}

impl UserFunctions for Owner {}

impl private::Sealed for Owner {}

#[derive(Debug, Clone)]
pub struct Prover;

impl ProverFunctions for Prover {}

impl UserFunctions for Prover {}

impl private::Sealed for Prover {}

#[derive(Debug, Clone)]
pub struct User;

impl UserFunctions for User {}

impl private::Sealed for User {}

#[derive(Debug, Clone)]
pub struct EvmAccount<U: UserFunctions> {
    account: AccountKind,
    phantom: PhantomData<U>,
}

impl EvmAccount<Private> {
    pub fn with_self(contract: Contract) -> EvmAccount<Private> {
        Self {
            account: AccountKind::Contract(contract),
            phantom: PhantomData::default(),
        }
    }
}

impl EvmAccount<Owner> {
    pub fn with_owner(account: Account, contract_id: AccountId) -> EvmAccount<Owner> {
        Self {
            account: AccountKind::Account {
                contract_id,
                inner: account,
            },
            phantom: PhantomData::default(),
        }
    }
}

impl EvmAccount<Prover> {
    pub fn with_prover(account: Account, contract_id: AccountId) -> EvmAccount<Prover> {
        Self {
            account: AccountKind::Account {
                contract_id,
                inner: account,
            },
            phantom: PhantomData::default(),
        }
    }
}

impl EvmAccount<User> {
    pub async fn new(account: Account, contract_id: AccountId) -> EvmAccount<User> {
        Self {
            account: AccountKind::Account {
                contract_id,
                inner: account,
            },
            phantom: PhantomData::default(),
        }
    }
}

impl<U: UserFunctions> EvmAccount<U> {
    fn near_call<'a, F: AsRef<str> + ?Sized>(
        &'a self,
        function: &'a F,
    ) -> EngineCallTransaction<'_> {
        self.account.call(function)
    }

    async fn near_view<F: AsRef<str>>(
        &self,
        function: &F,
        args: Vec<u8>,
    ) -> Result<workspaces::result::ViewResultDetails> {
        self.account.view(function, args).await
    }

    pub fn id(&self) -> &AccountId {
        self.account.id()
    }

    pub fn set_eth_connector_contract_data(
        &self,
        prover_account: impl AsRef<str>,
        eth_custodian_address: impl Into<String>,
        metadata: FungibleTokenMetadata,
    ) -> CallSetEthConnectorContractData<'_> {
        let args = SetContractDataCallArgs {
            prover_account: aurora_engine_types::account_id::AccountId::new(
                prover_account.as_ref(),
            )
            .unwrap(),
            eth_custodian_address: eth_custodian_address.into(),
            metadata,
        };
        CallSetEthConnectorContractData(
            self.near_call(&SelfCall::SetEthConnectorContractData)
                .args_borsh(args),
        )
    }

    pub fn set_paused_flags(&self, paused_mask: u8) -> CallSetPausedFlags<'_> {
        let args = PauseEthConnectorCallArgs { paused_mask };
        CallSetPausedFlags(self.near_call(&SelfCall::SetPausedFlags).args_borsh(args))
    }

    pub fn factory_update_address_version(
        &self,
        address: impl Into<Address>,
        version: u32,
    ) -> CallFactoryUpdateAddressVersion<'_> {
        let args = AddressVersionUpdateArgs {
            address: aurora_engine_types::types::Address::new(address.into()),
            version: aurora_engine::xcc::CodeVersion(version),
        };
        CallFactoryUpdateAddressVersion(
            self.near_call(&SelfCall::FactoryUpdateAddressVersion)
                .args_borsh(args),
        )
    }

    pub fn refund_on_error<A: Into<Address>>(
        &self,
        recipient_address: A,
        erc20_address: Option<A>,
        amount: U256,
    ) -> CallRefundOnError<'_> {
        let mut raw_amount: aurora_engine_types::types::RawU256 = Default::default();
        amount.to_big_endian(&mut raw_amount);
        let args = aurora_engine_types::parameters::RefundCallArgs {
            recipient_address: aurora_engine_types::types::Address::new(recipient_address.into()),
            erc20_address: erc20_address
                .map(Into::into)
                .map(aurora_engine_types::types::Address::new),
            amount: raw_amount,
        };
        CallRefundOnError(self.near_call(&SelfCall::RefundOnError).args_borsh(args))
    }

    /// Deploys contract code using the caller's NEAR account ID as an Ethereum address.
    ///
    /// The logic which creates the ETH address is as follows:
    ///
    /// `Address = keccak(NEAR account Id)[12..]`
    pub fn deploy_code(&self, code: Vec<u8>) -> CallDeployCode<'_> {
        let args = Raw(code);
        CallDeployCode(self.near_call(&Call::DeployCode).args_borsh(args))
    }

    /// Deploys an ERC-20 contract for a given NEP-141 account ID.
    ///
    /// The calling NEAR account ID is translated to an Ethereum address for
    /// deployment with the given logic:
    ///
    /// `Address = keccak(NEAR account Id)[12..]`
    pub fn deploy_erc20(&self, account_id: AccountId) -> CallDeployErc20<'_> {
        let args = DeployErc20Input { nep141: account_id };
        CallDeployErc20(self.near_call(&Call::DeployErc20Token).args_borsh(args))
    }

    pub fn call<A: Into<U256>>(&self, contract: Address, amount: A, input: Vec<u8>) -> CallEvm<'_> {
        let value: U256 = amount.into();
        let mut buf = [0u8; 32];
        value.to_big_endian(&mut buf);
        let args = CallInput {
            contract: contract.0,
            value: buf,
            input,
        };
        CallEvm(self.near_call(&Call::Evm).args_borsh(args))
    }

    pub fn submit(&self, input: Vec<u8>) -> CallSubmit<'_> {
        CallSubmit(self.near_call(&Call::Submit).args_borsh(Raw(input)))
    }

    pub fn register_relayer<A: Into<Address>>(&self, address: A) -> CallRegisterRelayer<'_> {
        CallRegisterRelayer(
            self.near_call(&Call::RegisterRelayer)
                .args(address.into().0.to_vec()),
        )
    }

    #[cfg(feature = "deposit-withdraw")]
    pub fn deposit(&self, proof: ProofInput) -> CallDeposit<'_> {
        CallDeposit(self.near_call(&Call::Deposit).args_borsh(proof))
    }

    #[cfg(feature = "deposit-withdraw")]
    pub fn withdraw<R: Into<Address>, A: Into<u128>>(
        &self,
        receiver_address: R,
        amount: A,
    ) -> CallWithdraw<'_> {
        let args = WithdrawInput {
            recipient_address: receiver_address.into().0,
            amount: amount.into(),
        };
        CallWithdraw(self.near_call(&Call::Withdraw).args_borsh(args))
    }

    pub fn ft_transfer<R: AsRef<str>, A: Into<U128>>(
        &self,
        receiver_id: R,
        amount: A,
        memo: Option<String>,
    ) -> CallFtTransfer<'_> {
        let args = TransferCallArgs {
            // TODO: impl error
            receiver_id: aurora_engine_types::account_id::AccountId::new(receiver_id.as_ref())
                .unwrap(),
            amount: aurora_engine_types::types::NEP141Wei::new(amount.into().0),
            memo,
        };
        CallFtTransfer(self.near_call(&Call::FtTransfer).args_json(args))
    }

    pub fn ft_on_transfer<A: Into<U128>, R: AsRef<str>>(
        &self,
        sender_id: R,
        amount: A,
        message: String,
    ) -> CallFtOnTransfer<'_> {
        let args = FtOnTransferInput {
            sender_id: AccountId::from_str(sender_id.as_ref()).unwrap(),
            amount: amount.into(),
            msg: message,
        };
        CallFtOnTransfer(self.near_call(&Call::FtOnTransfer).args_json(args))
    }

    pub fn ft_transfer_call<R: AsRef<str>>(
        &self,
        receiver_id: R,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> CallFtTransferCall<'_> {
        let args = json!( {
            "receiver_id": aurora_engine_types::account_id::AccountId::new(receiver_id.as_ref())
                .unwrap(),
            "amount": amount,
            "memo": memo,
            "msg": msg,
        });
        CallFtTransferCall(self.near_call(&Call::FtTransferCall).args_json(args))
    }

    // TODO we are not NEP-145 compliant
    pub fn storage_deposit<A: AsRef<str>>(
        &self,
        account_id: Option<A>,
        registration_only: Option<bool>,
    ) -> CallStorageDeposit<'_> {
        let args = StorageDepositCallArgs {
            account_id: account_id
                .map(|a| aurora_engine_types::account_id::AccountId::new(a.as_ref()).unwrap()),
            registration_only,
        };
        CallStorageDeposit(self.near_call(&Call::StorageDeposit).args_json(args))
    }

    pub fn storage_unregister(&self, force: bool) -> CallStorageUnregister<'_> {
        let val = serde_json::json!({ "force": force });
        CallStorageUnregister(self.near_call(&Call::StorageUnregister).args_json(val))
    }

    pub fn storage_withdraw(&self, amount: Option<u128>) -> CallStorageWithdraw<'_> {
        let args = StorageWithdrawCallArgs {
            amount: amount.map(aurora_engine_types::types::Yocto::new),
        };
        CallStorageWithdraw(self.near_call(&Call::StorageWithdraw).args_json(args))
    }

    pub fn factory_update(&self, bytes: Vec<u8>) -> CallFactoryUpdate<'_> {
        CallFactoryUpdate(
            self.near_call(&OwnerCall::FactoryUpdate)
                .args_borsh(Raw(bytes)),
        )
    }

    pub fn factory_set_wnear_address(
        &self,
        address: impl Into<Address>,
    ) -> CallFactorySetWNearAddress<'_> {
        let bytes = address.into().0;
        CallFactorySetWNearAddress(
            self.near_call(&OwnerCall::FactorySetWNEARAddress)
                .args_borsh(aurora_engine_types::types::Address::new(bytes.into())),
        )
    }

    pub fn stage_upgrade(&self, bytes: Vec<u8>) -> CallStageUpgrade<'_> {
        CallStageUpgrade(
            self.near_call(&OwnerCall::StageUpgrade)
                .args_borsh(Raw(bytes)),
        )
    }

    pub fn deploy_upgrade(&self) -> CallDeployUpgrade<'_> {
        CallDeployUpgrade(self.near_call(&OwnerCall::DeployUpgrade))
    }

    pub fn resume_precompiles(&self, paused_mask: u32) -> CallResumePrecompiles<'_> {
        let args = PausePrecompilesCallArgs { paused_mask };
        CallResumePrecompiles(
            self.near_call(&OwnerCall::ResumePrecompiles)
                .args_borsh(args),
        )
    }

    pub fn state_migration(&self) -> CallStateMigration<'_> {
        CallStateMigration(self.near_call(&OwnerCall::StateMigration))
    }

    pub async fn version(&self) -> Result<ViewResultDetails<String>> {
        ViewResultDetails::try_from(self.near_view(&View::Version, vec![]).await?)
    }

    pub async fn owner(&self) -> Result<ViewResultDetails<AccountId>> {
        ViewResultDetails::try_from(self.near_view(&View::Owner, vec![]).await?)
    }

    pub async fn bridge_prover(&self) -> Result<ViewResultDetails<AccountId>> {
        ViewResultDetails::try_from(self.near_view(&View::BridgeProver, vec![]).await?)
    }

    pub async fn chain_id(&self) -> Result<ViewResultDetails<String>> {
        ViewResultDetails::try_from(self.near_view(&View::ChainId, vec![]).await?)
    }

    pub async fn upgrade_index(&self) -> Result<ViewResultDetails<u64>> {
        Ok(ViewResultDetails::from(
            self.near_view(&View::UpgradeIndex, vec![]).await?,
        ))
    }

    pub async fn paused_precompiles(&self) -> Result<ViewResultDetails<u32>> {
        Ok(ViewResultDetails::from(
            self.near_view(&View::PausedPrecompiles, vec![]).await?,
        ))
    }

    pub async fn block_hash(&self, block_height: u64) -> Result<ViewResultDetails<H256>> {
        let args = block_height.try_to_vec()?;
        Ok(ViewResultDetails::from(
            self.near_view(&View::BlockHash, args).await?,
        ))
    }

    #[cfg(not(feature = "ethabi"))]
    pub async fn code<A: Into<Address>>(&self, address: A) -> Result<ViewResultDetails<Vec<u8>>> {
        let address = address.into();
        Ok(ViewResultDetails::from(
            self.near_view(&View::Code, address.as_bytes().to_vec())
                .await?,
        ))
    }

    #[cfg(feature = "ethabi")]
    pub async fn code(
        &self,
        types: &[ParamType],
        address: Address,
    ) -> Result<ViewResultDetails<Vec<Token>>> {
        let address = aurora_engine_types::types::Address::new(address);
        ViewResultDetails::decode(
            types,
            self.near_view(&View::Code, address.try_to_vec()?).await?,
        )
    }

    pub async fn balance<A: Into<Address>>(&self, address: A) -> Result<ViewResultDetails<u128>> {
        Ok(ViewResultDetails::from_u256(
            self.near_view(&View::Balance, address.into().0.to_vec())
                .await?,
        ))
    }

    pub async fn nonce<A: Into<Address>>(&self, address: A) -> Result<ViewResultDetails<u128>> {
        Ok(ViewResultDetails::from_u256(
            self.near_view(&View::Nonce, address.into().0.to_vec())
                .await?,
        ))
    }

    pub async fn storage<A: Into<Address>, K: Into<H256>>(
        &self,
        address: A,
        key: K,
    ) -> Result<ViewResultDetails<H256>> {
        let args = GetStorageAtArgs {
            address: aurora_engine_types::types::Address::new(address.into()),
            key: key.into().0,
        };
        Ok(ViewResultDetails::from(
            self.near_view(&View::Storage, args.try_to_vec()?).await?,
        ))
    }

    pub async fn view<A: Into<Address>, V: Into<U256>>(
        &self,
        sender: A,
        address: A,
        amount: V,
        input: Vec<u8>,
    ) -> Result<ViewResultDetails<TransactionStatus>> {
        let mut buf = [0u8; 32];
        amount.into().to_big_endian(&mut buf);
        let args = ViewCallArgs {
            sender: aurora_engine_types::types::Address::new(sender.into()),
            address: aurora_engine_types::types::Address::new(address.into()),
            amount: buf,
            input,
        };
        ViewResultDetails::try_from(self.near_view(&View::Evm, args.try_to_vec()?).await?)
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
    }

    pub async fn ft_metadata(&self) -> Result<ViewResultDetails<FungibleTokenMetadata>> {
        ViewResultDetails::try_from(self.near_view(&View::FtMetadata, vec![]).await?)
    }

    pub async fn eth_balance_of<A: Into<Address>>(
        &self,
        address: A,
    ) -> Result<ViewResultDetails<U256>> {
        Ok(ViewResultDetails::from(
            self.near_view(&View::BalanceOfEth, address.into().0.to_vec())
                .await?,
        ))
    }

    pub async fn eth_total_supply(&self) -> Result<ViewResultDetails<U256>> {
        ViewResultDetails::try_from_json(self.near_view(&View::EthTotalSupply, vec![]).await?)
    }

    pub async fn storage_balance_of<A: AsRef<str>>(
        &self,
        account_id: A,
    ) -> Result<ViewResultDetails<StorageBalance>> {
        let account = AccountId::from_str(account_id.as_ref()).unwrap();
        let args = borsh::to_vec(&account).unwrap();
        ViewResultDetails::try_from(self.near_view(&View::StorageBalanceOf, args).await?)
    }

    pub async fn erc20_from_nep141(
        &self,
        nep141_account_id: AccountId,
    ) -> Result<ViewResultDetails<AccountId>> {
        ViewResultDetails::try_from(
            self.near_view(&View::Erc20FromNep141, nep141_account_id.try_to_vec()?)
                .await?,
        )
    }

    pub async fn nep141_from_erc20(
        &self,
        erc20_account_id: AccountId,
    ) -> Result<ViewResultDetails<AccountId>> {
        ViewResultDetails::try_from(
            self.near_view(&View::Nep141FromErc20, erc20_account_id.try_to_vec()?)
                .await?,
        )
    }

    pub async fn paused_flags(&self) -> Result<ViewResultDetails<u8>> {
        Ok(ViewResultDetails::from(
            self.near_view(&View::PausedFlags, Vec::new()).await?,
        ))
    }
}

/// A collection of sources where you can get the contract.
pub enum ContractSource<P: AsRef<Path>> {
    /// A path to the file containing the contract binary.
    Dir(P),
    /// Source the contract binary from NEAR testnet.
    Testnet,
    /// Source the contract binary from NEAR mainnet.
    Mainnet,
}

impl<P: AsRef<Path>> From<P> for ContractSource<P> {
    fn from(path: P) -> Self {
        ContractSource::Dir(path)
    }
}

pub struct InitConfig {
    /// The owner ID of the contract.
    pub owner_id: AccountId,
    /// The prover ID of the contract.
    pub prover_id: AccountId,
    /// The Ethereum chain ID.
    pub chain_id: U256,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            owner_id: AccountId::from_str("owner.test.near").expect("Account ID somehow failed"),
            prover_id: AccountId::from_str("prover.test.near").expect("Prover ID somehow failed"),
            chain_id: U256::from(1313161556),
        }
    }
}

// TODO: Put all parameters per input, not as the struct args!
// TODO: implement a signer when a method is called, return a signer with
// TODO: builder
// information required about the transaction to be made. Then give the option
// to sign with another key, or with some default. Preferably, run `transact`.

/// A wrapper over workspaces' `Contract` type which provides ease of use when interacting with
/// the Aurora EVM contract.
///
/// ## Constructors
///
/// The contract *must* first be deployed before you can interact with it else it will return
/// errors that the contract is invalid. This can be done by invoking the `deploy_and_init` method
/// which will deploy and call `new` on the contract automatically using the provided parameters
/// from the `DeployConfig`.
///
/// If it already has been deployed and exists on the network, it is fine to construct the
/// `EvmContract` directly using the `new` method. Though the naming may be a bit confusing, this
/// follows Rust language conventions of creating a new `EvmContract` struct.
///
/// The final constructor is `from_secret_key` which will create a new `EvmContract` struct directly
/// from the provided `AccountId` and `SecretKey`. This also does not deploy the contract onto the
/// network.
///
/// ## Deployment
///
/// It should not be expected that the underlying methods return the same results as the wrapped
/// contract. Instead, it returns only the EVM result, logs, and other related meta data such as
/// data usage on both ETH and NEAR.
///
/// This type *can not* implement `Default` as the deployment may not already exist. Likewise, the
/// library does not provide a ready built EVM binary to be deployed. This must be specified.
#[derive(Debug, Clone)]
pub struct EvmContract {
    contract: EvmAccount<Private>,
}

impl AsRef<EvmAccount<Private>> for EvmContract {
    fn as_ref(&self) -> &EvmAccount<Private> {
        &self.contract
    }
}

impl AsMut<EvmAccount<Private>> for EvmContract {
    fn as_mut(&mut self) -> &mut EvmAccount<Private> {
        &mut self.contract
    }
}

impl Borrow<EvmAccount<Private>> for EvmContract {
    fn borrow(&self) -> &EvmAccount<Private> {
        &self.contract
    }
}

impl BorrowMut<EvmAccount<Private>> for EvmContract {
    fn borrow_mut(&mut self) -> &mut EvmAccount<Private> {
        &mut self.contract
    }
}

// TODO have another PhantomData (maybe) which will note if its the public, owner, etc.
impl From<Contract> for EvmContract {
    fn from(contract: Contract) -> Self {
        EvmContract {
            contract: EvmAccount::with_self(contract),
        }
    }
}

impl EvmContract {
    pub fn new<C: Into<Contract>>(contract: C) -> EvmContract {
        EvmContract {
            contract: EvmAccount::with_self(contract.into()),
        }
    }

    pub async fn deploy_and_init(
        account: Account,
        init_config: InitConfig,
        wasm: Vec<u8>,
    ) -> Result<EvmContract> {
        let contract = Self::deploy(account, wasm).await?;
        contract.init(init_config).await?;
        Ok(contract)
    }

    pub async fn deploy(account: Account, wasm: Vec<u8>) -> Result<EvmContract> {
        let contract = account.deploy(&wasm).await?.into_result()?;
        Ok(EvmContract {
            contract: EvmAccount::with_self(contract),
        })
    }

    pub async fn init(&self, init_config: InitConfig) -> Result<()> {
        use crate::types::input::NewInput;

        let chain_id = {
            let mut buf = [0u8; 32];
            init_config.chain_id.to_big_endian(&mut buf);
            buf
        };
        let new_args = NewInput {
            chain_id,
            // TODO: https://github.com/aurora-is-near/aurora-engine/issues/604, unwrap is safe here
            owner_id: init_config.owner_id,
            bridge_prover_id: init_config.prover_id,
            upgrade_delay_blocks: 1,
        };
        self.contract
            .near_call("new")
            .args_borsh(new_args)
            .transact()
            .await?
            .into_result()?;

        Ok(())
    }

    pub fn from_secret_key<N: Network + 'static, D: AsRef<str>>(
        id: D,
        sk: SecretKey,
        worker: &Worker<N>,
    ) -> Result<EvmContract> {
        let account_id = AccountId::from_str(id.as_ref())?;
        let contract = Contract::from_secret_key(account_id, sk, worker);
        Ok(EvmContract {
            contract: EvmAccount::with_self(contract),
        })
    }

    pub fn as_account(&self) -> &EvmAccount<Private> {
        &self.contract
    }
}

mod private {
    pub trait Sealed {}
}
