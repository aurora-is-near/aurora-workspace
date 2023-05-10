use near_sdk::{json_types::U128, PromiseOrValue};
use serde::de::DeserializeOwned;
use std::borrow::Borrow;
use workspaces::result::{ExecutionFinalResult, ExecutionOutcome};
use workspaces::types::Gas;

#[derive(Debug, Eq, PartialOrd, PartialEq)]
pub struct ViewResult<T> {
    pub result: T,
    pub logs: Vec<String>,
}

impl<T: DeserializeOwned> ViewResult<T> {
    pub fn json(view: workspaces::result::ViewResultDetails) -> anyhow::Result<Self> {
        Ok(Self {
            result: view.json()?,
            logs: view.logs,
        })
    }
}

impl<T: borsh::BorshDeserialize> ViewResult<T> {
    pub fn borsh(view: workspaces::result::ViewResultDetails) -> anyhow::Result<Self> {
        Ok(Self {
            result: view.borsh()?,
            logs: view.logs,
        })
    }
}

#[derive(Debug)]
pub struct ExecutionResult<T> {
    inner: workspaces::result::ExecutionSuccess,
    value: T,
    success: bool,
}

impl<T: DeserializeOwned> ExecutionResult<T> {
    pub fn json(result: workspaces::result::ExecutionFinalResult) -> anyhow::Result<Self> {
        let success = result.is_success();
        let inner = result.into_result()?;
        let value = inner.json()?;
        Ok(Self::new(inner, value, success))
    }
}

impl TryFrom<ExecutionFinalResult> for ExecutionResult<PromiseOrValue<U128>> {
    type Error = anyhow::Error;

    fn try_from(result: ExecutionFinalResult) -> Result<Self, Self::Error> {
        let success = result.is_success();
        let inner = result.into_result()?;
        let res: U128 = inner.json()?;
        let value = PromiseOrValue::Value(res);
        Ok(Self::new(inner, value, success))
    }
}

impl<T: borsh::BorshDeserialize> ExecutionResult<T> {
    pub fn borsh(result: ExecutionFinalResult) -> anyhow::Result<Self> {
        let success = result.is_success();
        let inner = result.into_result()?;
        let value = inner.borsh()?;
        Ok(Self::new(inner, value, success))
    }
}

impl<T> ExecutionResult<T> {
    pub fn new(inner: workspaces::result::ExecutionSuccess, value: T, success: bool) -> Self {
        Self {
            inner,
            value,
            success,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn into_value(self) -> T {
        self.value
    }

    pub fn total_gas_burnt(&self) -> Gas {
        self.inner.total_gas_burnt
    }

    pub fn outcome(&self) -> &ExecutionOutcome {
        self.inner.outcome()
    }

    pub fn outcomes(&self) -> Vec<&ExecutionOutcome> {
        self.inner.outcomes()
    }

    pub fn receipt_outcomes(&self) -> &[ExecutionOutcome] {
        self.inner.receipt_outcomes()
    }

    pub fn failures(&self) -> Vec<&ExecutionOutcome> {
        self.inner.failures()
    }

    pub fn receipt_failures(&self) -> Vec<&ExecutionOutcome> {
        self.inner.receipt_failures()
    }

    pub fn logs(&self) -> Vec<&str> {
        self.inner.logs()
    }

    pub fn is_success(&self) -> bool {
        self.success
    }

    pub fn is_failure(&self) -> bool {
        !self.success
    }
}

impl<T> AsRef<T> for ExecutionResult<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> Borrow<T> for ExecutionResult<T> {
    fn borrow(&self) -> &T {
        &self.value
    }
}
