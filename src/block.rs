use crate::hashable::Hashable;

use super::*;
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Blockhash,
    pub prev_block_hash: Blockhash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]:{} at: {} with: {} transaction(s) and nonce:{}", //define format in which to print block
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce
        )
    }
}
// Implements functions for Block
impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Blockhash,
        nonce: u64,
        transactions: Vec<Transaction>,
        difficulty: u128,
    ) -> Self //Returns Block Type
    {
        Block {
            index,
            timestamp,
            hash: vec![0; 32], //repeats 0 32 times i.e. 32 bytes int
            prev_block_hash,
            nonce,
            transactions,
            difficulty,
        }
    }

    pub fn mine(&mut self) {
        //  Loop through every u64 int
        // Then generate a hash and check if the difficulty meets the bar
        // Computationally expensive
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    //Hashing elements of Block
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes // return
    }
}

pub fn check_difficulty(hash: &Blockhash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash) //returns t/f
}
