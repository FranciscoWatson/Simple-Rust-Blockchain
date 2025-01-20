use std::fmt;

#[derive(Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction: {} -> {} | Amount: {}",
            self.sender, self.receiver, self.amount
        )
    }
}