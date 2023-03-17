use crate::out::SubmitResult;
use aurora_workspace_types::input::{CallInput, DeployErc20Input, NewInput};
use aurora_workspace_types::output::{Log, TransactionStatus};
use aurora_workspace_types::{AccountId, Address, Raw, H256};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

pub mod ft;
mod metadata;
mod out;
mod storage;

fn dummy_submit_result() -> SubmitResult {
    let log = Log::new(
        Address::from([1u8; 20]),
        vec![H256::from([2u8; 32])],
        vec![3u8; 10],
    );
    SubmitResult::new(TransactionStatus::Succeed(vec![0]), 100_000, vec![log])
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MockEvmContract {
    pub chain_id: [u8; 32],
    pub owner_id: AccountId,
    pub bridge_prover_id: AccountId,
    pub upgrade_delay_blocks: u64,
}

#[near_bindgen]
impl MockEvmContract {
    #[init]
    pub fn new(#[serializer(borsh)] input: NewInput) -> MockEvmContract {
        MockEvmContract {
            chain_id: input.chain_id,
            owner_id: input.owner_id,
            bridge_prover_id: input.bridge_prover_id,
            upgrade_delay_blocks: input.upgrade_delay_blocks,
        }
    }

    #[result_serializer(borsh)]
    pub fn deploy_code(&mut self, #[serializer(borsh)] _input: Raw) -> SubmitResult {
        dummy_submit_result()
    }

    #[result_serializer(borsh)]
    pub fn deploy_erc20_token(&mut self, #[serializer(borsh)] _input: DeployErc20Input) -> Raw {
        Raw(vec![1u8; 20])
    }

    #[result_serializer(borsh)]
    pub fn call(&mut self, #[serializer(borsh)] _input: CallInput) -> SubmitResult {
        dummy_submit_result()
    }

    #[result_serializer(borsh)]
    pub fn submit(&mut self, #[serializer(borsh)] _input: Raw) -> SubmitResult {
        dummy_submit_result()
    }

    #[result_serializer(borsh)]
    pub fn register_relayer(&mut self, #[serializer(borsh)] input: Raw) {
        assert_eq!(input.0.len(), 20);
    }

    //
    // OWNER CALL METHODS
    //

    #[allow(unused_variables)]
    pub fn factory_update(&mut self, #[serializer(borsh)] bytes: Raw) {}

    #[allow(unused_variables)]
    pub fn factory_set_wnear_address(
        &mut self,
        #[serializer(borsh)] input: aurora_engine_types::types::Address,
    ) {
    }

    #[allow(unused_variables)]
    pub fn deploy_upgrade(&mut self) {}

    #[allow(unused_variables)]
    pub fn resume_precompiles(&mut self, #[serializer(borsh)] paused_mask: u32) {}

    #[allow(unused_variables)]
    pub fn stage_upgrade(&mut self, #[serializer(borsh)] input: Raw) {}

    //
    // AUTHORIZED CALL METHODS
    //

    #[allow(unused_variables)]
    pub fn pause_precompiles(&mut self, #[serializer(borsh)] paused_mask: u32) {}

    //
    // SELF CALL METHODS
    //

    pub fn set_eth_connector_contract_data(&mut self, #[serializer(borsh)] _input: Raw) {}

    #[allow(unused_variables)]
    pub fn set_paused_flags(&mut self, #[serializer(borsh)] paused_mask: u32) {}

    //
    // CALLBACK HANDLER METHODS
    //

    #[result_serializer(borsh)]
    pub fn factory_update_address_version(&mut self, #[serializer(borsh)] _input: Raw) {}

    #[result_serializer(borsh)]
    pub fn refund_on_error(&mut self, #[serializer(borsh)] _input: Raw) {}
}
