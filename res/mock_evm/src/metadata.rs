use std::ascii;

use aurora_workspace_types::{output::TransactionStatus, AccountId, Raw};
use near_sdk::{borsh, near_bindgen};

use crate::{MockEvmContract, MockEvmContractExt};

#[near_bindgen]
impl MockEvmContract {
    pub fn get_version(&self) -> String {
        "v1".to_string()
    }

    pub fn ft_total_eth_supply_on_aurora(&self) -> String {
        "0".into()
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
}
