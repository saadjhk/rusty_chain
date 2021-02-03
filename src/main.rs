#[path = "lib/block.rs"] pub mod block;
#[path = "lib/transaction.rs"] pub mod transaction;

use sha2::{ Sha256, Digest };
fn main() {
    let hasher = Sha256::default();
    let first_hash = Digest::finalize(hasher);
    let mut gen_block = block::Block::new_block(10, &first_hash.iter().cloned().collect());

    println!("{}", gen_block);


    let n_transaction = transaction::Transaction::create_new_transaction(String::from("Saad"), String::from("Ahmed"), 8);
    gen_block.add_transaction(n_transaction);
    println!("{}", gen_block);
    
}
