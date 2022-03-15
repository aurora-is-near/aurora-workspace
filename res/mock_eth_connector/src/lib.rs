use crate::out::SubmitResult;
use aurora_workspace_types::input::{CallInput, DeployErc20Input, NewInput};
use aurora_workspace_types::output::{Log, TransactionStatus};
use aurora_workspace_types::{AccountId, Address, Raw, H256};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

mod fungible_token;
mod metadata;
mod out;
mod storage;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MockEthConnectorContract {
    pub chain_id: [u8; 32],
    pub owner_id: AccountId,
    pub bridge_prover_id: AccountId,
    pub upgrade_delay_blocks: u64,
}

#[near_bindgen]
impl MockEthConnectorContract {}
