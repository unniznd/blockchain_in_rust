use serde::{Deserialize, Serialize};
use crypto::{digest::Digest, sha2::Sha256};

const SUBSIDY: i32 = 10;


#[derive(Default, Serialize, Deserialize, Clone, Debug)]
struct TxInput{
    txid: Vec<u8>,
    vout: u128,
    signature: Vec<u8>,
    public_key: Vec<u8>
}

impl TxInput {
    pub fn new(txid:Vec<u8> , vout:u128) -> TxInput {
        TxInput{
            txid,
            vout,
            signature: vec![],
            public_key: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct TxOutput{
    value: i32,
    public_key: String
}

impl TxOutput{
    pub fn new(value: i32, public_key: String) -> TxOutput {
        TxOutput{
            value,
            public_key
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction{
    pub id: Vec<u8>,
    tx_input: Vec<TxInput>,
    tx_output: Vec<TxOutput>
}

impl Transaction {
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

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}