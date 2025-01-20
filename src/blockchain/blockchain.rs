use super::block::Block;
use crate::transactions::Transaction;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, vec![], "0");
        Self {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>){
        let previous_block = self.chain.last().unwrap();
        let chain_len = self.chain.len() as u64;
        let new_block = Block::new(chain_len, transactions, &previous_block.hash);
        self.chain.push(new_block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {          
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            
            if current_block.hash != Block::calculate_hash(current_block.index, current_block.timestamp, &current_block.transactions, &current_block.previous_hash) {
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