mod contract;
pub mod error;
pub(crate) mod impls;
pub mod operations;

pub use contract::{EvmContract, DeployConfig};
pub use operations::EvmCallTransaction;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;
