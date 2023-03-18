use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[derive(Debug, Default, BorshDeserialize, BorshSerialize, Clone)]
pub struct Proof {
    pub log_index: u64,
    pub log_entry_data: Vec<u8>,
    pub receipt_index: u64,
    pub receipt_data: Vec<u8>,
    pub header_data: Vec<u8>,
    pub proof: Vec<Vec<u8>>,
}

impl Proof {
    pub fn get_key(&self) -> String {
        String::from("some-proof-key")
    }
}
