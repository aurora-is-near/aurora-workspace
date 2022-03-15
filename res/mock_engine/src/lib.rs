use crate::submit::SubmitResult;
use aurora_workspace_types::input::{CallInput, DeployErc20Input, NewInput};
use aurora_workspace_types::output::TransactionStatus;
use aurora_workspace_types::{AccountId, Raw};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

mod fungible_token;
mod storage;
mod submit;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MockEngineContract {
    pub chain_id: [u8; 32],
    pub owner_id: AccountId,
    pub bridge_prover_id: AccountId,
    pub upgrade_delay_blocks: u64,
}

#[near_bindgen]
impl MockEngineContract {
    #[init]
    pub fn new(#[serializer(borsh)] input: NewInput) -> Self {
        Self {
            chain_id: input.chain_id,
            owner_id: input.owner_id,
            bridge_prover_id: input.bridge_prover_id,
            upgrade_delay_blocks: input.upgrade_delay_blocks,
        }
    }

    #[result_serializer(borsh)]
    pub fn deploy_code(&mut self, #[serializer(borsh)] _input: Raw) -> SubmitResult {
        SubmitResult::dummy_submit_result()
    }

    #[result_serializer(borsh)]
    pub fn deploy_erc20_token(&mut self, #[serializer(borsh)] _input: DeployErc20Input) -> Raw {
        Raw(vec![1u8; 20])
    }

    #[result_serializer(borsh)]
    pub fn call(&mut self, #[serializer(borsh)] _input: CallInput) -> SubmitResult {
        SubmitResult::dummy_submit_result()
    }

    #[result_serializer(borsh)]
    pub fn submit(&mut self, #[serializer(borsh)] _input: Raw) -> SubmitResult {
        SubmitResult::dummy_submit_result()
    }

    #[result_serializer(borsh)]
    pub fn register_relayer(&mut self, #[serializer(borsh)] input: Raw) {
        assert_eq!(input.0.len(), 20);
    }

    //
    // SELF CALL METHODS
    //

    pub fn set_eth_connector_contract_data(&mut self, #[serializer(borsh)] _input: Raw) {}

    pub fn set_paused_flags(&mut self, #[serializer(borsh)] _input: Raw) {}

    //
    // CALLBACK HANDLER METHODS
    //

    #[result_serializer(borsh)]
    pub fn factory_update_address_version(&mut self, #[serializer(borsh)] _input: Raw) -> u8 {
        0
    }

    #[result_serializer(borsh)]
    pub fn refund_on_error(&mut self, #[serializer(borsh)] _input: Raw) -> u8 {
        0
    }

    pub fn get_version(&self) -> String {
        "v1".to_string()
    }

    #[result_serializer(borsh)]
    pub fn get_owner(&self) -> AccountId {
        "owner.test.near".parse().unwrap()
    }

    pub fn get_chain_id(&self) -> String {
        1313161556.to_string()
    }

    #[result_serializer(borsh)]
    pub fn get_view(&self, #[serializer(borsh)] _input: Raw) -> TransactionStatus {
        TransactionStatus::Succeed(vec![])
    }

    #[result_serializer(borsh)]
    pub fn get_code(&self, #[serializer(borsh)] _input: Raw) -> Raw {
        // `(string,bool,string) (spiral,true,quasar)`
        Raw(hex::decode(b"00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000000673706972616c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000067175617361720000000000000000000000000000000000000000000000000000").unwrap())
    }

    #[result_serializer(borsh)]
    pub fn get_storage_at(&self, #[serializer(borsh)] _input: Raw) -> [u8; 32] {
        [1; 32]
    }

    #[result_serializer(borsh)]
    pub fn get_erc20_from_nep141(&self, #[serializer(borsh)] _input: Raw) -> AccountId {
        "erc20.test.near".parse().unwrap()
    }

    #[result_serializer(borsh)]
    pub fn get_nep141_from_erc20(&self, #[serializer(borsh)] _input: Raw) -> AccountId {
        "nep141.test.near".parse().unwrap()
    }

    #[result_serializer(borsh)]
    pub fn get_paused_flags(&self, #[serializer(borsh)] _input: Raw) -> u8 {
        0
    }

    #[result_serializer(borsh)]
    pub fn get_bridge_prover(&self) -> AccountId {
        self.bridge_prover_id.clone()
    }

    #[result_serializer(borsh)]
    pub fn get_upgrade_index(&self) -> u64 {
        self.upgrade_delay_blocks
    }

    #[result_serializer(borsh)]
    pub fn get_paused_precompiles(&self) -> u32 {
        0u32
    }

    #[result_serializer(borsh)]
    pub fn get_block_hash(&self, #[serializer(borsh)] _index: u64) -> [u8; 32] {
        [0u8; 32]
    }

    #[result_serializer(borsh)]
    pub fn get_balance(&self, #[serializer(borsh)] _address: [u8; 20]) -> [u64; 4] {
        [0u64; 4]
    }

    #[result_serializer(borsh)]
    pub fn get_nonce(&self, #[serializer(borsh)] _address: [u8; 20]) -> [u64; 4] {
        [0u64; 4]
    }
}
