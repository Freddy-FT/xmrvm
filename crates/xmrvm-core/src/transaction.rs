use std::fmt::format;
use sha2::{Sha256, Digest};
use rand::random;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxType {
    Coinbase,
    Transfer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub id: String,
    pub tx_type: TxType,
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub fee: u64,
    pub pub_key: Option<String>,
    pub signature: Option<String>,
}

impl Transaction {
    pub fn new(
        sender: impl Into<String>,
        recipient: impl Into<String>,
        amount: u64,
        fee: u64,
    ) -> Self {
        let sender = sender.into();
        let recipient = recipient.into();
        let id = compute_id(&sender, &recipient, amount);
        Transaction { 
            id, 
            tx_type: TxType::Transfer, 
            sender, 
            recipient, 
            amount, 
            fee, 
            pub_key: None, 
            signature: None, 
        }
    }

    pub fn compute_id(&self) -> String {
        compute_id(&self.sender, &self.recipient, self.amount)
    }
}

pub fn compute_id(sender: &str, recipient: &str, amount: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(sender.as_bytes());
    hasher.update(recipient.as_bytes());
    hasher.update(amount.to_string().as_bytes());
    format!("{:x}", hasher.finalize())
}