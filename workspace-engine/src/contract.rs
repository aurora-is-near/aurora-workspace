use crate::operation::{
    CallCall, CallDeployCode, CallDeployErc20Token, CallDeployUpgrade, CallDeposit,
    CallFactorySetWNearAddress, CallFactoryUpdate, CallFactoryUpdateAddressVersion,
    CallFtOnTransfer, CallFtTransfer, CallFtTransferCall, CallPausePrecompiles, CallRefundOnError,
    CallRegisterRelayer, CallResumePrecompiles, CallSetEthConnectorContractData, CallStageUpgrade,
    CallStateMigration, CallStorageDeposit, CallStorageUnregister, CallStorageWithdraw, CallSubmit,
    CallWithdraw, ViewFtBalanceOf, ViewFtMetadata, ViewFtTotalSupply, ViewStorageBalanceOf,
};
use crate::types::Account;
use aurora_engine::fungible_token::FungibleTokenMetadata;
use aurora_engine_types::types::{Address, Balance};
use aurora_engine_types::U256;
use aurora_workspace_types::input::ProofInput;
use aurora_workspace_types::AccountId;
use aurora_workspace_utils::Contract;
#[cfg(feature = "ethabi")]
use ethabi::{ParamType, Token};
use near_sdk::json_types::U128;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct EngineContract {
    contract: Contract,
}

impl EngineContract {
    pub fn new(contract: Contract) -> Self {
        Self { contract }
    }

    pub fn as_contract(&self) -> &Contract {
        &self.contract
    }

    pub fn id(&self) -> &AccountId {
        self.contract.id()
    }

    pub fn new_from_contract(contract_id: AccountId, account: Account) -> Self {
        Self {
            contract: Contract::new(contract_id, account),
        }
    }
}

/// Call functions
impl EngineContract {
    pub fn ft_transfer(
        &self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    ) -> CallFtTransfer {
        CallFtTransfer::call(&self.contract)
            .args_json(json!({ "receiver_id": receiver_id, "amount": amount, "memo": memo }))
    }

    pub fn ft_transfer_call(
        &self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> CallFtTransferCall {
        CallFtTransferCall::call(&self.contract).args_json(json!({
           "receiver_id": receiver_id,
           "amount": amount,
           "memo": memo,
           "msg": msg,
        }))
    }

    pub fn storage_deposit(
        &self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> CallStorageDeposit {
        CallStorageDeposit::call(&self.contract)
            .args_json(json!({ "account_id": account_id, "registration_only": registration_only}))
    }

    pub fn storage_withdraw(&self, amount: Option<U128>) -> CallStorageWithdraw {
        CallStorageWithdraw::call(&self.contract).args_json(json!({ "amount": amount }))
    }

    pub fn storage_unregister(&self, force: Option<bool>) -> CallStorageUnregister {
        CallStorageUnregister::call(&self.contract).args_json(serde_json::json!({ "force": force }))
    }

    pub fn withdraw(&self, recipient_address: Address, amount: Balance) -> CallWithdraw {
        CallWithdraw::call(&self.contract).args_borsh((recipient_address, amount))
    }

    pub fn deposit(&self, raw_proof: ProofInput) -> CallDeposit {
        CallDeposit::call(&self.contract).args_borsh(raw_proof)
    }

    pub fn set_eth_connector_contract_data(
        &self,
        prover_account: AccountId,
        eth_custodian_address: Address,
        metadata: FungibleTokenMetadata,
    ) -> CallSetEthConnectorContractData {
        CallSetEthConnectorContractData::call(&self.contract).args_borsh((
            prover_account,
            eth_custodian_address,
            metadata,
        ))
    }

    pub fn factory_update_address_version(
        &self,
        address: Address,
        version: u32,
    ) -> CallFactoryUpdateAddressVersion {
        CallFactoryUpdateAddressVersion::call(&self.contract).args_borsh((address, version))
    }

    pub fn refund_on_error(
        &self,
        recipient_address: Address,
        erc20_address: Option<Address>,
        amount: U256,
    ) -> CallRefundOnError {
        let mut raw_amount: aurora_engine_types::types::RawU256 = Default::default();
        amount.to_big_endian(&mut raw_amount);
        CallRefundOnError::call(&self.contract).args_borsh((
            recipient_address,
            erc20_address,
            raw_amount,
        ))
    }

    pub fn deploy_code(&self, code: Vec<u8>) -> CallDeployCode {
        CallDeployCode::call(&self.contract).args_borsh(code)
    }

    pub fn deploy_erc20_token(&self, account_id: AccountId) -> CallDeployErc20Token {
        CallDeployErc20Token::call(&self.contract).args_borsh(account_id)
    }

    pub fn call(&self, contract: Address, amount: U256, input: Vec<u8>) -> CallCall {
        let mut raw_amount: aurora_engine_types::types::RawU256 = Default::default();
        amount.to_big_endian(&mut raw_amount);
        CallCall::call(&self.contract).args_borsh((contract, raw_amount, input))
    }

    pub fn submit(&self, input: Vec<u8>) -> CallSubmit {
        CallSubmit::call(&self.contract).args_borsh(input)
    }

    pub fn register_relayer(&self, address: Address) -> CallRegisterRelayer {
        CallRegisterRelayer::call(&self.contract).args_borsh(address)
    }

    pub fn ft_on_transfer(
        &self,
        sender_id: AccountId,
        amount: U128,
        message: String,
    ) -> CallFtOnTransfer {
        CallFtOnTransfer::call(&self.contract).args_json(json!({
            "sender_id": sender_id,
            "amount": amount,
            "message": message
        }))
    }

    pub fn factory_update(&self, bytes: Vec<u8>) -> CallFactoryUpdate {
        CallFactoryUpdate::call(&self.contract).args_borsh(bytes)
    }

    pub fn factory_set_wnear_address(&self, address: Address) -> CallFactorySetWNearAddress {
        CallFactorySetWNearAddress::call(&self.contract).args_borsh(address)
    }

    pub fn stage_upgrade(&self, bytes: Vec<u8>) -> CallStageUpgrade {
        CallStageUpgrade::call(&self.contract).args_borsh(bytes)
    }

    pub fn deploy_upgrade(&self) -> CallDeployUpgrade {
        CallDeployUpgrade::call(&self.contract)
    }

    pub fn pause_precompiles(&self, paused_mask: u32) -> CallPausePrecompiles {
        CallPausePrecompiles::call(&self.contract).args_borsh(paused_mask)
    }

    pub fn resume_precompiles(&self, paused_mask: u32) -> CallResumePrecompiles {
        CallResumePrecompiles::call(&self.contract).args_borsh(paused_mask)
    }

    pub fn state_migration(&self) -> CallStateMigration {
        CallStateMigration::call(&self.contract)
    }
}

/// View functions
impl EngineContract {
    pub fn ft_total_supply(&self) -> ViewFtTotalSupply {
        ViewFtTotalSupply::view(&self.contract)
    }

    pub fn ft_balance_of(&self, account_id: AccountId) -> ViewFtBalanceOf {
        ViewFtBalanceOf::view(&self.contract).args_json(json!((account_id,)))
    }

    pub fn storage_balance_of(&self, account_id: AccountId) -> ViewStorageBalanceOf {
        ViewStorageBalanceOf::view(&self.contract).args_json(json!({ "account_id": account_id }))
    }

    pub fn ft_metadata(&self) -> ViewFtMetadata {
        ViewFtMetadata::view(&self.contract)
    }
}

/*
impl EngineContract {
    pub async fn version(&self) -> anyhow::Result<ViewResultDetails<String>> {
        ViewResultDetails::try_from(self.near_view(&View::Version, vec![]).await?)
    }

    pub async fn owner(&self) -> anyhow::Result<ViewResultDetails<AccountId>> {
        ViewResultDetails::try_from(self.near_view(&View::Owner, vec![]).await?)
    }

    pub async fn bridge_prover(&self) -> anyhow::Result<ViewResultDetails<AccountId>> {
        ViewResultDetails::try_from(self.near_view(&View::BridgeProver, vec![]).await?)
    }

    pub async fn chain_id(&self) -> anyhow::Result<ViewResultDetails<String>> {
        ViewResultDetails::try_from(self.near_view(&View::ChainId, vec![]).await?)
    }

    pub async fn upgrade_index(&self) -> anyhow::Result<ViewResultDetails<u64>> {
        Ok(ViewResultDetails::from(
            self.near_view(&View::UpgradeIndex, vec![]).await?,
        ))
    }

    pub async fn paused_precompiles(&self) -> anyhow::Result<ViewResultDetails<u32>> {
        Ok(ViewResultDetails::from(
            self.near_view(&View::PausedPrecompiles, vec![]).await?,
        ))
    }

    pub async fn block_hash(&self, block_height: u64) -> anyhow::Result<ViewResultDetails<H256>> {
        let args = block_height.try_to_vec()?;
        Ok(ViewResultDetails::from(
            self.near_view(&View::BlockHash, args).await?,
        ))
    }

    #[cfg(not(feature = "ethabi"))]
    pub async fn code<A: Into<Address>>(
        &self,
        address: A,
    ) -> anyhow::Result<ViewResultDetails<Vec<u8>>> {
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
    ) -> anyhow::Result<ViewResultDetails<Vec<Token>>> {
        let address = aurora_engine_types::types::Address::new(address);
        ViewResultDetails::decode(
            types,
            self.near_view(&View::Code, address.try_to_vec()?).await?,
        )
    }

    pub async fn balance<A: Into<Address>>(
        &self,
        address: A,
    ) -> anyhow::Result<ViewResultDetails<u128>> {
        Ok(ViewResultDetails::from_u256(
            self.near_view(&View::Balance, address.into().0.to_vec())
                .await?,
        ))
    }

    pub async fn nonce<A: Into<Address>>(
        &self,
        address: A,
    ) -> anyhow::Result<ViewResultDetails<u128>> {
        Ok(ViewResultDetails::from_u256(
            self.near_view(&View::Nonce, address.into().0.to_vec())
                .await?,
        ))
    }

    pub async fn storage<A: Into<Address>, K: Into<H256>>(
        &self,
        address: A,
        key: K,
    ) -> anyhow::Result<ViewResultDetails<H256>> {
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
    ) -> anyhow::Result<ViewResultDetails<TransactionStatus>> {
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

    pub async fn is_proof_used(
        &self,
        proof: ProofInput,
    ) -> anyhow::Result<ViewResultDetails<bool>> {
        let args = IsUsedProofCallArgs { proof };
        ViewResultDetails::try_from(
            self.near_view(&View::IsProofUsed, args.try_to_vec()?)
                .await?,
        )
    }

    pub async fn ft_total_supply(&self) -> anyhow::Result<ViewResultDetails<u128>> {
        ViewResultDetails::try_from(self.near_view(&View::FtTotalSupply, vec![]).await?)
    }

    pub async fn eth_balance_of<A: Into<Address>>(
        &self,
        address: A,
    ) -> anyhow::Result<ViewResultDetails<U256>> {
        Ok(ViewResultDetails::from(
            self.near_view(&View::BalanceOfEth, address.into().0.to_vec())
                .await?,
        ))
    }

    pub async fn eth_total_supply(&self) -> anyhow::Result<ViewResultDetails<U256>> {
        ViewResultDetails::try_from_json(self.near_view(&View::EthTotalSupply, vec![]).await?)
    }

    pub async fn erc20_from_nep141(
        &self,
        nep141_account_id: AccountId,
    ) -> anyhow::Result<ViewResultDetails<AccountId>> {
        ViewResultDetails::try_from(
            self.near_view(&View::Erc20FromNep141, nep141_account_id.try_to_vec()?)
                .await?,
        )
    }

    pub async fn nep141_from_erc20(
        &self,
        erc20_account_id: AccountId,
    ) -> anyhow::Result<ViewResultDetails<AccountId>> {
        ViewResultDetails::try_from(
            self.near_view(&View::Nep141FromErc20, erc20_account_id.try_to_vec()?)
                .await?,
        )
    }

    pub async fn paused_flags(&self) -> anyhow::Result<ViewResultDetails<u8>> {
        Ok(ViewResultDetails::from(
            self.near_view(&View::PausedFlags, Vec::new()).await?,
        ))
    }
}
*/
