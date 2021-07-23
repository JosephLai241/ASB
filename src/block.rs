//! A simple block in a simple blockchain.

use data_encoding::HEXLOWER;
use chrono::prelude::Local;
use ring::digest::{
    Context,
    SHA256
};

/// `Block` object.
/// 
/// Each `Block` stores data such as:
/// * The hash value of this block
/// * Data in the form of a `String`
/// * The index value of this block, denoting its position within the blockchain
/// * The hash value of the previous block in the blockchain
/// * The timestamp on which this block was created
/// 
#[derive(Debug)]
pub struct Block {
    /// The hash value of this `Block`.
    pub block_hash: String,
    /// Data is stored into a `String`, for simplicity's sake.
    data: String,
    /// Indicates the `Block`'s position within the blockchain.           
    index: usize,
    /// The nonce calculated by the proof of work algorithm.
    nonce: usize,
    /// The hash of the previous `Block`.
    previous_hash: String,
    /// The timestamp on which this `Block` was created.
    timestamp: String,
}

/// `Block` operations.
impl Block {
    /// Check if the hash is valid based on the difficulty value.
    fn is_valid_hash(&self, difficulty: usize, hash: &str) -> bool {
        let pattern = &"0".repeat(difficulty);

        hash.starts_with(pattern)
    }

    /// Get the concatenated string containing `Block` metadata.
    fn get_hash_string(&self) -> String {
        let mut hash_string = self.index.to_string();
        hash_string.push_str(&self.data);
        hash_string.push_str(&self.timestamp);
        hash_string.push_str(&self.previous_hash);

        hash_string
    }

    /// Calculate the SHA256 hash value from the hash string.
    fn get_hash(hash_string: &str) -> String {
        let mut context = Context::new(&SHA256);
        context.update(hash_string.to_string().as_bytes());

        HEXLOWER.encode(context.finish().as_ref())
    }

    /// Generate the proof of work to mine the `Block`.
    fn mine_block(&mut self, difficulty: usize) {
        let mut hash = String::new();
        let mut nonce: usize = 0;

        while !self.is_valid_hash(difficulty, &hash) {
            let mut temp_hash_string = self.get_hash_string();
            temp_hash_string.push_str(&nonce.to_string());

            hash = Block::get_hash(&temp_hash_string);
            nonce += 1;
        }

        self.block_hash = hash;
        self.nonce = nonce;
    }

    /// Initialize a new `Block`.
    pub fn new(data: &str, difficulty: usize, index: usize, previous_hash: &str) -> Block {
        let mut new_block = Block{
            index: index,
            block_hash: "".to_string(),
            data: String::from(data),
            timestamp: Local::now().format("%m-%d-%Y %H:%M:%S").to_string(),
            previous_hash: String::from(previous_hash),
            nonce: 0
        };

        if index != 0 {
            println!("Calculating valid hash for block {}...", index);
        } else {
            println!("Calculating valid hash for genesis block...");
        }
        new_block.mine_block(difficulty);
        println!("Done.\n");

        new_block
    }

    /// Display `Block` contents.
    pub fn display(&self) {
        if self.index == 0 {
            println!("\nGENESIS BLOCK");
        } else {
            println!("BLOCK {}", self.index);
        }

        println!(r#"
hash:          {}
data:          {:?}
index:         {}
nonce:         {}
previous hash: {}
timestamp:     {}
        "#, 
            self.block_hash, 
            self.data, 
            self.index, 
            self.nonce,
            self.previous_hash,
            self.timestamp, 
        );
    }
}

#[cfg(test)]
mod test_block {
    use super::*;

    #[test]
    fn test_get_hash_string() {
        let block = Block::new("test", 1, 0, &"0".repeat(64));
        let hash_string = block.get_hash_string();

        assert_eq!(hash_string.len(), 88);
    }

    #[test]
    fn test_is_valid_hash() {
        let block = Block::new("test", 1, 0, &"0".repeat(64));

        assert_eq!(block.is_valid_hash(1, "012345"), true);
        assert_eq!(block.is_valid_hash(1, "123456"), false);
    }

    #[test]
    fn test_get_hash() {
        let test_string = "test";
        let test_hash = Block::get_hash(test_string);

        assert_eq!(test_hash.len(), 64);
    }

    #[test]
    fn test_mine_block() {
        let block = Block::new("test", 1, 0, &"0".repeat(64));

        assert_ne!(block.block_hash, "");
        assert_ne!(block.nonce, 0);
    }

    #[test]
    fn test_new() {
        let block = Block::new("test", 1, 0, &"0".repeat(64));

        assert_eq!(block.block_hash.len(), 64);
        assert_eq!(block.data, "test");
        assert_eq!(block.index, 0);
        assert!(block.nonce > 0);
        assert_eq!(block.previous_hash.len(), 64);
        assert_eq!(block.timestamp.len(), 19);
    }
}
