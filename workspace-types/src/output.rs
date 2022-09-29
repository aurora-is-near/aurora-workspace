use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use crate::{H256};
use crate::Address;
use std::io::{self, Write};

/// Borsh-encoded parameters for the `call`, `call_with_args`, `deploy_code`,
/// and `deploy_with_input` methods.
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, Serialize, Deserialize)]
pub struct SubmitReturn {
    version: u8,
    status: TransactionStatus,
    gas_used: u64,
    logs: Vec<Log>,
}

impl SubmitReturn {
    /// Must be incremented when making breaking changes to the SubmitResult ABI.
    /// The current value of 7 is chosen because previously a `TransactionStatus` object
    /// was first in the serialization, which is an enum with less than 7 variants.
    /// Therefore, no previous `SubmitResult` would have began with a leading 7 byte,
    /// and this can be used to distinguish the new ABI (with version byte) from the old.
    pub const VERSION: u8 = 7;

    pub fn status(&self) -> &TransactionStatus {
        &self.status
    }

    pub fn gas_used(&self) -> u64 {
        self.gas_used
    }

    pub fn logs(&self) -> &[Log] {
        &self.logs
    }
}

impl BorshDeserialize for SubmitReturn {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let version: u8 = *buf.first().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no data received"))?;
        if version != Self::VERSION {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("version mismatch, expected {} but received {}", Self::VERSION, version)));
        }

        let auto: AutoSubmitResultDes = <AutoSubmitResultDes as BorshDeserialize>::deserialize(buf)?;
        Ok(Self {
            version,
            status: auto.status,
            gas_used: auto.gas_used,
            logs: auto.logs,
        })
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
struct AutoSubmitResultDes {
    version: u8,
    pub status: TransactionStatus,
    pub gas_used: u64,
    pub logs: Vec<Log>,
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
        matches!(*self, TransactionStatus::Succeed(_))
    }

    pub fn is_revert(&self) -> bool {
        matches!(*self, TransactionStatus::Revert(_))
    }

    pub fn is_fail(&self) -> bool {
        *self == TransactionStatus::OutOfGas
            || *self == TransactionStatus::OutOfFund
            || *self == TransactionStatus::OutOfOffset
            || *self == TransactionStatus::CallTooDeep
    }
}

/// Borsh-encoded log for use in a `SubmitResult`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Log {
    pub address: Address,
    pub topics: Vec<H256>,
    pub data: Vec<u8>,
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
        let topics: Vec<H256> =  auto.topics.into_iter().map(H256::from).collect();
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
    topics: Vec<&'a[u8; 32]>,
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
    use ethereum_types::H160;
    use super::*;

    #[test]
    fn test_submit_result_deserialize() {
        // sourced from aurora-engine manually.
        let input_hex = "\
        0700200000000000000000000000000000000000000000000000000000000000\
        000000000000a086010000000000010000000101010101010101010101010101\
        0101010101010100000002020202020202020202020202020202020202020202\
        020202020202020202020a00000003030303030303030303";
        let input = hex::decode(input_hex).unwrap();
        let submit_return = <SubmitReturn as BorshDeserialize>::deserialize(&mut input.as_slice()).unwrap();

        let expected = SubmitReturn {
            version: 7,
            status: TransactionStatus::Succeed(vec![0u8; 32]),
            gas_used: 100_000,
            logs: vec![Log {
                address: H160::from([1u8; 20]),
                topics: vec![H256::from([2u8; 32])],
                data: vec![3u8; 10]
            }]
        };
        assert_eq!(expected, submit_return);
    }

    #[test]
    fn test_submit_result_serde_roundtrip() {
        let pre_return = SubmitReturn {
            version: 7,
            status: TransactionStatus::Succeed(vec![0u8; 32]),
            gas_used: 100_000,
            logs: vec![Log {
                address: H160::from([1u8; 20]),
                topics: vec![H256::from([2u8; 32])],
                data: vec![3u8; 10]
            }]
        };
        let serialized: Vec<u8> = pre_return.try_to_vec().unwrap();
        let post_return = <SubmitReturn as BorshDeserialize>::deserialize(&mut serialized.as_slice()).unwrap();
        assert_eq!(pre_return, post_return);
    }
}
