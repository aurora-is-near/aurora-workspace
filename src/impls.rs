use crate::Result;
use aurora_engine::parameters::{SubmitResult, TransactionStatus, WithdrawResult};
use aurora_engine_sdk::promise::PromiseId;
use ethereum_types::{Address, H256, U256};
use near_account_id::AccountId;
use std::str::FromStr;
use workspaces::result::ExecutionFinalResult;

pub(crate) trait AuroraReturns {
    fn try_to_account_id(self) -> Result<AccountId>;

    fn try_to_address(self) -> Result<Address>;

    fn try_to_bool(self) -> Result<bool>;

    fn try_to_evm_result(self) -> Result<SubmitResult>;

    fn try_to_promise_id(self) -> Result<PromiseId>;

    fn try_to_string(self) -> Result<String>;

    fn try_to_transaction_status(self) -> Result<TransactionStatus>;

    fn try_to_hash(self) -> Result<H256>;

    fn try_to_u64(self) -> Result<u64>;

    fn try_to_u256(self) -> Result<U256>;

    fn try_to_withdraw_result(self) -> Result<WithdrawResult>;

    fn try_to_vec(self) -> Result<Vec<u8>>;

    fn try_to_empty(self) -> Result<()>;
}

impl AuroraReturns for ExecutionFinalResult {
    fn try_to_account_id(self) -> Result<AccountId> {
        Ok(AccountId::from_str(&self.try_to_string()?)?)
    }

    fn try_to_address(self) -> Result<Address> {
        Ok(Address::from_slice(&self.raw_bytes()?))
    }

    fn try_to_bool(self) -> Result<bool> {
        Ok(self.borsh()?)
    }

    fn try_to_evm_result(self) -> Result<SubmitResult> {
        match self.into_result() {
            Ok(success) => Ok(success.borsh()?),
            Err(err) => {
                // TODO get actual error. See: near/workspaces-rs/issues/191
                Err(err.into())
            }
        }
    }

    fn try_to_promise_id(self) -> Result<PromiseId> {
        Ok(PromiseId::new(self.try_to_u64()?))
    }

    fn try_to_string(self) -> Result<String> {
        Ok(String::from_utf8(self.raw_bytes()?)?)
    }

    fn try_to_transaction_status(self) -> Result<TransactionStatus> {
        Ok(self.borsh()?)
    }

    fn try_to_hash(self) -> Result<H256> {
        Ok(H256::from_slice(&self.raw_bytes()?))
    }

    fn try_to_u64(self) -> Result<u64> {
        let bytes = self.raw_bytes()?;
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&bytes[0..8]);
        Ok(u64::from_le_bytes(buf))
    }

    fn try_to_u256(self) -> Result<U256> {
        Ok(U256::from_big_endian(&self.raw_bytes()?))
    }

    fn try_to_withdraw_result(self) -> Result<WithdrawResult> {
        Ok(self.borsh()?)
    }

    fn try_to_vec(self) -> Result<Vec<u8>> {
        Ok(self.raw_bytes()?)
    }

    fn try_to_empty(self) -> Result<()> {
        Ok(())
    }
}
