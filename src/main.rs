mod blockchain;
mod block;
mod pow;
mod transaction;
use crate::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    let balance = blockchain.get_balance("Bob".to_string());
    println!("{}", balance);

    let mut iterator = blockchain.iterator();
    loop {
        let current_block = iterator.next();
        if current_block.is_none() {
            break;
        }

        println!("{:?}", current_block.unwrap());

    }
}
