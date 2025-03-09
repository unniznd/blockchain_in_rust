use std::time::{SystemTime, UNIX_EPOCH};
use crypto::{digest::Digest, sha2::Sha256};
use hex::encode;
use serde::{Deserialize, Serialize};
use crate::transaction::Transaction;
use crate::pow::ProofOfWork;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    blocknumber: u128,
    timestamp: u128,
    transactions: Vec<Transaction>,
    previous_hash: Vec<u8>,
    hash: Vec<u8>,
    nonce: u128,
}

impl Block {
    pub fn create_block(
        blocknumber: u128, 
        transactions: Vec<Transaction>, 
        previous_hash: Vec<u8>
    ) -> Block {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = since_the_epoch.as_millis();

        let mut block = Block {
            blocknumber,
            timestamp,
            transactions,
            previous_hash,
            hash: vec![],
            nonce: 0,
        };

        let pow = ProofOfWork::new(block.clone(), 2);
        let (nonce, hash) = ProofOfWork::run(pow);
        block.nonce = nonce;
        block.hash = hash;

        block
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn deserialize(data: &[u8]) -> Block {
        bincode::deserialize(data).unwrap()
    }

    pub fn hash_transaction(&self) -> Vec<u8>{
        let mut tx_string = String::new();
        for tx in self.transactions.iter(){
            let tx_serialized = tx.serialize();
            tx_string.push_str(&encode(tx_serialized));

        }

        let mut hasher = Sha256::new();
        hasher.input_str(&tx_string);

        let mut result = vec![0; hasher.output_bytes()]; 
        hasher.result(&mut result);

        result
    }

    pub fn get_blocknumer(&self) -> u128{
        self.blocknumber
    }

    pub fn get_previous_hash(&self) -> Vec<u8> {
        self.previous_hash.clone()
    }

    pub fn get_block_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }
    
    pub fn get_timestamp(&self) -> u128 {
        self.timestamp
    }

    pub fn get_transaction(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }
}
