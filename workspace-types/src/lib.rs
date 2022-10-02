pub mod input;
pub mod output;
pub mod error;

pub use near_account_id::AccountId;

pub use ethereum_types::Address;
pub use ethereum_types::{H256, U256};

pub type Result<T, E = error::Error> = std::result::Result<T, E>;
