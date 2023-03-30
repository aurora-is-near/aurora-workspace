use crate::error::Error;
use crate::Result;
use borsh::BorshDeserialize;
use near_sdk::json_types::U128;
use near_sdk::PromiseOrValue;
use serde::de::DeserializeOwned;
use std::borrow::Borrow;
use std::fmt::Debug;
use workspaces::result::{ExecutionFinalResult, ExecutionOutcome};
use workspaces::types::Gas;

pub type ExecutionSuccess<T> = ExecutionResult<T>;

impl<T: DeserializeOwned> ExecutionSuccess<T> {
    pub(crate) fn try_from_json(result: ExecutionFinalResult) -> Result<Self> {
        let success = result.into_result()?;
        let value: T = success.json()?;
        Ok(ExecutionSuccess {
            inner: success,
            value,
        })
    }
}

impl<T: BorshDeserialize> ExecutionSuccess<T> {
    pub(crate) fn try_from_borsh(result: ExecutionFinalResult) -> Result<Self> {
        let inner = result.into_result()?;
        let value: T = T::try_from_slice(&inner.raw_bytes()?)?;
        Ok(ExecutionSuccess { inner, value })
    }
}

impl TryFrom<ExecutionFinalResult> for ExecutionSuccess<PromiseOrValue<U128>> {
    type Error = Error;

    fn try_from(result: ExecutionFinalResult) -> Result<Self> {
        let inner = result.into_result()?;
        let value: U128 = inner.json()?;

        Ok(ExecutionSuccess {
            inner,
            value: PromiseOrValue::Value(value),
        })
    }
}

impl TryFrom<ExecutionFinalResult> for ExecutionSuccess<()> {
    type Error = Error;

    fn try_from(result: ExecutionFinalResult) -> Result<Self> {
        let inner = result.into_result()?;
        Ok(ExecutionSuccess { inner, value: () })
    }
}

#[derive(Debug)]
pub struct ExecutionResult<T> {
    inner: workspaces::result::ExecutionSuccess,
    pub(crate) value: T,
}

impl<T> ExecutionResult<T> {
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
