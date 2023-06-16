pub mod error;
pub mod input;

pub use aurora_engine_types::{types::Address, H160, H256, U256};
pub use near_account_id::AccountId;
pub use near_account_id::ParseAccountError;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::io::{self, Write};

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Raw(pub Vec<u8>);

impl BorshSerialize for Raw {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.0)
    }
}

impl BorshDeserialize for Raw {
    fn deserialize(bytes: &mut &[u8]) -> io::Result<Self> {
        let res = bytes.to_vec();
        *bytes = &[];
        Ok(Self(res))
    }
}

pub mod output {
    pub use aurora_engine_types::parameters::engine::TransactionStatus;
}
