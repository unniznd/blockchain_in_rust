mod blockchain;
mod block;
mod pow;
mod transaction;
use crate::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
   
    // blockchain.add_block("First block".to_string());
   
    // blockchain.add_block("Second block".to_string());

    let blocks = blockchain.get_all_blocks();
    for i in (0..blocks.len()).rev() {
        println!("{:?}", blocks[i]);
    }
}
