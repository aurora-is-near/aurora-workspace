use crate::impls::AuroraReturns;
use crate::Result;
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
use workspaces::network::{Sandbox};
use workspaces::operations::CallTransaction;
use workspaces::result::{ExecutionFinalResult, ViewResultDetails};
use workspaces::types::SecretKey;
use workspaces::{Account, AccountId, Contract, Worker};

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

pub(crate) struct EvmOwner {
    account: Account,
    contract_id: AccountId,
}

impl EvmOwner {
    pub fn new(account: Account, contract_id: AccountId) -> EvmOwner {
        Self {
            account,
            contract_id,
        }
    }

    pub fn call<'a, 'b>(&'a self, function: &'b str) -> CallTransaction<'a, 'b> {
        self.account.call(self.account.id(), function)
    }

    pub fn id(&self) -> &AccountId {
        self.account.id()
    }
}

pub(crate) struct EvmProver {
    account: Account,
    contract_id: AccountId,
}

impl EvmProver {
    pub fn new(account: Account, contract_id: AccountId) -> EvmProver {
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

pub enum EvmContractSource<P: AsRef<Path>> {
    Dir(P),
    Testnet(AccountId),
    Mainnet(AccountId),
}

pub struct EthProverConfig {
    pub account_id: AccountId,
    pub evm_custodian_address: String,
}

pub struct DeployConfig<P: AsRef<Path>> {
    pub owner_id: AccountId,
    pub prover_id: AccountId,
    pub source: EvmContractSource<P>,
    pub eth_prover_config: Option<EthProverConfig>,
    pub chain_id: U256,
}

pub struct ContractConfig {
    pub contract_id: AccountId,
    pub contract_secret_key: SecretKey,
}

pub struct ImportConfig {
    pub account_id: AccountId,
}

// TODO: Put all parameters per input, not as the struct args!
// TODO: implement a signer when a method is called, return a signer with
// TODO: builder
// information required about the transaction to be made. Then give the option
// to sign with another key, or with some default. Preferably, run `transact`.
pub struct EvmContract(Contract);

impl EvmContract {
    pub async fn new(worker: &Worker<Sandbox>, contract_config: ContractConfig) -> EvmContract {
        let contract = Contract::from_secret_key(
            contract_config.contract_id,
            contract_config.contract_secret_key,
            worker,
        );
        EvmContract(contract)
    }

    pub async fn create_tla_and_deploy<P: AsRef<Path>>(
        worker: &Worker<Sandbox>,
        contract_config: ContractConfig,
        deploy_config: DeployConfig<P>,
    ) -> Result<Self> {
        let contract = match deploy_config.source {
            EvmContractSource::Dir(path) => {
                let wasm = std::fs::read(path)?;
                worker
                    .create_tla_and_deploy(
                        contract_config.contract_id,
                        contract_config.contract_secret_key,
                        &wasm,
                    )
                    .await?
                    .into_result()?
            }
            EvmContractSource::Testnet(account_id) => {
                let testnet_worker = workspaces::testnet().await?;
                worker.import_contract(&account_id, &testnet_worker).transact().await?
            }
            EvmContractSource::Mainnet(account_id) => {
                let mainnet_worker = workspaces::mainnet().await?;
                worker.import_contract(&account_id, &mainnet_worker).transact().await?
            }
        };

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

        Ok(EvmContract(contract))
    }

    pub async fn call_near(&self, function: &str, args: Vec<u8>) -> Result<ExecutionFinalResult> {
        let execution_result = self
            .0
            .call(function)
            .args(args)
            .max_gas()
            .transact()
            .await?;

        Ok(execution_result)
    }

    // TODO: improve view to be like call.
    pub async fn view_near(&self, function: &str, args: Vec<u8>) -> ViewResultDetails {
        self.0.view(function, args).await.unwrap() // TODO: fix error handling here
    }

    // TODO: improve with making the args vec on method above
    pub async fn call_near_borsh<U: borsh::BorshSerialize>(
        &self,
        function: &str,
        args: U,
    ) -> Result<ExecutionFinalResult> {
        let execution_result = self
            .0
            .call(function)
            .max_gas()
            .args_borsh(args)
            .transact()
            .await?;

        Ok(execution_result)
    }

    pub async fn call_near_json<U: serde::Serialize>(
        &self,
        function: &str,
        args: U,
    ) -> Result<ExecutionFinalResult> {
        let execution_result = self
            .0
            .call(function)
            .max_gas()
            .args_json(args)
            .transact()
            .await?;

        Ok(execution_result)
    }

    // TODO: Test which ensures that the version was properly bumped.
    // TODO: replace with view
    // pub async fn get_version(&self) -> String {
    //     self.call_near("get_version", vec![])
    //         .await
    //         .unwrap()
    //         .to_string()
    // }

    // TODO: replace with view
    pub async fn get_owner(&self) -> Result<AccountId> {
        self.call_near("get_owner", vec![])
            .await?
            .try_to_account_id()
    }

    // TODO: replace with view
    pub async fn get_bridge_prover(&self) -> Result<AccountId> {
        self.call_near("get_bridge_prover", vec![])
            .await?
            .try_to_account_id()
    }

    // TODO: replace with view
    pub async fn get_chain_id(&self) -> Result<U256> {
        self.call_near("get_chain_id", vec![]).await?.try_to_u256()
    }

    // TODO: replace with view
    pub async fn get_upgrade_index(&self) -> Result<u64> {
        self.call_near("get_upgrade_index", vec![])
            .await?
            .try_to_u64()
    }

    pub async fn stage_upgrade(&self) {
        todo!()
    }

    pub async fn deploy_upgrade(&self) {
        todo!()
    }

    pub async fn stage_migration(&self) {
        todo!()
    }

    pub async fn deploy_code(&self, code: Vec<u8>) -> Result<SubmitResult> {
        self.call_near("deploy_code", code)
            .await?
            .try_to_evm_result()
    }

    pub async fn call(&self, args: FunctionCallArgsV2) -> Result<SubmitResult> {
        self.call_near_borsh("call", args)
            .await?
            .try_to_evm_result()
    }

    pub async fn submit(&self, args: Vec<u8>) -> Result<SubmitResult> {
        self.call_near("submit", args).await?.try_to_evm_result()
    }

    pub async fn register_relayer(&self, relayer_id: AccountId) -> Result<()> {
        // We don't need the result here as there is none.
        let _ = self
            .call_near("register_relayer", relayer_id.as_bytes().to_vec())
            .await?;
        Ok(())
    }

    pub fn factory_update(&self, _router_bytes: Vec<u8>) -> Result<()> {
        todo!()
    }

    pub async fn factory_update_address_version(
        &self,
        _update_args: AddressVersionUpdateArgs,
    ) -> Result<()> {
        todo!()
    }

    pub async fn factory_set_wnear_address(&self, wnear_address: Address) -> Result<()> {
        let _ = self
            .call_near("set_wnear_address", wnear_address.as_bytes().to_vec())
            .await?;
        Ok(())
    }

    pub async fn ft_on_transfer(&self, ft_on_transfer_args: NEP141FtOnTransferArgs) -> Result<()> {
        let _ = self
            .call_near_borsh("ft_on_transfer", ft_on_transfer_args)
            .await?;
        Ok(())
    }

    pub async fn deploy_erc20_token(&self, deploy_args: DeployErc20TokenArgs) -> Result<Address> {
        self
            .call_near_borsh("deploy_erc20_token", deploy_args)
            .await?
            .try_to_address()
    }

    // TODO: check if this is necessary, considering that it is a private
    // function. Else, could improve it with adding a removal of
    // io.assert_private_call in refund_on_error engine function for tests only.
    // pub async fn refund_on_error(&self)

    // TODO: all view returns should be the correct types.
    pub async fn view(&self, view_args: ViewCallArgs) -> Result<TransactionStatus> {
        self.call_near_borsh("view", view_args)
            .await?
            .try_to_transaction_status()
    }

    pub async fn get_block_hash(&self, block_height: u64) -> ViewResultDetails {
        self.view_near("get_block_hash", block_height.try_to_vec().unwrap()) // TODO: fix unwrap
            .await
    }

    pub async fn get_code(&self, address: Address) -> Vec<u8> {
        self.view_near("get_code", address.as_bytes().to_vec())
            .await
            .result
    }

    pub async fn get_balance(&self, address: Address) -> Wei {
        Wei::new(U256::from_big_endian(
            &self
                .view_near("get_balance", address.as_bytes().to_vec())
                .await
                .result,
        ))
    }

    pub async fn get_nonce(&self, address: Address) -> U256 {
        U256::from_big_endian(
            &self
                .view_near("get_nonce", address.as_bytes().to_vec())
                .await
                .result,
        )
    }

    // TODO return
    pub async fn get_storage_at(&self, args: GetStorageAtArgs) -> ViewResultDetails {
        self.view_near("get_storage_at", args.try_to_vec().unwrap())
            .await // TODO: fix unwrap
    }

    // pub async fn new_eth_connector

    // pub async fn set_eth_connector_contract_data()

    pub async fn withdraw(&self, args: WithdrawCallArgs) -> Result<ExecutionFinalResult> {
        self.call_near_borsh("withdraw", args).await
    }

    pub async fn deposit(&self, proof: Proof) -> Result<ExecutionFinalResult> {
        self.call_near("deposit", proof.try_to_vec()?).await
    }

    // pub async fn finish_deposit

    pub async fn is_used_proof(&self, args: IsUsedProofCallArgs) -> Result<bool> {
        self.call_near_borsh("is_used_proof", args)
            .await?
            .try_to_bool()
    }

    // TODO ... rest

    pub async fn mint_account(&self, address: Address, nonce: u64, balance: u64) -> Result<()> {
        let args = (
            aurora_engine_types::types::Address::new(address),
            nonce,
            balance,
        );
        let _ = self.call_near_borsh("mint_account", args).await;
        Ok(())
    }

    pub async fn ft_balance_of(&self, account_id: AccountId) -> Result<NEP141Wei> {
        // TODO: https://github.com/aurora-is-near/aurora-engine/issues/604, unwrap is safe here
        let args = BalanceOfCallArgs {
            account_id: aurora_engine_types::account_id::AccountId::new(account_id.as_str())
                .unwrap(),
        };
        let call = self
            .call_near_json("ft_balance_of", args)
            .await?
            .into_result()?;
        println!("ft_balance_of: {call:#?}");
        let res_string: String = call.json()?;
        let value = NEP141Wei::new(u128::from_str(&res_string)?);
        Ok(value)
    }

    pub async fn ft_balance_of_eth(&self, address: Address) -> Result<Wei> {
        let args = BalanceOfEthCallArgs {
            address: aurora_engine_types::types::Address::new(address),
        };
        let res: Wei = self
            .call_near_json("ft_balance_of_eth", args)
            .await?
            .json()?;
        Ok(res)
    }

    pub async fn ft_total_supply(&self) -> Result<NEP141Wei> {
        let res: NEP141Wei = self.call_near("ft_total_supply", vec![]).await?.json()?;
        Ok(res)
    }

    pub async fn ft_total_eth_supply_on_aurora(&self) -> Result<Wei> {
        let res: Wei = self
            .call_near("ft_total_eth_supply_on_aurora", vec![])
            .await?
            .json()?;
        Ok(res)
    }
}
