use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crypto::{digest::Digest, sha2::Sha256};

const SUBSIDY: u128 = 10;


#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct TxInput{
    txid: Vec<u8>,
    vout: u128,
    signature: Vec<u8>,
    public_key: String
}

impl TxInput {
    pub fn new(txid:Vec<u8> , vout:u128, public_key:String) -> TxInput {
        TxInput{
            txid,
            vout,
            signature: vec![],
            public_key
        }
    }
    pub fn get_txid(&self) -> Vec<u8> {
        self.txid.clone()
    }

    pub fn get_vout(&self) -> u128 {
        self.vout
    }

    pub fn _get_public_key(&self) -> String {
        self.public_key.clone()
    }

    pub fn verify_signature(&self) -> bool {
        self.signature.len() == 0
    }

    pub fn is_used_by(&self, public_key_hash: &String) -> bool {
        self.public_key == *public_key_hash
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxOutput{
    value: u128,
    public_key_hash: String
}

impl TxOutput{
    pub fn new(value: u128, public_key_hash: String) -> TxOutput {
        TxOutput{
            value,
            public_key_hash
        }
    }

    pub fn get_value(&self) -> u128 {
        self.value
    }

    pub fn _get_public_key_hash(&self) -> String {
        self.public_key_hash.clone()
    }

    pub fn can_be_unlocked_with(&self, public_key_hash: &String) -> bool {
        self.public_key_hash == *public_key_hash
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction{
    id: Vec<u8>,
    tx_input: Vec<TxInput>,
    tx_output: Vec<TxOutput>
}

impl Transaction {
    pub fn new(tx_input: Vec<TxInput>, tx_output: Vec<TxOutput>) -> Transaction {
        let mut tx = Transaction{
            id: vec![],
            tx_input,
            tx_output
        };

        tx.id = tx.hash();

        tx
    }
    pub fn new_coinbase_tx(to: Vec<String>) -> Transaction {

        let mut tx_output = Vec::new();
        for addr in to.iter(){
            let tx = TxOutput::new(SUBSIDY, addr.to_string());
            tx_output.push(tx);
        }
        let tx_input = TxInput::default();

        let mut tx = Transaction{
            id: vec![],
            tx_input: vec![tx_input],
            tx_output
        };

        tx.id = tx.hash();

        tx
    }

    pub fn validate(&self, unspend_txo: HashMap<Vec<u8>, Vec<(u128, u128)>>, balance:u128) -> bool{
        let tx_input = self.tx_input.clone();
        let mut total_input = 0;
        let total_output = self.tx_output.iter().fold(0, |acc, x| acc + x.value);

        for tx_in in tx_input.iter() {
            if unspend_txo.get(&tx_in.get_txid()).is_none() {
                return false;
            }
            let tx_out = unspend_txo.get(&tx_in.txid).unwrap();
            let mut spendable = false;
            for (_idx, out) in tx_out.iter().enumerate() {
                if out.0 == tx_in.vout {
                    spendable = true;
                    total_input += out.1;
                    break;
                }
            }
            if !spendable {
                return false;
            }

            if tx_in.verify_signature() == false {
                return false;
            } 
        }
        if total_input < balance {
            return false;
        }
        if total_input != total_output {
            return false;
        }

        true
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        let tx_copy = Transaction{
            id: vec![],
            tx_input: self.tx_input.clone(),
            tx_output: self.tx_output.clone()
        };

        hasher.input(&tx_copy.serialize());

        let mut result = vec![0; hasher.output_bytes()]; 
        hasher.result(&mut result);

        result
    }

    pub fn is_coinbase(&self) -> bool {
        self.tx_input.len() == 1 &&  self.tx_input[0].public_key.len() == 0
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn get_tx_output(&self) -> Vec<TxOutput> {
        self.tx_output.clone()
    }

    pub fn get_tx_input(&self) -> Vec<TxInput> {
        self.tx_input.clone()
    }

    pub fn get_tx_id(&self) -> Vec<u8> {
        self.id.clone()
    }

    pub fn get_public_key_hash(&self) -> Option<String> {
        let mut previous_hash = self.tx_input[0].public_key.clone();
        let len =  self.tx_input.len();
        for i in 1..len{
            if previous_hash != self.tx_input[i].public_key {
                return None;
            }   
            previous_hash = self.tx_input[i].public_key.clone();
        }
        Some(previous_hash)
    } 
}