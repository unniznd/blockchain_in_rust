mod blockchain;
mod block;
mod pow;
mod transaction;
use crate::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
   
    // blockchain.add_block("First block".to_string());
   
    // blockchain.add_block("Second block".to_string());

    let mut iterator = blockchain.iterator();
    loop {
        let current_block = iterator.next();
        if current_block.is_none() {
            break;
        }

        println!("{:?}", current_block.unwrap());

    }
}
