//! A simple blockchain.

use crate::block::Block;

/// `Blockchain` object.
/// 
/// The `Blockchain` stores:
/// * A `Vec` containing all `Block`s
/// * A difficulty level which will impact time spent mining/generating proof of work (the nonce)
/// * A `Vec` containing SHA256 values of each `Block`
/// 
#[derive(Debug)]
pub struct Blockchain {
    /// Contains all `Block`s stored in this particular `Blockchain`
    pub chain: Vec<Block>,
    /// A difficulty level which will impact the time spent mining/generating proof of work (the nonce)
    pub difficulty: usize,
    /// Contains SHA256 values of all `Block`s stored in this particular `Blockchain`
    pub transactions: Vec<String>
}

/// `Blockchain` operations.
impl Blockchain {
    /// Initialize a new `Blockchain`.
    pub fn new(difficulty: usize) -> Blockchain {
        println!("Creating genesis block");
        let genesis = Block::new("Genesis Block", difficulty, 0, &"0".repeat(64));
        let genesis_hash = genesis.block_hash.clone().to_string();

        Blockchain{
            chain: vec![genesis],
            difficulty: difficulty,
            transactions: vec![genesis_hash]
        }
    }

    /// Add a new `Block` to the `Blockchain`.
    pub fn add_transaction(&mut self, data: String) {
        let last_block = &self.chain[self.chain.len() - 1];
        let previous_hash = &last_block.block_hash;

        let new_block = Block::new(&data, self.difficulty, self.chain.len(), previous_hash);
        self.transactions.push(new_block.block_hash.clone().to_string());
        self.chain.push(new_block);
    }

    /// Display the `Blockchain`.
    pub fn display(&self) {
        println!("TRANSACTIONS");
        println!("------------\n");
        for transaction in &self.transactions {
            println!("SHA256: {}", transaction);
        }

        println!("\nBLOCKS");
        println!("------");
        for block in &self.chain {
            block.display();
        }
    }
}

#[cfg(test)]
mod test_blockchain {
    use super::*;

    #[test]
    fn test_new() {
        let blockchain = Blockchain::new(1);

        assert_eq!(blockchain.chain.len(), 1);
        assert_eq!(blockchain.transactions.len(), 1);
    }

    #[test]
    fn test_add_transaction() {
        let mut blockchain = Blockchain::new(1);
        blockchain.add_transaction("test".to_string());

        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(blockchain.transactions.len(), 2);
    }
}
