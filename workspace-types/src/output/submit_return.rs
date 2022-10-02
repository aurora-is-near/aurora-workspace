use crate::Address;
use crate::H256;
use crate::Result;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::io;
use crate::error::ErrorKind;

/// Borsh-encoded parameters for the `call`, `call_with_args`, `deploy_code`,
/// and `deploy_with_input` methods.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, Serialize, Deserialize)]
pub struct SubmitResult {
    status: TransactionStatus,
    total_gas_used: u64,
    logs: Vec<Log>,
}

impl SubmitResult {
    /// Must be incremented when making breaking changes to the SubmitResult ABI.
    /// The current value of 7 is chosen because previously a `TransactionStatus` object
    /// was first in the serialization, which is an enum with less than 7 variants.
    /// Therefore, no previous `SubmitResult` would have began with a leading 7 byte,
    /// and this can be used to distinguish the new ABI (with version byte) from the old.
    pub const VERSION: u8 = 7;

    pub fn new(status: TransactionStatus, total_gas_used: u64, logs: Vec<Log>) -> Self {
        Self {
            status,
            total_gas_used,
            logs,
        }
    }

    pub fn gas_used(&self) -> u64 {
        self.total_gas_used
    }

    pub fn logs(&self) -> &[Log] {
        &self.logs
    }

    pub fn is_ok(&self) -> bool {
        self.status.is_ok()
    }

    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }

    pub fn is_revert(&self) -> bool {
        self.status.is_revert()
    }

    pub fn is_err(&self) -> bool {
        self.status.is_err()
    }

    pub fn into_result(self) -> Result<TransactionStatus> {
        match self.status {
            TransactionStatus::Succeed(d) => Ok(TransactionStatus::Succeed(d)),
            TransactionStatus::Revert(d) => Ok(TransactionStatus::Revert(d)),
            TransactionStatus::OutOfGas => Err(ErrorKind::OutOfGas.into()),
            TransactionStatus::OutOfFund => Err(ErrorKind::OutOfFunds.into()),
            TransactionStatus::CallTooDeep => Err(ErrorKind::CallTooDeep.into()),
            TransactionStatus::OutOfOffset => Err(ErrorKind::OutOfBounds.into()),
        }
    }
}

impl BorshDeserialize for SubmitResult {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let auto: AutoSubmitResultDes =
            <AutoSubmitResultDes as BorshDeserialize>::deserialize(buf)?;

        let version: u8 = auto.version;
        if version != Self::VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "version mismatch, expected {} but received {}",
                    Self::VERSION,
                    version
                ),
            ));
        }

        Ok(Self {
            status: auto.status,
            total_gas_used: auto.gas_used,
            logs: auto.logs,
        })
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
struct AutoSubmitResultDes {
    version: u8,
    status: TransactionStatus,
    gas_used: u64,
    logs: Vec<Log>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub enum TransactionStatus {
    Succeed(Vec<u8>),
    Revert(Vec<u8>),
    OutOfGas,
    OutOfFund,
    OutOfOffset,
    CallTooDeep,
}

impl TransactionStatus {
    pub fn is_ok(&self) -> bool {
        matches!(*self, TransactionStatus::Succeed(_) | TransactionStatus::Revert(_))
    }

    pub fn is_success(&self) -> bool {
        matches!(*self, TransactionStatus::Succeed(_))
    }

    pub fn is_revert(&self) -> bool {
        matches!(*self, TransactionStatus::Revert(_))
    }

    pub fn is_err(&self) -> bool {
        *self == TransactionStatus::OutOfGas
            || *self == TransactionStatus::OutOfFund
            || *self == TransactionStatus::OutOfOffset
            || *self == TransactionStatus::CallTooDeep
    }
}

/// Borsh-encoded log for use in a `SubmitResult`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Log {
    address: Address,
    topics: Vec<H256>,
    data: Vec<u8>,
}

impl Log {
    pub fn new(address: Address, topics: Vec<H256>, data: Vec<u8>) -> Self {
        Self {
            address,
            topics,
            data,
        }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn topics(&self) -> &[H256] {
        &self.topics
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl BorshSerialize for Log {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        let topics: Vec<&[u8; 32]> = self.topics.iter().map(|t| &t.0).collect();
        let auto = LogAutoSer {
            address: &self.address.0,
            topics,
            data: &self.data,
        };
        let ser = auto.try_to_vec()?;
        writer.write_all(&ser)
    }
}

impl BorshDeserialize for Log {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let auto: LogAutoDe = <LogAutoDe as BorshDeserialize>::deserialize(buf)?;
        let topics: Vec<H256> = auto.topics.into_iter().map(H256::from).collect();
        Ok(Self {
            address: Address::from(auto.address),
            topics,
            data: auto.data,
        })
    }
}

impl From<LogAutoDe> for Log {
    fn from(log: LogAutoDe) -> Self {
        let address = Address::from(log.address);
        let topics: Vec<H256> = log.topics.into_iter().map(H256::from).collect();
        Log {
            address,
            topics,
            data: log.data,
        }
    }
}

#[derive(Debug, BorshSerialize)]
struct LogAutoSer<'a> {
    address: &'a [u8; 20],
    topics: Vec<&'a [u8; 32]>,
    data: &'a [u8],
}

/// A helper type to easily deserialize from the raw data to then be used in
/// [`Log`] as it is not entirely possible to use [`BorshDeserialize`]'s
/// deserialize function.
#[derive(Debug, BorshSerialize, BorshDeserialize)]
struct LogAutoDe {
    address: [u8; 20],
    topics: Vec<[u8; 32]>,
    data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethereum_types::H160;

    #[test]
    fn test_submit_result_deserialize() {
        // sourced from aurora-engine manually.
        let input_hex = "\
        0700200000000000000000000000000000000000000000000000000000000000\
        000000000000a086010000000000010000000101010101010101010101010101\
        0101010101010100000002020202020202020202020202020202020202020202\
        020202020202020202020a00000003030303030303030303";
        let input = hex::decode(input_hex).unwrap();
        let submit_return =
            <SubmitResult as BorshDeserialize>::deserialize(&mut input.as_slice()).unwrap();

        let expected = SubmitResult {
            status: TransactionStatus::Succeed(vec![0u8; 32]),
            total_gas_used: 100_000,
            logs: vec![Log {
                address: H160::from([1u8; 20]),
                topics: vec![H256::from([2u8; 32])],
                data: vec![3u8; 10],
            }],
        };
        assert_eq!(expected, submit_return);
    }

    #[test]
    fn test_submit_result_serde_roundtrip() {
        let pre_return = SubmitResult {
            status: TransactionStatus::Succeed(vec![0u8; 32]),
            total_gas_used: 100_000,
            logs: vec![Log {
                address: H160::from([1u8; 20]),
                topics: vec![H256::from([2u8; 32])],
                data: vec![3u8; 10],
            }],
        };
        let serialized: Vec<u8> = pre_return.try_to_vec().unwrap();
        let post_return =
            <SubmitResult as BorshDeserialize>::deserialize(&mut serialized.as_slice()).unwrap();
        assert_eq!(pre_return, post_return);
    }}
