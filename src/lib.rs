mod contract;
pub mod error;
pub(crate) mod impls;
pub(crate) mod operations;

pub use contract::{DeployConfig, EvmAccount, EvmContract};
pub use operations::EvmCallTransaction;

pub use aurora_engine::proof::Proof;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;
