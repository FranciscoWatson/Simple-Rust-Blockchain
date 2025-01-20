use super::block::Block;
use crate::transactions::Transaction;
use std::{collections::HashMap, fmt};

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub balances: HashMap<String, u64>
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, vec![], "0");

        Self {
            chain: vec![genesis_block],
            balances: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>){

        if !self.validate_transactions(&transactions) {
            println!("Block rejected due to invalid transactions.");
            return;
        }
        
        for transaction in &transactions {
            let sender_balance = self.balances.get(&transaction.sender).copied().unwrap_or(0);
            if sender_balance < transaction.amount {
            println!("Insufficient funds for transaction from '{}' to '{}'.", transaction.sender, transaction.receiver);
            return;
            }

            let receiver_balance = self.balances.get(&transaction.receiver).copied().unwrap_or(0);

            self.balances.insert(transaction.sender.clone(), sender_balance - transaction.amount);
            self.balances.insert(transaction.receiver.clone(), receiver_balance + transaction.amount);
        }

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

    pub fn create_account(&mut self, account_name: &str, initial_balance: u64) -> bool {
        if self.balances.contains_key(account_name) {
            println!("Account '{}' already exists.", account_name);
            return false;
        }
        self.balances.insert(account_name.to_string(), initial_balance);
        println!("Account '{}' succesfully created with a balance of '{}'.", account_name, initial_balance);
        return  true;
    }

    fn validate_transactions(&self, transactions: &Vec<Transaction>) -> bool {
        for transaction in transactions {
            let sender_balance = self.balances.get(&transaction.sender).copied().unwrap_or(0);
            if sender_balance < transaction.amount {
                println!("Insufficient funds for transaction from '{}' to '{}'.", transaction.sender, transaction.receiver);
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.chain
                .iter()
                .map(|block| format!("{}", block))
                .collect::<Vec<String>>()
                .join("\n\n")
        )
    }
}