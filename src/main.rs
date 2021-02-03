#[path = "lib/block.rs"] pub mod block;
#[path = "lib/transaction.rs"] pub mod transaction;
#[path = "lib/network.rs"] pub mod network;

use sha2::{ Sha256, Digest };
fn main() {
    let mut hasher = Sha256::default();
    hasher.update("SIMPLE DUMB BLOCK CHAIN");
    let first_hash = Digest::finalize(hasher);
    let mut gen_block = block::Block::new_block(10, &first_hash.iter().cloned().collect());
    println!("{}", gen_block);

    let n_transaction = transaction::Transaction::create_new_transaction(String::from("Saad"), String::from("Ahmed"), 8);
    gen_block.add_transaction(n_transaction);
    println!("{}", gen_block);
}

pub fn add_new_transaction(curr_block: &mut block::Block, transaction: transaction::Transaction) {
    if curr_block.get_transactions_count() > 50 {
        // Some logic
    } else {
        curr_block.add_transaction(transaction);
    }
}