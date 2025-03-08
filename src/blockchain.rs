use sled::Db;

use crate::block::Block;
use crate::pow::ProofOfWork;


const BLOCKS_TREE: &str = "blocks_tree";
const LAST_BLOCK_HASH: &str = "last_block_hash";


pub struct Blockchain {
    pub db: Db,
    pub last_block_hash: Vec<u8>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        
        let db = sled::open("blockchain").unwrap();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
        let last_block_hash = match blocks_tree.get(LAST_BLOCK_HASH) {
            Ok(Some(hash)) => hash.to_vec(),
            _ => String::from("0").into_bytes(),
        };

        if last_block_hash == String::from("0").into_bytes() {
            let blocknumber: u128 = 1;
            let transactions = String::from("This is genesis block");
            let previous_hash = vec![];

            let genesis_block = Block::create_block(blocknumber, transactions, previous_hash);
            let pow = ProofOfWork::new(genesis_block, 2);
            let block = ProofOfWork::run(pow);
            blocks_tree.insert(block.hash.clone(), block.serialize()).unwrap();
            blocks_tree.insert(LAST_BLOCK_HASH, block.hash.clone()).unwrap();

            Blockchain {
                db,
                last_block_hash: block.hash,
            }
        } else {
            Blockchain {
                db,
                last_block_hash,
            }
        }
    }

    pub fn add_block(&mut self, transactions: String) {
        let blocks_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        let last_block_hash = blocks_tree.get(LAST_BLOCK_HASH).unwrap().unwrap().to_vec();
        let last_block = blocks_tree.get(last_block_hash).unwrap().unwrap().to_vec();
        let last_block = Block::deserialize(&last_block);
        let blocknumber = last_block.blocknumber + 1;
        let previous_hash = last_block.hash; 

        let block = Block::create_block(
            blocknumber, 
            transactions, 
            previous_hash
        );

        let pow = ProofOfWork::new(block, 2);
        let block = ProofOfWork::run(pow);

        blocks_tree.insert(block.hash.clone(), block.serialize()).unwrap();
        blocks_tree.insert(LAST_BLOCK_HASH, block.hash.clone()).unwrap();

        self.last_block_hash = block.hash;
    }

    pub fn get_all_blocks(&self) -> Vec<Block> {
        let blocks_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        let mut blocks = Vec::new();
        let mut current_block_hash = self.last_block_hash.clone();
        for _ in 0..blocks_tree.len() {
            if current_block_hash == vec![]{
                break;
            }
            let current_block = blocks_tree.get(current_block_hash).unwrap().unwrap().to_vec();
            let current_block = Block::deserialize(&current_block);
            blocks.push(current_block.clone());
            current_block_hash = current_block.previous_hash;
        }
        blocks
    }
}
