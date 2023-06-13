use aurora_engine_types::account_id::AccountId;
use aurora_engine_types::parameters::connector::InitCallArgs;
use aurora_engine_types::parameters::engine::{
    CallArgs, DeployErc20TokenArgs, NewCallArgs, SubmitResult, TransactionStatus,
};
use aurora_engine_types::parameters::RefundCallArgs;
use aurora_engine_types::types::Address;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, serde, PanicOnDefault};

mod fungible_token;
// mod storage;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MockEngineContract {
    pub chain_id: [u8; 32],
    pub owner_id: AccountId,
    pub upgrade_delay_blocks: u64,
}

#[near_bindgen]
impl MockEngineContract {
    #[init]
    pub fn new(#[serializer(borsh)] input: NewCallArgs) -> Self {
        let input = match input {
            NewCallArgs::V1(_) => panic!("Wrong version of the init args"),
            NewCallArgs::V2(args) => args,
        };

        Self {
            chain_id: input.chain_id,
            owner_id: input.owner_id,
            upgrade_delay_blocks: input.upgrade_delay_blocks,
        }
    }

    #[result_serializer(borsh)]
    pub fn deploy_code(&mut self, #[serializer(borsh)] _input: Vec<u8>) -> SubmitResult {
        dummy_submit_result()
    }

    #[result_serializer(borsh)]
    pub fn deploy_erc20_token(
        &mut self,
        #[serializer(borsh)] _input: DeployErc20TokenArgs,
    ) -> Address {
        Address::from_array([1; 20])
    }

    #[result_serializer(borsh)]
    pub fn call(&mut self, #[serializer(borsh)] _input: CallArgs) -> SubmitResult {
        dummy_submit_result()
    }

    #[result_serializer(borsh)]
    pub fn submit(&mut self, #[serializer(borsh)] _input: Vec<u8>) -> SubmitResult {
        dummy_submit_result()
    }

    #[result_serializer(borsh)]
    pub fn register_relayer(&mut self, #[serializer(borsh)] input: [u8; 20]) {
        let _ = Address::from_array(input);
    }

    //
    // AUTHORIZED CALL METHODS
    //

    #[allow(unused_variables)]
    pub fn pause_precompiles(&mut self, #[serializer(borsh)] paused_mask: u32) {}

    //
    // SELF CALL METHODS
    //

    pub fn set_eth_connector_contract_data(&mut self, #[serializer(borsh)] _input: InitCallArgs) {}

    pub fn set_paused_flags(&mut self, #[serializer(borsh)] _input: Vec<u8>) {}

    //
    // CALLBACK HANDLER METHODS
    //

    #[result_serializer(borsh)]
    pub fn factory_update_address_version(
        &mut self,
        #[serializer(borsh)] _input: AddressVersionUpdateArgs,
    ) {
    }

    #[result_serializer(borsh)]
    pub fn refund_on_error(&mut self, #[serializer(borsh)] _input: RefundCallArgs) {}

    #[result_serializer(borsh)]
    pub fn get_version(&self) -> String {
        "2.9.1".to_string()
    }

    #[result_serializer(borsh)]
    pub fn get_owner(&self) -> AccountId {
        "owner.test.near".parse().unwrap()
    }

    pub fn get_chain_id(&self) -> String {
        1313161556.to_string()
    }

    #[result_serializer(borsh)]
    pub fn get_view(&self, #[serializer(borsh)] _input: Vec<u8>) -> TransactionStatus {
        TransactionStatus::Succeed(vec![])
    }

    #[result_serializer(borsh)]
    pub fn get_code(&self, #[serializer(borsh)] _address: Address) -> Vec<u8> {
        hex::decode(b"00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000000673706972616c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000067175617361720000000000000000000000000000000000000000000000000000").unwrap()
    }

    #[result_serializer(borsh)]
    pub fn get_storage_at(&self, #[serializer(borsh)] _input: Vec<u8>) -> [u8; 32] {
        [1; 32]
    }

    #[result_serializer(borsh)]
    pub fn get_erc20_from_nep141(&self, #[serializer(borsh)] _input: Vec<u8>) -> AccountId {
        "erc20.test.near".parse().unwrap()
    }

    #[result_serializer(borsh)]
    pub fn get_nep141_from_erc20(&self, #[serializer(borsh)] _input: Vec<u8>) -> AccountId {
        "nep141.test.near".parse().unwrap()
    }

    #[result_serializer(borsh)]
    pub fn get_paused_flags(&self, #[serializer(borsh)] _input: Vec<u8>) -> u8 {
        0
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
    pub fn get_balance(&self, #[serializer(borsh)] _address: Address) -> [u64; 4] {
        [0u64; 4]
    }

    #[result_serializer(borsh)]
    pub fn get_nonce(&self, #[serializer(borsh)] _address: Address) -> [u64; 4] {
        [0u64; 4]
    }

    //
    // OWNER CALL METHODS
    //

    #[allow(unused_variables)]
    pub fn factory_update(&mut self, #[serializer(borsh)] bytes: Vec<u8>) {}

    #[allow(unused_variables)]
    pub fn factory_set_wnear_address(&mut self, #[serializer(borsh)] _input: Address) {}

    #[allow(unused_variables)]
    pub fn deploy_upgrade(&mut self) {}

    pub fn state_migration(&mut self) {}

    #[allow(unused_variables)]
    pub fn resume_precompiles(&mut self, #[serializer(borsh)] paused_mask: u32) {}

    #[allow(unused_variables)]
    pub fn stage_upgrade(&mut self, #[serializer(borsh)] input: Vec<u8>) {}
}

fn dummy_submit_result() -> SubmitResult {
    SubmitResult::new(TransactionStatus::Succeed(vec![]), 0, vec![])
}

#[derive(Debug, Clone, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
pub struct AddressVersionUpdateArgs {
    pub address: Address,
    pub version: u32,
}
