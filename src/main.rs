use blockchainlib::*;

fn main() {
    let difficulty = 0x00ffffffffffffffffffffffffffffff; //Adding more zeroes upfront increases difficulty, hence time to mine and nonce value

    let mut block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        "Genesis Block".to_owned(),
        difficulty,
    );

    // // How the hash  works - Block implements Hashable.bytes. that is used in Hashable.hash() to generate the hash for the block
    // let h = block.hash();

    // //Set generated hash as hash of current block
    // block.hash = h;
    // // println!("{:?}", &h);

    // // Genesis block
    // println!("{:?}", &block);

    block.mine();

    // Mined genesis block
    println!("Mined genesis block {:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    // Add more blocks
    for i in 1..=10 {
        let mut block = Block::new(
            i,
            now(),
            last_hash,
            0,
            "Another Block".to_owned(),
            difficulty,
        );

        block.mine();

        println!("Mined genesis block {:?}", &block);

        last_hash =block.hash.clone();

        blockchain.blocks.push(block);
    }
}
