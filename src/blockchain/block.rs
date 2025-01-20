use chrono::prelude::*;
use sha2::{Digest, Sha256};
use std::fmt;


use crate::transactions::Transaction;

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64, // Unix timestamp
    pub transactions: Vec<Transaction>, // Transaction list
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: &str) -> Self {
        let timestamp = Utc::now().timestamp() as u64;
        let hash = Self::calculate_hash(index, timestamp, &transactions, previous_hash);
        Self {
            index,
            timestamp,
            transactions,
            previous_hash: previous_hash.to_string(),
            hash,
        }
    }
    pub fn calculate_hash(index: u64, timestamp: u64, transactions: &Vec<Transaction>, previous_hash: &str) -> String {
        let tx_data: String = transactions
            .iter()
            .map(|tx| format!("{}{}{}", tx.sender, tx.receiver, tx.amount))
            .collect(); // String with all tx
        let input = format!("{}{}{}{}", index, timestamp, tx_data, previous_hash); // Concatenate all the input values
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize()) // Return the hash as a hex string
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block {}\nTimestamp: {}\nPrevious Hash: {}\nHash: {}\nTransactions:\n{}",
            self.index,
            self.timestamp,
            self.previous_hash,
            self.hash,
            self.transactions
                .iter()
                .map(|tx| format!("{}", tx))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}