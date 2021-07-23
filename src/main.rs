//! A Simple Blockchain.

mod block;
mod blockchain;

use std::time;

/// Run a simple blockchain implementation.
fn main() {
    let start = time::Instant::now();

    let difficulty = 5;   // Change this value to set a difficulty level.
    let total_blocks = 5; // Change this value to set the total number of 
                          //    `Block`s in the `Blockchain`.

    if total_blocks < 2 {
        panic!(
            format!(
                "Cannot provide value of {}. Genesis block is the first block.", 
                total_blocks
            )
        );
    }

    println!("\nINITIALIZING BLOCKCHAIN");
    println!("=======================\n");
    
    println!("SETTING DIFFICULTY LEVEL OF {}\n", difficulty);

    let mut blockchain = blockchain::Blockchain::new(difficulty);

    let mut plurality = String::from("BLOCK");
    if total_blocks > 2 {
        plurality.push_str(&"S");
    }
    println!("ADDING {} {} TO THE BLOCKCHAIN\n", total_blocks - 1, plurality);
    
    for i in 1..total_blocks {
        println!("Adding block #{}", i);

        let data = format!("BLOCK {}", i);
        blockchain.add_block(data);
    }

    println!("\nDISPLAYING BLOCKCHAIN METADATA");
    println!("==============================\n");
    blockchain.display();

    println!(
        "FINISHED CREATING BLOCKCHAIN WITH {} BLOCKS IN {:?} SECONDS.\n",
        blockchain.chain.len(),
        start.elapsed().as_secs()
    );
}
