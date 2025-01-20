use chrono::prelude::*;
use sha2::{Digest, Sha256};


fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("Transaction Data");

    for block in &blockchain.chain {
        println!("{:?}", block)
    }

    // TO-DO, read blockchain from a file.
    audit_blockchain(&blockchain);
}

fn audit_blockchain(blockchain: &Blockchain) {
    if blockchain.is_chain_valid() {
        println!("Chain is valid.");
    } else {
        println!("Chain is invalid.");
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
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block", "0");
        Self {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: &str){
        let previous_block = self.chain.last().unwrap();
        let chain_len = self.chain.len() as u64;
        let new_block = Block::new(chain_len, data, &previous_block.hash);
        self.chain.push(new_block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {          
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            
            if current_block.hash != Block::calculate_hash(current_block.index, current_block.timestamp, &current_block.data, &current_block.previous_hash) {
                println!("Invalid hash in block {}", current_block.index);
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                println!("Previous hash isn't equal {}", current_block.index);
                return false;
            }
        }
        true
    }
}