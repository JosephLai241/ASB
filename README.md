# ASB

A Simple Blockchain. Written in Rust.

I will be needing a similar data structure for a future Rust project, so I figured I should create a simple blockchain implementation to get familiar with it.

# Table of Contents

* [The Implementation](#the-implementation)
    + [`block.rs`](#blockrs)
        * [Calculating a `Block`'s SHA256 Hash](#calculating-a-blocks-sha256-hash)
    + [`blockchain.rs`](#blockchainrs)
* [Running `asb`](#running-asb)
    + [Examples](#examples)
        * [5 Blocks With a Difficulty of 5](#5-blocks-with-a-difficulty-of-5)
        * [3 Blocks With a Difficulty of 6](#3-blocks-with-a-difficulty-of-6)
        * [6 Blocks With a Difficulty of 3](#6-blocks-with-a-difficulty-of-3)

# The Implementation

## `block.rs`

Each `Block` within the `Blockchain` contains:

* The SHA256 hash of the `Block`
* Data - in this case a `String`, for simplicity's sake
* An index indicating the block's position within the `Blockchain`
* A nonce - a one-use number generated by the Proof of Work algorithm
* The SHA256 hash of the previous `Block` in the `Blockchain`
* A timestamp indicating the `Block`'s inception

This is the `struct` definition:

```rust
#[derive(Debug)]
pub struct Block {
    pub block_hash: String,
    data: String,
    index: usize,
    nonce: usize,
    previous_hash: String,
    timestamp: String,
}
```

`block_hash` is `pub` since it will be stored in the `Blockchain`'s `transactions` `Vec` when the `Block` is added.

### Calculating a `Block`'s SHA256 Hash

I then defined some methods to calculate the hash of each `Block`. Let's go over them one by one.

```rust
impl Block {
    fn is_valid_hash(&self, difficulty: usize, hash: &str) -> bool {
        let pattern = &"0".repeat(difficulty);

        hash.starts_with(pattern)
    }
```

`is_valid_hash()` checks whether the `hash` begins with repeating `0`s. `0` is repeated `n` times, depending on the `difficulty` value. This method then returns a boolean value depending on whether `hash` matches the prefix defined by `pattern`.

```rust
    fn get_hash_string(&self) -> String {
        let mut hash_string = self.index.to_string();
        hash_string.push_str(&self.data);
        hash_string.push_str(&self.timestamp);
        hash_string.push_str(&self.previous_hash);

        hash_string
    }
```

To calculate the hash, all data excluding the `block_hash` and `nonce` contained within the `Block` is concatenated into a `String`. `get_hash_string()` does exactly this.

```rust
// Imports at the top of `block.rs`.
use data_encoding::HEXLOWER;
use ring::digest::{
    Context,
    SHA256
};

    fn get_hash(hash_string: &str) -> String {
        let mut context = Context::new(&SHA256);
        context.update(hash_string.to_string().as_bytes());

        HEXLOWER.encode(context.finish().as_ref())
    }
```

`get_hash()` then takes the `String` returned from `get_hash_string()` and calculates the hash. This is returned in the form of a `String`.

```rust
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

```

`mine_block()` is where the [Proof of Work][Proof of Work] happens. 

I initialize the `hash` to an empty `String`, and the `nonce` to `0`. The `hash` is continuously checked against the pattern by calling the previously defined `is_valid_hash()` method. 

While the hash does not match the correct pattern, a new hash is calculated by concatenating the incremented `nonce` with the target string returned by `get_hash_string()`.

If the hash matches the correct pattern, the `block_hash` and `nonce` are set within the `Block`.

```rust
// Import at the top of `block.rs`.
use chrono::prelude::Local;

    pub fn new(data: &str, difficulty: usize, index: usize, previous_hash: &str) -> Block {
        let mut new_block = Block{
            index: index,
            block_hash: "".to_string(),
            data: String::from(data),
            timestamp: Local::now().format("%m-%d-%Y %H:%M:%S").to_string(),
            previous_hash: String::from(previous_hash),
            nonce: 0
        };
        new_block.mine_block(difficulty);

        new_block
    }
}
```

`new()` will return a new `Block` after calculating its `block_hash` and `nonce` by calling the previously defined `mine_block()` method.

## `blockchain.rs`

A `Blockchain` contains:

* A `Vec` which holds `Block`s
* A difficulty level which is used when calculating the `nonce`/Proof of Work
* A `Vec` which holds the SHA256 hashes of each `Block`

This is the `struct` definition:

```rust
// Import at the top of `blockchain.rs`.
use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    difficulty: usize,
    transactions: Vec<String>
}
```

I then defined some methods to initialize a new `Blockchain`. Let's go over them one by one.

```rust
impl Blockchain {
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
```

`new()` creates a new `Blockchain` with a genesis block. The genesis block is the first `Block` in the `chain` `Vec`. Its hash is also the first hash in the `transactions` `Vec`.

```rust
    pub fn add_block(&mut self, data: String) {
        let last_block = &self.chain[self.chain.len() - 1];
        let previous_hash = &last_block.block_hash;

        let new_block = Block::new(&data, self.difficulty, self.chain.len(), previous_hash);
        self.transactions.push(new_block.block_hash.clone().to_string());
        self.chain.push(new_block);
    }
}
```

I just defined one more method, `add_block()`, which will obviously add a new `Block` to the `Blockchain`. The new `Block`'s `block_hash` and `nonce` are calculated based on the `difficulty` value set in the `Blockchain` struct.

The new `Block` and its hash is then `push`ed onto the `chain` and `transactions` `Vec`s respectively. 

# Running `asb`

You can play around with this demo by changing these two values defined in `main.rs`:

```rust
let difficulty = 5;
let total_blocks = 5;
```

> Above is the default configuration.

You will notice time spent performing the Proof of Work algorithm drastically increases with the `difficulty` level.

After modifying the values, you can just run the unoptimized binary with your changes:

```
cargo run
```

But I recommend re-compiling and running the optimized version:

```
cargo build --release
./target/release/asb
```

Re-compiling only takes about a second after the initial build. It is also *much* faster, to no surprise.

## Examples

### 5 Blocks With a Difficulty of 5

Output:

```

INITIALIZING BLOCKCHAIN
=======================

SETTING DIFFICULTY LEVEL OF 5

Creating genesis block
Calculating valid hash for genesis block...
Done.

ADDING 4 BLOCKS TO THE BLOCKCHAIN

Adding block #1
Calculating valid hash for block 1...
Done.

Adding block #2
Calculating valid hash for block 2...
Done.

Adding block #3
Calculating valid hash for block 3...
Done.

Adding block #4
Calculating valid hash for block 4...
Done.


DISPLAYING BLOCKCHAIN METADATA
==============================

TRANSACTIONS
------------

SHA256: 000002ef9b8faa31df2ffca79ea7a40e8fb99bf6d0218fc41af2a9460b69cae9
SHA256: 00000dc727c2ad0d4e0651f04697729c19f3bfeaf0fde13d22d138390c032b2f
SHA256: 000004c1f922033e6cdc2e81b1786a5e2f9c056b7720eebf922bbff257815611
SHA256: 00000d693810e04371b7b7d78cd0decee2c69def64960fca5bfa5109cb05ea9c
SHA256: 000007378b30eb3af6fbbf79a0b730eccae2a7e94af2a59b7ede3f1538eafdf1

BLOCKS
------

GENESIS BLOCK

hash:          000002ef9b8faa31df2ffca79ea7a40e8fb99bf6d0218fc41af2a9460b69cae9
data:          "Genesis Block"
index:         0
nonce:         246429
previous hash: 0000000000000000000000000000000000000000000000000000000000000000
timestamp:     07-23-2021 03:47:23
        
BLOCK 1

hash:          00000dc727c2ad0d4e0651f04697729c19f3bfeaf0fde13d22d138390c032b2f
data:          "BLOCK 1"
index:         1
nonce:         35498
previous hash: 000002ef9b8faa31df2ffca79ea7a40e8fb99bf6d0218fc41af2a9460b69cae9
timestamp:     07-23-2021 03:47:23
        
BLOCK 2

hash:          000004c1f922033e6cdc2e81b1786a5e2f9c056b7720eebf922bbff257815611
data:          "BLOCK 2"
index:         2
nonce:         3279400
previous hash: 00000dc727c2ad0d4e0651f04697729c19f3bfeaf0fde13d22d138390c032b2f
timestamp:     07-23-2021 03:47:23
        
BLOCK 3

hash:          00000d693810e04371b7b7d78cd0decee2c69def64960fca5bfa5109cb05ea9c
data:          "BLOCK 3"
index:         3
nonce:         941862
previous hash: 000004c1f922033e6cdc2e81b1786a5e2f9c056b7720eebf922bbff257815611
timestamp:     07-23-2021 03:47:28
        
BLOCK 4

hash:          000007378b30eb3af6fbbf79a0b730eccae2a7e94af2a59b7ede3f1538eafdf1
data:          "BLOCK 4"
index:         4
nonce:         2277566
previous hash: 00000d693810e04371b7b7d78cd0decee2c69def64960fca5bfa5109cb05ea9c
timestamp:     07-23-2021 03:47:30
        
FINISHED CREATING BLOCKCHAIN WITH 5 BLOCKS IN 10 SECONDS.

```

### 3 Blocks With a Difficulty of 6

Configuration:

```rust
let difficulty = 6;
let total_blocks = 3;
```

Output:

```

INITIALIZING BLOCKCHAIN
=======================

SETTING DIFFICULTY LEVEL OF 6

Creating genesis block
Calculating valid hash for genesis block...
Done.

ADDING 2 BLOCKS TO THE BLOCKCHAIN

Adding block #1
Calculating valid hash for block 1...
Done.

Adding block #2
Calculating valid hash for block 2...
Done.


DISPLAYING BLOCKCHAIN METADATA
==============================

TRANSACTIONS
------------

SHA256: 000000f915a0ed32185029b7c740a7113a5535ffc63b669fe1f392faec95ce22
SHA256: 000000ff34769d20b4ea5ed87dd6ab9758796c9255424b4d3b57856a3d3a73bd
SHA256: 00000051fc46f747d87944feb22401fa401398e2fdabda0a13e96b638336c435

BLOCKS
------

GENESIS BLOCK

hash:          000000f915a0ed32185029b7c740a7113a5535ffc63b669fe1f392faec95ce22
data:          "Genesis Block"
index:         0
nonce:         25611139
previous hash: 0000000000000000000000000000000000000000000000000000000000000000
timestamp:     07-23-2021 03:49:20
        
BLOCK 1

hash:          000000ff34769d20b4ea5ed87dd6ab9758796c9255424b4d3b57856a3d3a73bd
data:          "BLOCK 1"
index:         1
nonce:         4520028
previous hash: 000000f915a0ed32185029b7c740a7113a5535ffc63b669fe1f392faec95ce22
timestamp:     07-23-2021 03:50:01
        
BLOCK 2

hash:          00000051fc46f747d87944feb22401fa401398e2fdabda0a13e96b638336c435
data:          "BLOCK 2"
index:         2
nonce:         50128421
previous hash: 000000ff34769d20b4ea5ed87dd6ab9758796c9255424b4d3b57856a3d3a73bd
timestamp:     07-23-2021 03:50:08
        
FINISHED CREATING BLOCKCHAIN WITH 3 BLOCKS IN 123 SECONDS.

```

### 6 Blocks With a Difficulty of 3

Configuration:

```rust
let difficulty = 3;
let total_blocks = 6;
```

Output:

```

INITIALIZING BLOCKCHAIN
=======================

SETTING DIFFICULTY LEVEL OF 3

Creating genesis block
Calculating valid hash for genesis block...
Done.

ADDING 5 BLOCKS TO THE BLOCKCHAIN

Adding block #1
Calculating valid hash for block 1...
Done.

Adding block #2
Calculating valid hash for block 2...
Done.

Adding block #3
Calculating valid hash for block 3...
Done.

Adding block #4
Calculating valid hash for block 4...
Done.

Adding block #5
Calculating valid hash for block 5...
Done.


DISPLAYING BLOCKCHAIN METADATA
==============================

TRANSACTIONS
------------

SHA256: 000b112ffe3d23ab19717397ca86eb9cea1c4961c92e3c8e354a6a2ce21d7e6e
SHA256: 000a22a4c8f82fd5036213b67827d39d334670cfe6f83933ade1ec816984368a
SHA256: 000788050d9eb1e4ad38d00497842733375737a86dd0af7ce6f6c6562b3b7e93
SHA256: 00019fffd8991621d28e51a68c603088070f917894e81d73d54cac0e42eda03a
SHA256: 0004900e7daa43cf8b9b914e7056e0353aead7accc0f935a7920d790c599c466
SHA256: 00012b9f667c95cd3abc9bb879f5b2a0c7de2fac5681825e508e3eb210058396

BLOCKS
------

GENESIS BLOCK

hash:          000b112ffe3d23ab19717397ca86eb9cea1c4961c92e3c8e354a6a2ce21d7e6e
data:          "Genesis Block"
index:         0
nonce:         5439
previous hash: 0000000000000000000000000000000000000000000000000000000000000000
timestamp:     07-23-2021 03:53:13
        
BLOCK 1

hash:          000a22a4c8f82fd5036213b67827d39d334670cfe6f83933ade1ec816984368a
data:          "BLOCK 1"
index:         1
nonce:         5361
previous hash: 000b112ffe3d23ab19717397ca86eb9cea1c4961c92e3c8e354a6a2ce21d7e6e
timestamp:     07-23-2021 03:53:13
        
BLOCK 2

hash:          000788050d9eb1e4ad38d00497842733375737a86dd0af7ce6f6c6562b3b7e93
data:          "BLOCK 2"
index:         2
nonce:         3743
previous hash: 000a22a4c8f82fd5036213b67827d39d334670cfe6f83933ade1ec816984368a
timestamp:     07-23-2021 03:53:13
        
BLOCK 3

hash:          00019fffd8991621d28e51a68c603088070f917894e81d73d54cac0e42eda03a
data:          "BLOCK 3"
index:         3
nonce:         202
previous hash: 000788050d9eb1e4ad38d00497842733375737a86dd0af7ce6f6c6562b3b7e93
timestamp:     07-23-2021 03:53:13
        
BLOCK 4

hash:          0004900e7daa43cf8b9b914e7056e0353aead7accc0f935a7920d790c599c466
data:          "BLOCK 4"
index:         4
nonce:         79
previous hash: 00019fffd8991621d28e51a68c603088070f917894e81d73d54cac0e42eda03a
timestamp:     07-23-2021 03:53:13
        
BLOCK 5

hash:          00012b9f667c95cd3abc9bb879f5b2a0c7de2fac5681825e508e3eb210058396
data:          "BLOCK 5"
index:         5
nonce:         197
previous hash: 0004900e7daa43cf8b9b914e7056e0353aead7accc0f935a7920d790c599c466
timestamp:     07-23-2021 03:53:13
        
FINISHED CREATING BLOCKCHAIN WITH 6 BLOCKS IN 0 SECONDS.

```

<!-- LINKS -->
[Proof of Work]: https://en.wikipedia.org/wiki/Proof_of_work
