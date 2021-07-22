// Blockchain entry point.

mod block;
mod blockchain;

use std::{
    thread, 
    time
};

fn main() {
    let mut blockchain = blockchain::Blockchain::new();

    println!("\nCREATING BLOCKCHAIN\n");
    for i in 0..5 {
        println!("Adding block #{}", i);

        let data = format!("BLOCK {}", i);
        blockchain.add_transaction(data);

        thread::sleep(time::Duration::from_secs(5));
    }

    println!("\n\nDISPLAYING BLOCKCHAIN\n");
    blockchain.display();
}
