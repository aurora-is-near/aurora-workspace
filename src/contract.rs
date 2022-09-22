use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;
use crate::impls::AuroraReturns;
use crate::{EvmCallTransaction, Result};
use aurora_engine::fungible_token::FungibleTokenMetadata;
use aurora_engine::parameters::{
    BalanceOfCallArgs, BalanceOfEthCallArgs, DeployErc20TokenArgs, FunctionCallArgsV2,
    GetStorageAtArgs, InitCallArgs, IsUsedProofCallArgs, NEP141FtOnTransferArgs, NewCallArgs,
    SubmitResult, TransactionStatus, ViewCallArgs, 
};
use aurora_engine::proof::Proof;
use aurora_engine::xcc::AddressVersionUpdateArgs;
use aurora_engine_types::parameters::WithdrawCallArgs;
use aurora_engine_types::types::{NEP141Wei, Wei};
use borsh::BorshSerialize;
use ethereum_types::{Address, U256};
use lazy_static::lazy_static;
use std::path::{Path};
use std::str::FromStr;
use workspaces::network::{Betanet, Mainnet, Sandbox, Testnet};
use workspaces::operations::{CallTransaction};
use workspaces::result::{ExecutionFinalResult, ViewResultDetails};
use workspaces::types::{KeyType, SecretKey};
use workspaces::{Account, AccountId, Contract, Network, Worker};
use crate::operations::{Call, Function};

pub const AURORA_LOCAL_CHAIN_ID: u64 = 1313161556;
pub const AURORA_ACCOUNT_ID: &str = "aurora.test.near";
pub const OWNER_ACCOUNT_ID: &str = "owner.test.near";
pub const PROVER_ACCOUNT_ID: &str = "prover.test.near";
pub const INITIAL_BALANCE: u128 = 100_000_000_000_000_000_000_000_000; // 100 NEAR
pub const EVM_CUSTODIAN_ADDRESS: &str = "096DE9C2B8A5B8c22cEe3289B101f6960d68E51E";

lazy_static! {
    static ref DEFAULT_AURORA_ACCOUNT_ID: AccountId =
        AccountId::from_str("aurora.test.near").unwrap();
    static ref DEFAULT_OWNER_ACCOUNT_ID: AccountId =
        AccountId::from_str("owner.test.near").unwrap();
    static ref DEFAULT_PROVER_ACCOUNT_ID: AccountId =
        AccountId::from_str("prover.test.near").unwrap();
}

// TODO: use me
pub struct CallResult<T> {
    result: T,
    logs: Vec<String>,
}

// TODO(engine): Self should be able to call owner functions.
pub trait EvmSelf: EvmUser {}

pub trait EvmOwner: EvmUser {}

pub trait EvmProver: EvmUser {}

pub trait EvmUser: private::Sealed {}

pub trait EvmTester {}

pub(crate) struct EvmAccount<U: EvmUser> {
    account: Account,
    contract_id: AccountId,
    phantom: PhantomData<U>
}

impl<U: EvmOwner> EvmAccount<U> {
    pub fn with_owner(account: Account, contract_id: AccountId) -> EvmAccount<U> {
        Self {
            account,
            contract_id,
            phantom: Default::default()
        }
    }
}

impl<U: EvmProver> EvmAccount<U> {
    pub fn with_prover(account: Account, contract_id: AccountId) -> EvmAccount<U> {
        Self {
            account,
            contract_id,
            phantom: Default::default()
        }
    }
}

impl<U: EvmUser> EvmAccount<U> {
    pub async fn new(account: Account, contract_id: AccountId) -> EvmAccount<U> {
        Self {
            account,
            contract_id,
            phantom: Default::default()
        }
    }

    async fn call<'a, F: AsRef<str>>(&'a self, function: &'a Function) -> EvmCallTransaction {
        let transaction = self.account.call(&self.contract_id, function.as_ref());
        EvmCallTransaction::call(function, transaction)
    }

    async fn view(&self, function: &str, args: Vec<u8>) -> Result<ViewResultDetails> {
        Ok(self.account.view(&self.contract_id, function.as_ref(), args).await?)
    }

    pub fn id(&self) -> &AccountId {
        self.account.id()
    }
}

pub(crate) struct EvmProverAccount {
    account: Account,
    contract_id: AccountId,
}

impl EvmProverAccount {
    pub fn new(account: Account, contract_id: AccountId) -> EvmProverAccount {
        Self {
            account,
            contract_id,
        }
    }

    pub fn call<'a, 'b>(&'a self, function: &'b str) -> CallTransaction<'a, 'b> {
        self.account.call(self.account.id(), function)
    }

    pub async fn deposit(&self, proof: Proof) -> Result<ExecutionFinalResult> {
        Ok(self
            .call("deposit")
            .args_borsh(proof)
            .max_gas()
            .transact()
            .await?)
    }

    pub fn id(&self) -> &AccountId {
        self.account.id()
    }
}

/// A collection of sources where you can get the contract.
pub enum EvmContractSource<P: AsRef<Path>> {
    /// A path to the file containing the contract binary.
    Dir(P),
    /// Source the contract binary from NEAR testnet.
    Testnet,
    /// Source the contract binary from NEAR mainnet.
    Mainnet,
}

pub struct EthProverConfig {
    pub account_id: AccountId,
    pub evm_custodian_address: String,
}

pub struct DeployConfig {
    /// The owner ID of the contract.
    pub owner_id: AccountId,
    /// The prover ID of the contract.
    pub prover_id: AccountId,
    /// The optional configuration for the Ethereum prover (bridge).
    pub eth_prover_config: Option<EthProverConfig>,
    /// The Ethereum chain ID.
    pub chain_id: U256,
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
pub struct EvmContract<N: Network + 'static> {
    contract: Contract,
    phantom: PhantomData<N>
}

impl<N: Network + 'static> AsRef<Contract> for EvmContract<N> {
    fn as_ref(&self) -> &Contract {
        &self.contract
    }
}

impl<N: Network + 'static> AsMut<Contract> for EvmContract<N> {
    fn as_mut(&mut self) -> &mut Contract {
        &mut self.contract
    }
}

impl<N: Network + 'static> Borrow<Contract> for EvmContract<N> {
    fn borrow(&self) -> &Contract {
        &self.contract
    }
}

impl<N: Network + 'static> BorrowMut<Contract> for EvmContract<N> {
    fn borrow_mut(&mut self) -> &mut Contract {
        &mut self.contract
    }
}

// TODO have another PhantomData (maybe) which will note if its the public, owner, etc.
impl<N: Network + 'static> From<Contract> for EvmContract<N> {
    fn from(contract: Contract) -> Self {
        EvmContract {
            contract,
            phantom: Default::default(),
        }
    }
}

impl EvmContract<Sandbox> {
    pub async fn deploy_and_init<P: AsRef<Path>>(
        account: Account,
        deploy_config: DeployConfig,
        source: EvmContractSource<P>,
        worker: &Worker<Sandbox>,
    ) -> Result<EvmContract<Sandbox>> {
        let contract = match source {
            EvmContractSource::Dir(path) => {
                let wasm = std::fs::read(path)?;
                account.deploy(&wasm).await?.into_result()?
            }
            EvmContractSource::Testnet => {
                let testnet_worker = workspaces::testnet().await?;
                let account_id = account.id();
                worker.import_contract(account_id, &testnet_worker).transact().await?
            }
            EvmContractSource::Mainnet => {
                let mainnet_worker = workspaces::mainnet().await?;
                let account_id = account.id();
                worker.import_contract(account_id, &mainnet_worker).transact().await?
            }
        };

        Self::deploy_and_init_inner(contract, deploy_config).await
    }
}

impl EvmContract<Betanet> {
    pub async fn deploy_and_init<P: AsRef<Path>>(
        account: Account,
        deploy_config: DeployConfig,
        path: P,
    ) -> Result<EvmContract<Betanet>> {
        let contract = deploy_contract(path, account).await?;
        Self::deploy_and_init_inner(contract, deploy_config).await
    }
}

impl EvmContract<Testnet> {
    pub async fn deploy_and_init<P: AsRef<Path>>(
        account: Account,
        deploy_config: DeployConfig,
        path: P,
    ) -> Result<EvmContract<Testnet>> {
        let contract = deploy_contract(path, account).await?;
        Self::deploy_and_init_inner(contract, deploy_config).await
    }
}

impl EvmContract<Mainnet> {
    pub async fn deploy_and_init<P: AsRef<Path>>(
        account: Account,
        deploy_config: DeployConfig,
        path: P,
    ) -> Result<EvmContract<Mainnet>> {
        let contract = deploy_contract(path, account).await?;
        Self::deploy_and_init_inner(contract, deploy_config).await
    }
}

impl<N: Network + 'static> EvmContract<N> {
    pub async fn new<C: Into<Contract>>(contract: C) -> EvmContract<N> {
        EvmContract {
            contract: contract.into(),
            phantom: Default::default(),
        }
    }

    async fn deploy_and_init_inner(contract: Contract, deploy_config: DeployConfig) -> Result<EvmContract<N>> {
        let new_args = NewCallArgs {
            chain_id: aurora_engine_types::types::u256_to_arr(&deploy_config.chain_id),
            // TODO: https://github.com/aurora-is-near/aurora-engine/issues/604, unwrap is safe here
            owner_id: aurora_engine_types::account_id::AccountId::from_str(
                deploy_config.owner_id.as_str(),
            )
                .unwrap(),
            bridge_prover_id: aurora_engine_types::account_id::AccountId::from_str(
                deploy_config.prover_id.as_str(),
            )
                .unwrap(),
            upgrade_delay_blocks: 1,
        };
        contract
            .call("new")
            .args_borsh(new_args)
            .transact()
            .await?
            .into_result()?;

        if let Some(eth_prover_config) = deploy_config.eth_prover_config {
            let new_eth_connector_args = InitCallArgs {
                prover_account: aurora_engine_types::account_id::AccountId::from_str(
                    eth_prover_config.account_id.as_str(),
                )
                    .unwrap(),
                eth_custodian_address: eth_prover_config.evm_custodian_address,
                metadata: FungibleTokenMetadata::default(),
            };
            contract
                .call("new_eth_connector")
                .args_borsh(new_eth_connector_args)
                .transact()
                .await?
                .into_result()?;
        }

        Ok(EvmContract{
            contract,
            phantom: Default::default(),
        })
    }
    
    pub async fn from_secret_key<D: AsRef<str>>(id: D, sk: SecretKey, worker: &Worker<N>) -> Result<EvmContract<N>> {
        let account_id = AccountId::from_str(id.as_ref())?;
        Ok(EvmContract{
            contract: Contract::from_secret_key(account_id, sk, worker),
            phantom: Default::default(),
        })
    }

    // async fn near_call<'a, 'b>(&self, function: &'b CallFunction, args: Vec<u8>) -> EvmCallTransaction<'_, 'b> {
    //     let transaction = self.contract.call(function.as_ref()).args(args);
    //     EvmCallTransaction::new(function, transaction)
    // }
    //
    // // TODO: improve view to be like call.
    // async fn near_view<F: AsRef<str>>(&self, function: F, args: Vec<u8>) -> ViewResultDetails {
    //     self.contract.view(function.as_ref(), args).await.unwrap() // TODO: fix error handling here
    // }
    //
    // // TODO: improve with making the args vec on method above
    // async fn near_call_borsh<U: borsh::BorshSerialize>(
    //     &self,
    //     function: &str,
    //     args: U,
    // ) -> Result<ExecutionFinalResult> {
    //     let execution_result = self
    //         .contract
    //         .call(function)
    //         .max_gas()
    //         .args_borsh(args)
    //         .transact()
    //         .await?;
    //
    //     Ok(execution_result)
    // }
    //
    // async fn near_call_json<U: serde::Serialize>(
    //     &self,
    //     function: &str,
    //     args: U,
    // ) -> Result<ExecutionFinalResult> {
    //     let execution_result = self
    //         .contract
    //         .call(function)
    //         .max_gas()
    //         .args_json(args)
    //         .transact()
    //         .await?;
    //
    //     Ok(execution_result)
    // }

    // TODO: Test which ensures that the version was properly bumped.
    // TODO: replace with view
    // pub async fn get_version(&self) -> String {
    //     self.call_near("get_version", vec![])
    //         .await
    //         .unwrap()
    //         .to_string()
    // }

    // // TODO: replace with view
    // pub async fn get_owner(&self) -> Result<AccountId> {
    //     self.near_call("get_owner", vec![])
    //         .await?
    //         .try_to_account_id()
    // }
    //
    // // TODO: replace with view
    // pub async fn get_bridge_prover(&self) -> Result<AccountId> {
    //     self.near_call("get_bridge_prover", vec![])
    //         .await?
    //         .try_to_account_id()
    // }
    //
    // // TODO: replace with view
    // pub async fn get_chain_id(&self) -> Result<U256> {
    //     self.near_call("get_chain_id", vec![]).await?.try_to_u256()
    // }
    //
    // // TODO: replace with view
    // pub async fn get_upgrade_index(&self) -> Result<u64> {
    //     self.near_call("get_upgrade_index", vec![])
    //         .await?
    //         .try_to_u64()
    // }
    //
    // pub async fn stage_upgrade(&self) {
    //     todo!()
    // }
    //
    // pub async fn deploy_upgrade(&self) {
    //     todo!()
    // }
    //
    // pub async fn stage_migration(&self) {
    //     todo!()
    // }
    //
    // pub async fn deploy_code(&self, code: Vec<u8>) -> Result<SubmitResult> {
    //     self.near_call("deploy_code", code)
    //         .await?
    //         .try_to_evm_result()
    // }
    //
    // pub async fn call(&self, args: FunctionCallArgsV2) -> Result<SubmitResult> {
    //     self.near_call_borsh("call", args)
    //         .await?
    //         .try_to_evm_result()
    // }
    //
    // pub async fn submit(&self, args: Vec<u8>) -> Result<SubmitResult> {
    //     self.near_call("submit", args).await?.try_to_evm_result()
    // }
    //
    // pub async fn register_relayer(&self, relayer_id: AccountId) -> Result<()> {
    //     // We don't need the result here as there is none.
    //     let _ = self
    //         .near_call("register_relayer", relayer_id.as_bytes().to_vec())
    //         .await?;
    //     Ok(())
    // }
    //
    // pub fn factory_update(&self, _router_bytes: Vec<u8>) -> Result<()> {
    //     todo!()
    // }
    //
    // pub async fn factory_update_address_version(
    //     &self,
    //     _update_args: AddressVersionUpdateArgs,
    // ) -> Result<()> {
    //     todo!()
    // }
    //
    // pub async fn factory_set_wnear_address(&self, wnear_address: Address) -> Result<()> {
    //     let _ = self
    //         .near_call("set_wnear_address", wnear_address.as_bytes().to_vec())
    //         .await?;
    //     Ok(())
    // }
    //
    // pub async fn ft_on_transfer(&self, ft_on_transfer_args: NEP141FtOnTransferArgs) -> Result<()> {
    //     let _ = self
    //         .near_call_borsh("ft_on_transfer", ft_on_transfer_args)
    //         .await?;
    //     Ok(())
    // }
    //
    // pub async fn deploy_erc20_token(&self, deploy_args: DeployErc20TokenArgs) -> Result<Address> {
    //     self
    //         .near_call_borsh("deploy_erc20_token", deploy_args)
    //         .await?
    //         .try_to_address()
    // }
    //
    // // TODO: check if this is necessary, considering that it is a private
    // // function. Else, could improve it with adding a removal of
    // // io.assert_private_call in refund_on_error engine function for tests only.
    // // pub async fn refund_on_error(&self)
    //
    // // TODO: all view returns should be the correct types.
    // pub async fn view(&self, view_args: ViewCallArgs) -> Result<TransactionStatus> {
    //     self.near_call_borsh("view", view_args)
    //         .await?
    //         .try_to_transaction_status()
    // }
    //
    // pub async fn get_block_hash(&self, block_height: u64) -> ViewResultDetails {
    //     self.view_near("get_block_hash", block_height.try_to_vec().unwrap()) // TODO: fix unwrap
    //         .await
    // }
    //
    // pub async fn get_code(&self, address: Address) -> Vec<u8> {
    //     self.view_near("get_code", address.as_bytes().to_vec())
    //         .await
    //         .result
    // }
    //
    // pub async fn get_balance(&self, address: Address) -> Wei {
    //     Wei::new(U256::from_big_endian(
    //         &self
    //             .view_near("get_balance", address.as_bytes().to_vec())
    //             .await
    //             .result,
    //     ))
    // }
    //
    // pub async fn get_nonce(&self, address: Address) -> U256 {
    //     U256::from_big_endian(
    //         &self
    //             .view_near("get_nonce", address.as_bytes().to_vec())
    //             .await
    //             .result,
    //     )
    // }
    //
    // // TODO return
    // pub async fn get_storage_at(&self, args: GetStorageAtArgs) -> ViewResultDetails {
    //     self.view_near("get_storage_at", args.try_to_vec().unwrap())
    //         .await // TODO: fix unwrap
    // }
    //
    // // pub async fn new_eth_connector
    //
    // // pub async fn set_eth_connector_contract_data()
    //
    // pub async fn withdraw(&self, args: WithdrawCallArgs) -> Result<ExecutionFinalResult> {
    //     self.near_call_borsh("withdraw", args).await
    // }
    //
    // pub async fn deposit(&self, proof: Proof) -> Result<ExecutionFinalResult> {
    //     self.near_call("deposit", proof.try_to_vec()?).await
    // }
    //
    // // pub async fn finish_deposit
    //
    // pub async fn is_used_proof(&self, args: IsUsedProofCallArgs) -> Result<bool> {
    //     self.near_call_borsh("is_used_proof", args)
    //         .await?
    //         .try_to_bool()
    // }
    //
    // // TODO ... rest
    //
    // pub async fn mint_account(&self, address: Address, nonce: u64, balance: u64) -> Result<()> {
    //     let args = (
    //         aurora_engine_types::types::Address::new(address),
    //         nonce,
    //         balance,
    //     );
    //     let _ = self.near_call_borsh("mint_account", args).await;
    //     Ok(())
    // }
    //
    // pub async fn ft_balance_of(&self, account_id: AccountId) -> Result<NEP141Wei> {
    //     // TODO: https://github.com/aurora-is-near/aurora-engine/issues/604, unwrap is safe here
    //     let args = BalanceOfCallArgs {
    //         account_id: aurora_engine_types::account_id::AccountId::new(account_id.as_str())
    //             .unwrap(),
    //     };
    //     let call = self
    //         .near_call_json("ft_balance_of", args)
    //         .await?
    //         .into_result()?;
    //     println!("ft_balance_of: {call:#?}");
    //     let res_string: String = call.json()?;
    //     let value = NEP141Wei::new(u128::from_str(&res_string)?);
    //     Ok(value)
    // }
    //
    // pub async fn ft_balance_of_eth(&self, address: Address) -> Result<Wei> {
    //     let args = BalanceOfEthCallArgs {
    //         address: aurora_engine_types::types::Address::new(address),
    //     };
    //     let res: Wei = self
    //         .near_call_json("ft_balance_of_eth", args)
    //         .await?
    //         .json()?;
    //     Ok(res)
    // }
    //
    // pub async fn ft_total_supply(&self) -> Result<NEP141Wei> {
    //     let res: NEP141Wei = self.near_call("ft_total_supply", vec![]).await?.json()?;
    //     Ok(res)
    // }
    //
    // pub async fn ft_total_eth_supply_on_aurora(&self) -> Result<Wei> {
    //     let res: Wei = self
    //         .near_call("ft_total_eth_supply_on_aurora", vec![])
    //         .await?
    //         .json()?;
    //     Ok(res)
    // }
}

async fn deploy_contract<P: AsRef<Path>>(path: P, account: Account) -> Result<Contract> {
    let wasm = std::fs::read(path)?;
    Ok(account.deploy(&wasm).await?.into_result()?)
}

mod private {
    pub trait Sealed {}
}
