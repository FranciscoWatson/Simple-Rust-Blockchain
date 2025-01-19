use chrono::prelude::*;
use sha2::{Digest, Sha256};


fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("Transaction Data");

    for block in &blockchain.blocks {
        println!("{:?}", block)
    }

}

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u64, // Unix timestamp
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    pub fn new(index: u64, data: &str, previous_hash: &str) -> Self {
        let timestamp = Utc::now().timestamp() as u64;
        let hash = Self::calculate_hash(index, timestamp, data, previous_hash);
        Self {
            index,
            timestamp,
            data: data.to_string(),
            previous_hash: previous_hash.to_string(),
            hash,
        }
    }
    fn calculate_hash(index: u64, timestamp: u64, data: &str, previous_hash: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, data, previous_hash); // Concatenate all the input values
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize()) // Return the hash as a hex string
    }
}

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block", "0");
        Self {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: &str){
        let previous_block = self.blocks.last().unwrap();
        let chain_len = self.blocks.len() as u64;
        let new_block = Block::new(chain_len, data, &previous_block.hash);
        self.blocks.push(new_block);
    }
}