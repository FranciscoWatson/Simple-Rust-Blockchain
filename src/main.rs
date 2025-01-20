mod blockchain;
mod transactions;

use blockchain::Blockchain;
use transactions::Transaction;


fn main() {
    let mut blockchain = Blockchain::new();

    let tx1 = Transaction {
        sender: "Alice".to_string(),
        receiver: "Bob".to_string(),
        amount: 50,
    };
    let tx2 = Transaction {
        sender: "Bob".to_string(),
        receiver: "Charlie".to_string(),
        amount: 30,
    };
    
    blockchain.add_block(vec![tx1, tx2]);

    println!("{}", blockchain);

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