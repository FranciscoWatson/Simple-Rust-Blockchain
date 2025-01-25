use super::block::Block;
use crate::transactions::Transaction;
use std::{collections::HashMap, fmt};
use rand::Rng;


pub struct Blockchain {
    pub chain: Vec<Block>,
    pub balances: HashMap<String, u64>,
    pub validators: HashMap<String, u64>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, vec![], "0");

        Self {
            chain: vec![genesis_block],
            balances: HashMap::new(),
            validators: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>){

        if !self.validate_transactions(&transactions) {
            println!("Block rejected due to invalid transactions.");
            return;
        }

        let validator = match self.select_validator() {
            Some(v) => v,
            None => {
                println!("No valid validator found.");
                return;
            }
        };
        
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

        let reward = 100;
        *self.balances.entry(validator.clone()).or_insert(0) += reward;

        let new_block = BlockFactory::create_block(&self.chain, transactions);

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

    pub fn add_validator(&mut self, validator_name: &str, stake: u64) -> bool {
        if let Some(balance) = self.balances.get_mut(validator_name) {
            if *balance >= stake {
                *balance -= stake;
                self.validators.insert(validator_name.to_string(), stake);
                println!("Validator '{}' added with stake {}", validator_name, stake);
                return true;
            } else {
                println!("Insufficient balance for staking.");
            }
        } else {
            println!("Account '{}' does not exist.", validator_name);
        }
        false
    }

    pub fn select_validator(&self) -> Option<String> {
        if self.validators.is_empty() {
            println!("No validators available.");
            return None;
        }

        let total_stake: u64 = self.validators.values().sum();
        let mut rng = rand::thread_rng();
        let mut selected_value = rng.gen_range(0..total_stake);

        for (account, stake) in &self.validators {
            if selected_value < *stake {
                return Some(account.clone());
            }
            selected_value -= stake;
        }

        None
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

pub struct BlockFactory;

impl BlockFactory{
    pub fn create_block(chain: &Vec<Block>, transactions: Vec<Transaction>) -> Block {
        let previous_block = chain.last().unwrap();
        let chain_len = chain.len() as u64;
        let new_block = Block::new(chain_len, transactions, &previous_block.hash);
        new_block
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    
    use crate::{blockchain, transactions::Transaction};

    #[test]
    fn test_create_account(){
        let mut blockchain = Blockchain::new();

        blockchain.create_account("Alice", 100);

        assert_eq!(blockchain.balances.get("Alice").unwrap(), &100);
    }

    #[test]
    fn test_create_account_already_exists(){
        let mut blockchain = Blockchain::new();

        blockchain.create_account("Alice", 100);
        blockchain.create_account("Alice", 1001);

        assert_eq!(blockchain.balances.get("Alice").unwrap(), &100);
    }

    #[test]
    fn test_add_block(){
        let mut blockchain = Blockchain::new();

        blockchain.create_account("Alice", 100);
        blockchain.create_account("Bob", 100);
        blockchain.create_account("Charlie", 100);

        blockchain.add_validator("Charlie", 20);

        let tx = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 50,
        };

        blockchain.add_block(vec![tx]);

        assert_eq!(blockchain.balances.get("Alice").unwrap(), &50);
        assert_eq!(blockchain.balances.get("Bob").unwrap(), &150);
        assert_eq!(blockchain.balances.get("Charlie").unwrap(), &180);
    }

    #[test]
    fn test_add_block_insufficient_funds(){
        let mut blockchain = Blockchain::new();

        blockchain.create_account("Alice", 100);
        blockchain.create_account("Bob", 100);
        blockchain.create_account("Charlie", 100);

        blockchain.add_validator("Charlie", 20);


        let tx = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 150,
        };

        blockchain.add_block(vec![tx]);

        assert_eq!(blockchain.balances.get("Alice").unwrap(), &100);
        assert_eq!(blockchain.balances.get("Bob").unwrap(), &100);
        assert_eq!(blockchain.balances.get("Charlie").unwrap(), &80);
    }

    #[test]
    fn test_is_chain_valid(){
        let mut blockchain = Blockchain::new();

        blockchain.create_account("Alice", 100);
        blockchain.create_account("Bob", 100);
        blockchain.create_account("Charlie", 100);

        blockchain.add_validator("Charlie", 20);

        let tx1 = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 50,
        };
        let tx2 = Transaction {
            sender: "Bob".to_string(),
            receiver: "Alice".to_string(),
            amount: 5,
        };
        let tx3 = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 20,
        };
        let tx4 = Transaction {
            sender: "Bob".to_string(),
            receiver: "Alice".to_string(),
            amount: 5,
        };

        blockchain.add_block(vec![tx1, tx2]);
        blockchain.add_block(vec![tx3, tx4]);

        assert_eq!(blockchain.is_chain_valid(), true);
    }
}