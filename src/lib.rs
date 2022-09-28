mod contract;
pub mod error;
pub(crate) mod operation;
pub(crate) mod result;
#[cfg(not(feature = "mock"))]
mod input;
#[cfg(feature = "mock")]
pub mod input;

pub use contract::{InitConfig, EvmAccount, EvmContract};
pub use operation::EvmCallTransaction;

pub use aurora_engine::proof::Proof;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;
