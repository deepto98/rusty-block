use blockchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Deepto".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Friend".to_owned(),
                    value: 10,
                },
            ],
        }],
        difficulty,
    );

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    // Add new block
    let mut block = Block::new(
        1,
        now(),
        last_hash,
        0,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: "Deepto".to_owned(),
                    value: 100,
                }],
            },
            Transaction {
                inputs: vec![
                    blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Musk".to_owned(),
                        value: 31, 
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 12,
                    },
                ],
            },
            Transaction {
                inputs: vec![
                    blockchain.blocks[0].transactions[0].outputs[1].clone()],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 1, 
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 1,
                    },
                ],
            },
        ],
        difficulty,
    );

    block.mine();

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    blockchain
        .update_with_block(block)
        .expect("Failed to add block");
}
