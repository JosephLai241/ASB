# blockchain-rs

A simple blockchain implementation in Rust for my own reference.

# Table of Contents

* [Block](#block)
* [Blockchain](#blockchain)

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
```