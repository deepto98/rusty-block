use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            // 1. Check if indices match
            if block.index != i as u32 {
                println!("Index mismatch {} != {}", &block.index, &i);
                return false;
            }
            //2. Check difficulty
            else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty Fail");
                return false;
            }
            //3. Check if time increased for non genesis block
            else if i != 0 {
                let prev_block = &self.blocks[i - 1];
                if block.timestamp <= prev_block.timestamp {
                    print!("Time didn't increase");
                    return false;
                }
                //Check hash mismatch
                else if block.prev_block_hash != prev_block.hash {
                    print!("Hash mismatch");
                    return false;
                }
            } else
            // Check if prev hash for genesis is all zeroes
            {
                if block.prev_block_hash != vec![0; 32] {
                    println!("Genesis block's prev_block_hash is invalid");
                    return false;
                }
            }
        }
        true
    }
}
