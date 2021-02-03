use crate::transaction;
use sha2::{ Sha256, Digest };
use std::fmt;

pub struct Block {

    block_hash: Vec<u8>,
    previous_block_hash: Vec<u8>,
    nonce: u128,

    transactions: Vec<transaction::Transaction>
}


impl Block {
    pub fn new_block(nonce: u128, previous_hash: &Vec<u8>) -> Block {
        Block {
            block_hash: Vec::new(),
            previous_block_hash: previous_hash.iter().cloned().collect(),
            transactions: Vec::new(),
            nonce
        }
    }

    pub fn add_transaction(&mut self, transaction: transaction::Transaction) {
        self.transactions.push(transaction);
        let mut sha_hasher = Sha256::default(); 

        for tran in &self.transactions {
            let mut tran_from = tran.get_from();
            tran_from.push_str(&tran.get_to());
            tran_from.push_str(&tran.get_tokens().to_string());
            sha_hasher.update(tran_from);
        }

        self.block_hash = Digest::finalize(sha_hasher).iter().cloned().collect();
    }

    pub fn get_transactions_count(&self) -> u32 {
        self.transactions.len() as u32
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "Prev Block Hash: ");
        for byte in &self.previous_block_hash {
            write!(f, "{:x}", byte);
        }

        write!(f, "\nBlock Hash: ");
        for byte in &self.block_hash {
            write!(f, "{:x}", byte);
        }

        write!(f, "\nBlock Nonce: {:x}", self.nonce)
    }
}