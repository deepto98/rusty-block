use super::*;
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Blockhash,
    pub prev_block_hash: Blockhash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block")
    }
}
// Implements functions for Block
impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Blockhash,
        nonce: u64,
        payload: String,
    ) -> Self //Returns Block Type
    {
        Block {
            index,
            timestamp,
            hash : vec![0;32], //repeats 0 32 times i.e. 32 bytes int
            prev_block_hash,
            nonce,
            payload,
        }
    }
}
