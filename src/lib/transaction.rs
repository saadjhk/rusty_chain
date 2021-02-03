pub enum TransactionType {
    SendTokens
}

pub struct Transaction {
    from: String,
    to: String,
    tokens: u128,
    transaction_type: TransactionType
}

impl Transaction {

    pub fn get_from(&self) -> String {
        self.from.clone()
    }

    pub fn get_to(&self) -> String {
        self.to.clone()
    }

    pub fn get_tokens(&self) -> u128 {
        self.tokens
    }

    pub fn get_type(&self) -> &TransactionType {
        &self.transaction_type
    }

    pub fn create_new_transaction(from: String, to: String, tokens: u128) -> Transaction {
        Transaction {
            from,
            to,
            tokens,
            transaction_type: TransactionType::SendTokens
        }
    }
}