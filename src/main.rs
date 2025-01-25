mod blockchain;
mod transactions;

use blockchain::Blockchain;
use transactions::Transaction;


fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.create_account("Alice", 100);
    blockchain.create_account("Bob", 100);

    blockchain.add_validator("Alice", 1);

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


    println!("{}", blockchain);

    println!("\nBalances after Txs.");
    for (account, balance) in &blockchain.balances {
        println!("{}: {}", account, balance);
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