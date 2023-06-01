use blockchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff; //Adding more zeroes upfront increases difficulty, hence time to mine and nonce value

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
                    to_addr: "Abcd".to_owned(),
                    value: 100,
                },
            ],
        }],
        difficulty,
    );

    // // How the hash  works - Block implements Hashable.bytes. that is used in Hashable.hash() to generate the hash for the block
    // let h = block.hash();

    // //Set generated hash as hash of current block
    // block.hash = h;
    // // println!("{:?}", &h);

    // // Genesis block
    // println!("{:?}", &block);

    genesis_block.mine();

    // Mined genesis block
    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain
        .update_with_block(genesis_block)
        .expect("Falied to add genesis block");
}
