use std::io;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ParseAccountError(#[from] crate::types::ParseAccountError),
    #[error(transparent)]
    Workspace(#[from] workspaces::error::Error),
    #[error(transparent)]
    WorkspaceExecutionFailure(#[from] workspaces::result::ExecutionFailure),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[cfg(feature = "ethabi")]
    #[error(transparent)]
    EthAbi(#[from] ethabi::Error),
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
