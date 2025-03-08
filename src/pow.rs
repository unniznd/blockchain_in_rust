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

    fn prepare_data(&self, nonce: u128) -> Vec<u8> {
        let previous_hash = self.block.get_previous_hash();
        let tx_hash = self.block.hash_transaction();
        let timestamp = self.block.get_timestamp();
        let blocknumber = self.block.get_blocknumer();
        let mut data_bytes = vec![];
        data_bytes.extend(blocknumber.to_be_bytes());
        data_bytes.extend(timestamp.to_be_bytes());
        data_bytes.extend(previous_hash);
        data_bytes.extend(tx_hash);
        data_bytes.extend(nonce.to_be_bytes());
        data_bytes
    }

    pub fn run(self) -> (u128, Vec<u8>) {
        let mut nonce = 0;
        while nonce < u128::MAX {
            let data = self.prepare_data(nonce);
            let mut hasher = Sha256::new();
            hasher.input(&data);
            let mut result = vec![0; hasher.output_bytes()]; 
            hasher.result(&mut result);
            let hash = encode(&result);
            if hash.starts_with(&"0".repeat(self.target as usize)) {
                return (nonce, result);
            }
            nonce += 1;
        }
        panic!("No nonce found");
    }
}
