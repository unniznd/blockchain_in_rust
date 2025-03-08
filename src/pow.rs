use crate::block::Block;
use crypto::{digest::Digest, sha2::Sha256};

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
                self.block.hash_transaction(),
                self.block.previous_hash,
                self.block.nonce
            );
            let mut hasher = Sha256::new();
            hasher.input_str(&block_serialized);
            let hash = hasher.result_str();
            if hash.starts_with(&"0".repeat(self.target as usize)) {
                self.block.hash = hash;
                return self.block; 
            }
            nonce += 1;
        }
        panic!("No nonce found");
    }
}
