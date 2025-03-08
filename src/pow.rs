use crate::block::Block;
use crypto::{digest::Digest, sha2::Sha256};
use hex::encode;
pub struct ProofOfWork {
    pub block: Block,
    pub target: u128,
}

impl ProofOfWork {
    pub fn new(block: Block, target: u128) -> ProofOfWork {
        ProofOfWork { block, target }
    }

    pub fn run(mut self) -> Block {
        let mut nonce = 0;
        while nonce < u128::MAX {
            self.block.nonce = nonce;
            let block_serialized = format!(
                "{}{}{}{}{}",
                self.block.blocknumber,
                self.block.timestamp,
                encode(self.block.hash_transaction()),
                encode(self.block.previous_hash.clone()),
                self.block.nonce
            );
            let mut hasher = Sha256::new();
            hasher.input_str(&block_serialized);
            let mut result = vec![0; hasher.output_bytes()]; 
            hasher.result(&mut result);
            let hash = encode(&result);
            if hash.starts_with(&"0".repeat(self.target as usize)) {
                self.block.hash = result;
                return self.block; 
            }
            nonce += 1;
        }
        panic!("No nonce found");
    }
}
