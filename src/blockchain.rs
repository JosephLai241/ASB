// Blockchain.

use crate::block::Block;

// Blockchain struct.
#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub transactions: Vec<String>
}

// Blockchain operations.
impl Blockchain {
    // Initialize a new blockchain.
    pub fn new() -> Blockchain {
        let genesis = Block::new("Genesis Block", "0");
        let genesis_hash = genesis.block_hash.clone().to_string();

        Blockchain{
            chain: vec![genesis],
            transactions: vec![genesis_hash]
        }
    }

    // Add a block to the blockchain.
    pub fn add_transaction(&mut self, data: String) -> bool {
        let last_block = &self.chain[self.chain.len() - 1];
        let previous_hash = &last_block.block_hash;

        let new_block = Block::new(&data, previous_hash);
        self.transactions.push(new_block.block_hash.clone().to_string());
        self.chain.push(new_block);

        true
    }

    // Display blockchain.
    pub fn display(&self) {
        println!("TRANSACTIONS\n");
        for transaction in &self.transactions {
            println!("SHA256: {}", transaction);
        }

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
        let blockchain = Blockchain::new();

        assert_eq!(blockchain.chain.len(), 1);
    }

    #[test]
    fn test_add_transaction() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction("test".to_string());

        assert_eq!(blockchain.chain.len(), 2);
    }
}
