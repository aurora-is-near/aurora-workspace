mod contract;
pub mod error;
pub(crate) mod impls;

pub use contract::EvmContract;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;
