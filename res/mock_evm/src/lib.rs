use aurora_workspace_types::input::{CallInput, DeployErc20Input, SetEthConnectorInput};
use aurora_workspace_types::output::{Log, TransactionStatus};
use aurora_workspace_types::{AccountId, Address, H256, Raw};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};
use crate::out::SubmitResult;

mod metadata;
mod storage;
mod out;

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
    pub eth_connector: Option<SetEthConnectorInput>,
}

#[near_bindgen]
impl MockEvmContract {
    #[init]
    pub fn new(
        chain_id: [u8; 32],
        owner_id: AccountId,
        bridge_prover_id: AccountId,
        upgrade_delay_blocks: u64,
    ) -> MockEvmContract {
        MockEvmContract {
            chain_id,
            owner_id,
            bridge_prover_id,
            upgrade_delay_blocks,
            eth_connector: None,
        }
    }

    pub fn new_eth_connector(&mut self, #[serializer(borsh)] input: SetEthConnectorInput) {
        self.eth_connector = Some(input);
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
}
