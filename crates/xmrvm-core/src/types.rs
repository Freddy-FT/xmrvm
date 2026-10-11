pub struct Account {
    pub id: u64,
    pub amount: u64,
    pub nonce: u64,
}

pub struct Transaction {
    pub from: u64,
    pub to: u64,
    pub amount: u64,
    pub nonce: u64,
}
pub struct Block {
    pub header: BlockHeader,
    pub body: BlockBody,
}
pub struct BlockBody {
    // Block body fields
    pub transactions: Vec<Transaction>,
}
pub struct BlockHeader {
    pub version: u64,
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
    pub previous_hash: [u8; 32],
    pub nonce: u64,
    pub bits: u64,
}