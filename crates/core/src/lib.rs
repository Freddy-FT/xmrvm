pub struct Account {
    id: u64,
    amount: u64,
    nonce: u64,
}

pub struct Transaction {
    from: u64,
    to: u64,
    amount: u64,
}

pub struct Block {
    // Block header fields
    version: u64,
    merkle_root: String,
    timestamp: u64,
    previous_hash: String,
    hash: String,
    nonce: u64,
    bits: u64,

    // Block body fields
    transaction_count: u64,
    transactions: Vec<Transaction>,
}
