use std::collections::HashMap;
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
        for tx in transactions.iter() {
            let public_key_hash = tx.get_public_key_hash();
            if public_key_hash.is_none() {
                panic!("Invalid transaction");
            }
            let public_key_hash = public_key_hash.unwrap();
            let (balance, unspend_txo) = self.find_utxo(public_key_hash);
            if !tx.validate(unspend_txo, balance) {
                panic!("Invalid transaction");
            }
        }

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

    pub fn find_utxo(&self, public_key_hash: String) -> (u128,HashMap<Vec<u8>, Vec<(u128, u128)>> ){
        let mut unspend_txo: HashMap<Vec<u8>,  Vec<(u128, u128)>> = HashMap::new();
        let mut spend_txo: HashMap<Vec<u8>, Vec<u128>> = HashMap::new();
        let mut iter = self.iterator();
        let mut balance = 0;

        while let Some(block) = iter.next() {
            for tx in block.get_transaction() {
                let tx_id = tx.get_tx_id();
               
        'outer: for (out_idx, tx_out) in tx.get_tx_output().iter().enumerate() {
                    if let Some(spent_outs) = spend_txo.get(&tx_id) {
                        for spent_out in spent_outs {
                            if *spent_out == out_idx as u128 {
                                continue 'outer;
                            }
                        }
                    }
                    if tx_out.can_be_unlocked_with(&public_key_hash) {
                        balance += tx_out.get_value();
                        unspend_txo.entry(tx_id.clone()).or_insert(vec![])
                                   .push((out_idx as u128, tx_out.get_value()));
                    }
                }
                if !tx.is_coinbase() {
                    for tx_in in tx.get_tx_input() {
                        if tx_in.is_used_by(&public_key_hash) {
                            let in_tx_id = tx_in.get_txid();
                            spend_txo.entry(in_tx_id).or_insert(vec![]).push(tx_in.get_vout());
                        }
                    }
                }
            }
        }

        (balance, unspend_txo)

    }

    pub fn get_balance(&self, public_key_hash: String) -> u128 {
        let mut balance: u128 = 0;
        let mut spend_txo: HashMap<Vec<u8>, Vec<u128>> = HashMap::new();
        let mut iter = self.iterator();

        while let Some(block) = iter.next() {
            for tx in block.get_transaction() {
                let tx_id = tx.get_tx_id();
               
        'outer: for (out_idx, tx_out) in tx.get_tx_output().iter().enumerate() {
                    if let Some(spent_outs) = spend_txo.get(&tx_id) {
                        for spent_out in spent_outs {
                            if *spent_out == out_idx as u128 {
                                continue 'outer;
                            }
                        }
                    }
                    if tx_out.can_be_unlocked_with(&public_key_hash) {
                        balance += tx_out.get_value();
                    }
                }
                if !tx.is_coinbase() {
                    for tx_in in tx.get_tx_input() {
                        if tx_in.is_used_by(&public_key_hash) {
                            let in_tx_id = tx_in.get_txid();
                            spend_txo.entry(in_tx_id).or_insert(vec![]).push(tx_in.get_vout());
                        }
                    }
                }
            }
        }

        balance
    }
}

#[derive(Clone)]
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