#![allow(dead_code)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

pub const STATE_KEY: &[u8; 5] = b"STATE";

pub enum VersionPrefix {
    V1 = 0x7,
}

impl From<VersionPrefix> for u8 {
    fn from(v: VersionPrefix) -> Self {
        match v {
            VersionPrefix::V1 => 0x7,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, BorshSerialize, BorshDeserialize)]
pub enum KeyPrefix {
    Config = 0x0,
    Nonce = 0x1,
    Balance = 0x2,
    Code = 0x3,
    Storage = 0x4,
    RelayerEvmAddressMap = 0x5,
    EthConnector = 0x6,
    Generation = 0x7,
    Nep141Erc20Map = 0x8,
    Erc20Nep141Map = 0x9,
    CrossContractCall = 0xa,
}

impl From<KeyPrefix> for u8 {
    fn from(k: KeyPrefix) -> Self {
        use KeyPrefix::*;
        match k {
            Config => 0x0,
            Nonce => 0x1,
            Balance => 0x2,
            Code => 0x3,
            Storage => 0x4,
            RelayerEvmAddressMap => 0x5,
            EthConnector => 0x6,
            Generation => 0x7,
            Nep141Erc20Map => 0x8,
            Erc20Nep141Map => 0x9,
            CrossContractCall => 0xa,
        }
    }
}

pub fn bytes_to_key(prefix: KeyPrefix, bytes: &[u8]) -> Vec<u8> {
    [&[u8::from(VersionPrefix::V1)], &[u8::from(prefix)], bytes].concat()
}
