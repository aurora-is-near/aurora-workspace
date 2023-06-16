use workspaces::result::ExecutionFinalResult;
use workspaces::rpc::query::{Query, ViewFunction};

pub struct ViewTransaction<'a> {
    inner: Query<'a, ViewFunction>,
}

impl<'a> ViewTransaction<'a> {
    pub(crate) fn new(view_tx: Query<'a, ViewFunction>) -> Self {
        Self { inner: view_tx }
    }

    pub fn args(mut self, args: Vec<u8>) -> Self {
        self.inner = self.inner.args(args);
        self
    }

    pub fn args_json<U: serde::Serialize>(mut self, args: U) -> Self {
        self.inner = self.inner.args_json(args);
        self
    }

    pub fn args_borsh<U: borsh::BorshSerialize>(mut self, args: U) -> Self {
        self.inner = self.inner.args_borsh(args);
        self
    }

    pub async fn transact(self) -> anyhow::Result<workspaces::result::ViewResultDetails> {
        Ok(self.inner.await?)
    }
}

pub struct CallTransaction<'a> {
    inner: workspaces::operations::CallTransaction<'a>,
}

impl<'a> CallTransaction<'a> {
    pub(crate) fn new(call_tx: workspaces::operations::CallTransaction<'a>) -> Self {
        Self { inner: call_tx }
    }

    pub fn args(mut self, args: Vec<u8>) -> Self {
        self.inner = self.inner.args(args);
        self
    }

    pub fn args_json<S: serde::Serialize>(mut self, args: S) -> Self {
        self.inner = self.inner.args_json(args);
        self
    }

    pub fn args_borsh<B: borsh::BorshSerialize>(mut self, args: B) -> Self {
        self.inner = self.inner.args_borsh(args);
        self
    }

    pub fn gas(mut self, gas: u64) -> Self {
        self.inner = self.inner.gas(gas);
        self
    }

    pub fn max_gas(mut self) -> Self {
        self.inner = self.inner.max_gas();
        self
    }

    pub fn deposit(mut self, deposit: u128) -> Self {
        self.inner = self.inner.deposit(deposit);
        self
    }

    pub async fn transact(self) -> anyhow::Result<ExecutionFinalResult> {
        Ok(self.inner.transact().await?)
    }
}
