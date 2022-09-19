use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("invalid address")]
    AddressError(#[from] std::),
}

pub enum EvmContractError {

}
