use crate::hashable::Hashable;

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
        write!(
            f,
            "Block[{}]:{} at: {} with: {}", //define format in which to print block
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload
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
        payload: String,
    ) -> Self //Returns Block Type
    {
        Block {
            index,
            timestamp,
            hash: vec![0; 32], //repeats 0 32 times i.e. 32 bytes int
            prev_block_hash,
            nonce,
            payload,
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
        bytes.extend(self.payload.as_bytes());

        bytes // return
    }
}
