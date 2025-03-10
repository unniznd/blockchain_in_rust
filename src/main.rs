mod blockchain;
mod block;
mod pow;
mod transaction;

use transaction::{Transaction, TxInput, TxOutput};

use crate::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    let balance = blockchain.get_balance("Bob".to_string());
    println!("{}", balance);
    let utxo = blockchain.find_utxo("Bob".to_string());
    println!("{:?}", utxo);

    let tx_input = TxInput::new(
        [28, 137, 215, 202, 222, 192, 7, 14, 247, 32, 208, 235, 45, 170, 144, 48, 205, 26, 117, 238, 202, 183, 5, 74, 130, 97, 116, 198, 238, 128, 40, 88].to_vec(),
        1,
        "Bob".to_string()
    );

    let tx_output1 = TxOutput::new(10, "Alice".to_string());
    // let tx_output2 = TxOutput::new(0, "Bob".to_string());

    let tx = Transaction::new(vec![tx_input], vec![tx_output1]);

    blockchain.add_block(vec![tx]);

    let balance = blockchain.get_balance("Bob".to_string());
    println!("{}", balance);
    let utxo = blockchain.find_utxo("Bob".to_string());
    println!("{:?}", utxo);

    let mut iterator = blockchain.iterator();
    loop {
        let current_block = iterator.next();
        if current_block.is_none() {
            break;
        }

        println!("{:?}", current_block.unwrap());

    }
}
