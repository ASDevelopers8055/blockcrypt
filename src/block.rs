use super::*;

/// Checks whether the hash is greater than the target. If true, the hash given as parameter is valid.
pub fn difficulty (hash: &Vec<u8>, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}

/// The Block structure
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Block {
    /// Creates a new block with the given parameters.
    pub fn new (index: u32, timestamp: u128, prev_block_hash: Vec<u8>, transactions: Vec<Transaction>, difficulty: u128) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty,
        }
    }

    /// Mines the block.
    pub fn mine (&mut self) {
        for i in 0..(u64::max_value()) {
            self.nonce = i;
            let hash = self.hash();
            if difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hash for Block {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>()
        );
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}
