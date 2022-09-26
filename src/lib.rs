mod contract;
pub mod error;
pub(crate) mod operation;
pub(crate) mod result;

pub use contract::{DeployConfig, EvmAccount, EvmContract};
pub use operation::EvmCallTransaction;

pub use aurora_engine::proof::Proof;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;
