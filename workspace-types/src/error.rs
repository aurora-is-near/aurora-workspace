#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("transaction ran out of gas")]
    OutOfGas,
    #[error("account ran out of funds during execution")]
    OutOfFunds,
    #[error("transaction call is too deep")]
    CallTooDeep,
    #[error("data is out of bounds")]
    OutOfBounds,
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(Box<ErrorKind>);

impl<E> From<E> for Error
where
    ErrorKind: From<E>,
{
    fn from(err: E) -> Self {
        Error(Box::new(ErrorKind::from(err)))
    }
}
