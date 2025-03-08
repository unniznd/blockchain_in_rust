use sled::Db;
use crate::block::Block;
use crate::transaction::Transaction;


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
            let alice = String::from("Alice");
            let bob = String::from("Bob");
            let tx = Transaction::new_coinbase_tx(vec![alice, bob]);
            let transactions = vec![tx];
            let previous_hash = vec![];

            let block = Block::create_block(blocknumber, transactions, previous_hash);
            
            blocks_tree.insert(block.get_block_hash(), block.serialize()).unwrap();
            blocks_tree.insert(LAST_BLOCK_HASH, block.get_block_hash()).unwrap();

            Blockchain {
                db,
                last_block_hash: block.get_block_hash(),
            }
        } else {
            Blockchain {
                db,
                last_block_hash,
            }
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let blocks_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        let last_block_hash = blocks_tree.get(LAST_BLOCK_HASH).unwrap().unwrap().to_vec();
        let last_block = blocks_tree.get(last_block_hash).unwrap().unwrap().to_vec();
        let last_block = Block::deserialize(&last_block);
        let blocknumber = last_block.get_blocknumer() + 1;
        let previous_hash = last_block.get_block_hash(); 

        let block = Block::create_block(
            blocknumber, 
            transactions, 
            previous_hash
        );

    
        blocks_tree.insert(block.get_block_hash(), block.serialize()).unwrap();
        blocks_tree.insert(LAST_BLOCK_HASH, block.get_block_hash()).unwrap();

        self.last_block_hash = block.get_block_hash();
    }

    pub fn iterator(&self) -> BlockchainIterator{
        BlockchainIterator::new(self.db.clone(), self.last_block_hash.clone())
    }
}


pub struct BlockchainIterator{
    db: Db,
    current_hash: Vec<u8>
}

impl BlockchainIterator {
    pub fn new(db: Db, current_hash: Vec<u8>) -> BlockchainIterator {
        BlockchainIterator{
            db,
            current_hash
        }
    }

    pub fn next(&mut self) -> Option<Block>{
        let blocks_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        let data = blocks_tree.get(self.current_hash.clone()).unwrap();
        if data.is_none() {
            return None;
        }
        let block = Block::deserialize(data.unwrap().to_vec().as_slice());
        self.current_hash = block.get_previous_hash();
        return Some(block);
    }
    
}