# ASB

A Simple Blockchain.

# Table of Contents

* [Why?](#why)
* [Block](#block)
* [Blockchain](#blockchain)

# Why?

I will be needing a similar data structure for a future Rust project, so I figured I should create a simple blockchain implementation to get familiar with it.

# Block

```rust
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
}
```

# Blockchain

```rust
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
}
```