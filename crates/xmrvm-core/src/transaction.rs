use sha2::{Sha256, Digest};
pub struct Transaction {
    pub id: u64,
    pub tx_type: TxType,
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub signature: u64,
}

impl Transaction {
    pub fn new (
        sender: impl Into<String>,
        
    )