use crate::operation::{
    CallCall, CallDeployCode, CallDeployErc20Token, CallDeployUpgrade, CallDeposit,
    CallFactorySetWNearAddress, CallFactoryUpdate, CallFactoryUpdateAddressVersion,
    CallFtOnTransfer, CallFtTransfer, CallFtTransferCall, CallPausePrecompiles, CallRefundOnError,
    CallRegisterRelayer, CallResumePrecompiles, CallSetEthConnectorContractData, CallStageUpgrade,
    CallStateMigration, CallStorageDeposit, CallStorageUnregister, CallStorageWithdraw, CallSubmit,
    CallWithdraw, ViewBalance, ViewBlockHash, ViewBridgeProver, ViewChainId, ViewCode,
    ViewErc20FromNep141, ViewFtBalanceOf, ViewFtBalanceOfEth, ViewFtMetadata,
    ViewFtTotalEthSupplyOnAurora, ViewFtTotalSupply, ViewIsUsedProof, ViewNep141FromErc20,
    ViewNonce, ViewOwner, ViewPausedFlags, ViewPausedPrecompiles, ViewStorageAt,
    ViewStorageBalanceOf, ViewUpgradeIndex, ViewVersion, ViewView,
};
use crate::types::Account;
use aurora_engine::fungible_token::FungibleTokenMetadata;
use aurora_engine_types::types::{Address, Balance};
use aurora_engine_types::U256;
use aurora_workspace_types::input::ProofInput;
use aurora_workspace_types::{AccountId, H256};
use aurora_workspace_utils::{Contract, ContractId};
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

    pub fn new_from_contract(contract_id: AccountId, account: Account) -> Self {
        Self {
            contract: Contract::new(contract_id, account),
        }
    }
}

impl ContractId for EngineContract {
    fn as_contract(&self) -> &Contract {
        &self.contract
    }

    fn id(&self) -> &AccountId {
        self.contract.id()
    }
}

/// Callable functions
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

    pub fn get_version(&self) -> ViewVersion {
        ViewVersion::view(&self.contract)
    }

    pub fn get_owner(&self) -> ViewOwner {
        ViewOwner::view(&self.contract)
    }

    pub fn get_bridge_prover(&self) -> ViewBridgeProver {
        ViewBridgeProver::view(&self.contract)
    }

    pub fn get_chain_id(&self) -> ViewChainId {
        ViewChainId::view(&self.contract)
    }

    pub fn get_upgrade_index(&self) -> ViewUpgradeIndex {
        ViewUpgradeIndex::view(&self.contract)
    }

    pub fn get_paused_precompiles(&self) -> ViewPausedPrecompiles {
        ViewPausedPrecompiles::view(&self.contract)
    }

    pub fn get_block_hash(&self, block_height: u64) -> ViewBlockHash {
        ViewBlockHash::view(&self.contract).args_borsh(block_height)
    }

    pub fn get_code(&self, address: Address) -> ViewCode {
        ViewCode::view(&self.contract).args_borsh(address)
    }

    pub fn get_balance(&self, address: Address) -> ViewBalance {
        ViewBalance::view(&self.contract).args_borsh(address)
    }

    pub fn get_nonce(&self, address: Address) -> ViewNonce {
        ViewNonce::view(&self.contract).args_borsh(address)
    }

    pub fn get_storage_at(&self, address: Address, key: H256) -> ViewStorageAt {
        let raw_key = <H256 as Into<aurora_engine_types::types::RawH256>>::into(key);
        ViewStorageAt::view(&self.contract).args_borsh((address, raw_key))
    }

    pub fn get_view(
        &self,
        sender: Address,
        address: Address,
        amount: U256,
        input: Vec<u8>,
    ) -> ViewView {
        let mut raw_amount = [0u8; 32];
        amount.to_big_endian(&mut raw_amount);
        ViewView::view(&self.contract).args_borsh((sender, address, raw_amount, input))
    }

    pub fn is_used_proof(&self, proof: ProofInput) -> ViewIsUsedProof {
        ViewIsUsedProof::view(&self.contract).args_borsh(proof)
    }

    pub fn ft_total_eth_supply_on_aurora(&self) -> ViewFtTotalEthSupplyOnAurora {
        ViewFtTotalEthSupplyOnAurora::view(&self.contract)
    }

    pub fn ft_balance_of_eth(&self, address: Address) -> ViewFtBalanceOfEth {
        ViewFtBalanceOfEth::view(&self.contract).args_borsh(address)
    }

    pub fn get_erc20_from_nep141(&self, account: AccountId) -> ViewErc20FromNep141 {
        ViewErc20FromNep141::view(&self.contract).args_borsh(account)
    }

    pub fn get_nep141_from_erc20(&self, address: Address) -> ViewNep141FromErc20 {
        ViewNep141FromErc20::view(&self.contract).args_borsh(address)
    }

    pub fn get_paused_flags(&self) -> ViewPausedFlags {
        ViewPausedFlags::view(&self.contract)
    }
}
