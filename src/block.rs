use std::time::{SystemTime, UNIX_EPOCH};
use crypto::{digest::Digest, sha2::Sha256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub blocknumber: u128,
    pub timestamp: u128,
    pub transactions: String,
    pub previous_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: u128,
}

impl Block {
    pub fn create_block(
        blocknumber: u128, 
        transactions: String, 
        previous_hash: Vec<u8>
    ) -> Block {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = since_the_epoch.as_millis();

        let block = Block {
            blocknumber,
            timestamp,
            transactions,
            previous_hash,
            hash: vec![],
            nonce: 0,
        };

        block
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn deserialize(data: &[u8]) -> Block {
        bincode::deserialize(data).unwrap()
    }

    pub fn hash_transaction(&self) -> Vec<u8>{
        let mut hasher = Sha256::new();
        hasher.input_str(&self.transactions);

        let mut result = vec![0; hasher.output_bytes()]; 
        hasher.result(&mut result);

        result
    }
}
