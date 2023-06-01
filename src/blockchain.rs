use std::collections::HashSet;

use super::*;

pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronoligicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}
pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Blockhash>,
}

impl Blockchain {
    // Using update with blocks for txns  instead of verify

    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i: usize = self.blocks.len(); //index of prev block

        // 1. Check if indices match
        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        }
        //2. Check difficulty
        else if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErr::InvalidHash);
        }
        //3. Check if time increased for non genesis block
        else if i != 0 {
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronoligicalTimestamp);
            }
            //Check hash mismatch
            else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        } else
        // Check if prev hash for genesis is all zeroes
        {
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        // Keep track of spent and unspent

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<Blockhash> = HashSet::new();
            let mut block_created: HashSet<Blockhash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                // Check if any inputs arent part of unspent hashes
                if !(&input_hashes - &self.unspent_outputs).is_empty()
                    || (&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }

                // check values
                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;

                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes())
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);

        Ok(())
    }
}
