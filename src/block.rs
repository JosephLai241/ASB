// A block in the blockchain.

use data_encoding::HEXLOWER;
use chrono::prelude::Local;
use ring::digest::{
    Context,
    SHA256
};

// Block struct.
#[derive(Debug)]
pub struct Block {
    pub block_hash: String,
    pub data: Vec<u8>,
    pub timestamp: String,
    pub previous_hash: String,
}

// Block operations.
impl Block {
    // Calculate the hash for the current block.
    pub fn get_hash(hash_string: String) -> String {
        let mut context = Context::new(&SHA256);
        context.update(&hash_string.to_string().as_bytes());

        HEXLOWER.encode(context.finish().as_ref())
    }

    // Initialize a new block.
    pub fn new(data: &str, previous_hash: &str) -> Block {
        let timestamp: &str = &Local::now().format("%m-%d-%Y %H:%M:%S").to_string();

        let mut hash_string = "0".to_owned();
        hash_string.push_str(data);
        hash_string.push_str(timestamp);
        hash_string.push_str(previous_hash);

        let block_hash = Block::get_hash(hash_string);

        Block{
            block_hash: block_hash,
            data: data.as_bytes().to_vec(),
            timestamp: String::from(timestamp),
            previous_hash: String::from(previous_hash)
        }
    }

    // Display block contents.
    pub fn display(&self) {
        println!(r#"
BLOCK

hash:          {}
data:          {:?}
timestamp:     {}
previous hash: {}
        "#, self.block_hash, self.data, self.timestamp, self.previous_hash);
    }
}

#[cfg(test)]
mod test_block {
    use super::*;

    #[test]
    fn test_get_hash() {
        
    }

    #[test]
    fn test_new() {
        
    }
}
