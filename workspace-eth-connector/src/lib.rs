pub mod contract;
pub mod error;
pub mod operation;
pub(crate) mod result;

use crate::error::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;
