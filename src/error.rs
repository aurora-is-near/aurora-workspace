use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ParseAccountError(#[from] near_account_id::ParseAccountError),
    #[error(transparent)]
    Workspace(#[from] workspaces::error::Error),
    #[error(transparent)]
    WorkspaceExecutionFailure(#[from] workspaces::result::ExecutionFailure),
    #[error(transparent)]
    Io(#[from] io::Error),
}
